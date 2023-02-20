pub mod helpers;
use helpers::{css_stringify, get_token_reference, REGEX_CALC, REGEX_HB};

use crate::Figtok;
use colors_transform::{Color, Rgb};
use convert_case::{Case, Casing};
use serde_json::json;
use serde_derive::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum ReplaceMethod {
	/// Convert the token into a css var() statement, pointing to a css variable somewhere else in the system.
    CssVariables,
	/// Get the inner token value, this technically recurses until it finds the deepest static value. (i.e. not a handlebar reference to another token)
    StaticValues,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    #[serde(alias = "borderRadius")]
    BorderRadius,
    #[serde(alias = "borderWidth")]
    BorderWidth,
    #[serde(alias = "boxShadow")]
    BoxShadow,
    #[serde(alias = "color")]
    Color,
    #[serde(alias = "composition")]
    Composition,
    #[serde(alias = "dimension")]
    Dimension,
    #[serde(alias = "fontFamilies")]
    FontFamily,
    #[serde(alias = "fontSizes")]
    FontSize,
    #[serde(alias = "fontWeights")]
    FontWeights,
    #[serde(alias = "letterSpacing")]
    LetterSpacing,
    #[serde(alias = "lineHeights")]
    LineHeight,
    #[serde(alias = "opacity")]
    Opacity,
    #[serde(alias = "sizing")]
    Sizing,
    #[serde(alias = "spacing")]
    Spacing,
    #[serde(alias = "typography")]
    Typography,
    #[serde(alias = "other")]
    Other,
}
impl ToString for TokenKind {
    fn to_string(&self) -> String {
        match self {
            TokenKind::BorderRadius => "border-radius",
            TokenKind::BorderWidth => "border-width",
            TokenKind::BoxShadow => "box-shadow",
            TokenKind::Color => "color",
            TokenKind::Composition => "composition",
            TokenKind::Dimension => "dimension",
            TokenKind::FontFamily => "font-family",
            TokenKind::FontSize => "font-size",
            TokenKind::FontWeights => "font-weight",
            TokenKind::LetterSpacing => "letter-spacing",
            TokenKind::LineHeight => "line-height",
            TokenKind::Opacity => "opacity",
            TokenKind::Sizing => "sizing",
            TokenKind::Spacing => "spacing",
            TokenKind::Typography => "typography",
            TokenKind::Other => "other",
        }
        .to_string()
    }
}

/// The Token enum encapsulates our different TokenDefinition variants, allowing us to store
/// them all together a single type (i.e. in a collection) whilst parsing/serializing each one
/// differently where necessary.
/// 
/// The Token enum also has some "getter" functions that alias the shared properties between token types
/// to give us an easy way to access inner values by a ref to an enum Token, and reduce the amount of match
/// statements everywhere.
#[derive(Debug)]
pub enum Token {
    Standard(TokenDefinition<String>),
    Composition(TokenDefinition<serde_json::Value>),
    Shadow(TokenDefinition<ShadowValue>),
}
impl Token {
	/// Get the token name from the underlying TokenDefinition<T>
    pub fn name(&self) -> String {
        match self {
            Token::Standard(t) => t.name.clone(),
            Token::Composition(t) => t.name.clone(),
            Token::Shadow(t) => t.name.clone(),
        }
    }

	/// Get the token id from the underlying TokenDefinition<T>
    pub fn id(&self) -> String {
        match self {
            Token::Standard(t) => t.id.clone(),
            Token::Composition(t) => t.id.clone(),
            Token::Shadow(t) => t.id.clone(),
        }
    }

	/// Get the token value. This method calls the get_value() method of a TokenDefinition<T>, we can impl a different 
	/// get_value for each possible value of T that we want to support, ultimately producing a string containing the value
	/// of the token.
	/// 
	/// This is primarily used to access the value of a token, when we are expanding a token value that references another token.
	/// Because of this, it's only ever called directly for Standard tokens and Shadow tokens. Composition tokens are processed
	/// differently as they are serialized as CSS classes containing multiple properties, as appose to CSS Variables. 
    pub fn value(&self, ctx: &Figtok, replace_method: ReplaceMethod, nested: bool) -> String {
        let mut value = match self {
            Token::Standard(t) => t.get_value(ctx, replace_method, nested),
            Token::Shadow(t) => t.get_value(ctx, replace_method),
			// We never call value() on Composition tokens as it currently stands, instead we access the value directly to process the inner values of the composition token.
			// Composition tokens also can't be referenced by other tokens, which means this arm never runs by the call to get_value finding a ref and recursively calling this fn.
            Token::Composition(_) => todo!(), 
        };

		// We check a regex for a css arithmetic expression and if we have a match,
        // then we wrap the value in calc() so CSS can do the actual calculations for us,
        // and we still keep the references to token variables alive.
        if REGEX_CALC.is_match(&value) {
            value = format!("calc({})", value);
        };

        value
    }

	// Serialize the token to a valid CSS string. 
    pub fn to_css(&self, ctx: &Figtok, replace_method: ReplaceMethod) -> String {
		match self {
			Token::Standard(_) | Token::Shadow(_) => {
				format!(
					"--{}: {};",
					css_stringify(&self.name()),
					self.value(ctx, replace_method, false)
				)
			}
			Token::Composition(t) => {
				let mut class = String::new();

				class.push_str(format!(".{} {{", css_stringify(&t.name)).as_str());

				for (key, value) in t.value.as_object().unwrap() {
					// Here we call get_token_reference directly as the inner values of a composition token are not tokens in their own right, 
					//so don't already exist on ctx - but may still contain references to tokens.
					let token_value = get_token_reference(serde_json::from_value::<String>(value.to_owned()).unwrap(), ctx, replace_method);
					class.push_str(
					format!(
							"{}: {};", 
							key.replace(".", "-").to_case(Case::Kebab),
							token_value
						).as_str()
					);
				};

				class.push_str("}");

				class
			},
		}
    }

	// Serialize the token to a valid JSON string. 
	pub fn to_json(&self, ctx: &Figtok, replace_method: ReplaceMethod) -> serde_json::Value {
		match &self {
			Token::Standard(_) | Token::Shadow(_) => {
				let token_name = self.name();
				let mut key_parts = token_name.split(".").collect::<Vec<&str>>();
				key_parts.reverse();

				let value = self.value(ctx, replace_method, false);
				
				let mut j = json!(value);
				for key in key_parts {
					j = json!({ key: j })
				};

				j
			}
			Token::Composition(t) => {
				let token_name = self.name();
				let mut key_parts = token_name.split(".").collect::<Vec<&str>>();
				key_parts.reverse();

				let mut properties: HashMap<String, String> = HashMap::new();

				for (property_name, property_value) in t.value.as_object().unwrap() {
					let inner_value = get_token_reference(serde_json::from_value::<String>(property_value.to_owned()).unwrap(), ctx, replace_method);
					properties.insert(property_name.clone(), inner_value);
				}

				let mut j = serde_json::to_value(properties).unwrap();
				for key in key_parts {
					j = json!({ key: j })
				}

				j
			}
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(bound(deserialize = "T: DeserializeOwned"))]
pub struct TokenDefinition<T> {
    /// The value from the original json file for this token. May be a static value, or a reference using handlebars syntax e.g. {color.purple.1}
    pub value: T,
    /// Tells us what kind of token this is. Aliased from "type" field in the original json.
    #[serde(alias = "type")]
    pub kind: TokenKind,
    /// The name field is constructed as the dot-notated selector for the value in the original JSON file. e.g. "color.purple.1"
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub id: String,
}
impl TokenDefinition<String> {
	// Follows references and returns a string value - this is super simple and applies to most tokens other than Composition, Typography and Shadow.
    pub fn get_value(&self, ctx: &Figtok, replace_method: ReplaceMethod, nested: bool) -> String {
        // Check if the original_value contains handlebar syntax with a reference to another token.
        let value = if REGEX_HB.is_match(&self.value) {
			// if so, follow the reference:
			let mut v = get_token_reference(self.value.to_string(), ctx, replace_method);
			
			// If the token is a color ref token that has a handlebar reference wrap it in rgb()
			// we must also insure we aren't nested so that values that are multiple refs deep don't
			// get wrapped n times.
			if self.kind == TokenKind::Color && !self.value.starts_with("rgb") && !nested {
				v = format!("rgb({})", v);
			}
			
			v
        } else {
			// If no reference and we have a color value, convert it to rgb
			if TokenKind::Color == self.kind {
					let rgb = Rgb::from_hex_str(&self.value).unwrap();
					format!(
						"{}, {}, {}",
						rgb.get_red(),
						rgb.get_green(),
						rgb.get_blue()
					)
			} else {
				// If there is no handlebar reference in the value, just return the value as is.
				self.value.clone()
			}
        };

        value
    }
}
impl TokenDefinition<ShadowValue> {
	/// Shadow values can be expressed as a single string. Because of this it can take the Vec<ShadowLayer>
	/// from serializing the JSON, and deref + concatenate it all together into a single css variable. 
    pub fn get_value(&self, ctx: &Figtok, replace_method: ReplaceMethod) -> String {
        let mut value: Vec<String> = vec![];

        for layer in &self.value.0 {
            match layer.kind {
                ShadowLayerKind::DropShadow => value.push(format!(
                    "{}px {}px {}px {}px {}",
                    layer.x, 
					layer.y, 
					layer.blur,
					layer.spread,
					layer.color,
                )),
                ShadowLayerKind::InnerShadow => value.push(format!(
                    "inset {}px {}px {}px {}px {}",
                    layer.x, 
					layer.y, 
					layer.blur,
					layer.spread,
					layer.color,
                )),
            };
        }

        get_token_reference(value.join(", "), ctx, replace_method)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShadowValue(pub Vec<ShadowLayer>);

#[derive(Serialize, Deserialize, Debug)]
pub struct ShadowLayer {
    color: String,
    #[serde(alias = "type")]
    kind: ShadowLayerKind,
    x: String,
    y: String,
    blur: String,
    spread: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ShadowLayerKind {
    #[serde(alias = "innerShadow")]
    InnerShadow,
    #[serde(alias = "dropShadow")]
    DropShadow,
}

pub type TokenSet = Vec<String>;
pub type TokenSets = HashMap<String, TokenSet>;
pub type Tokens = HashMap<String, Token>;
pub type Theme = HashMap<String, String>;
pub type Themes = HashMap<String, Theme>;
