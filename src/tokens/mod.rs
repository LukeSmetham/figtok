pub mod helpers;

use crate::Figtok;
use colors_transform::{Color, Rgb};
use convert_case::{Case, Casing};
use serde_derive::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use self::helpers::{css_stringify, deref_token_value, REGEX_CALC, REGEX_HB};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct ShadowValue(pub Vec<ShadowLayer>);

pub enum Token {
    Standard(TokenDefinition<String>),
    Composition(TokenDefinition<serde_json::Value>),
    Shadow(TokenDefinition<ShadowValue>),
}
impl Token {
    pub fn name(&self) -> String {
        match self {
            Token::Standard(t) => t.name.clone(),
            Token::Composition(t) => t.name.clone(),
            Token::Shadow(t) => t.name.clone(),
        }
    }

    pub fn id(&self) -> String {
        match self {
            Token::Standard(t) => t.id.clone(),
            Token::Composition(t) => t.id.clone(),
            Token::Shadow(t) => t.id.clone(),
        }
    }

    pub fn kind(&self) -> TokenKind {
        match self {
            Token::Standard(t) => t.kind,
            Token::Composition(t) => t.kind,
            Token::Shadow(t) => t.kind,
        }
    }

    pub fn value(&self, ctx: &Figtok, replace_method: ReplaceMethod, nested: bool) -> String {
        let mut value = match self {
            Token::Standard(t) => t.get_value(ctx, replace_method, nested),
            Token::Shadow(t) => t.get_value(ctx, replace_method),
            Token::Composition(t) => t.get_value(ctx, replace_method),
        };

		// We check a regex for a css arithmetic expression and if we have a match,
        // then we wrap the value in calc() so CSS can do the actual calculations for us,
        // and we still keep the references to token variables alive.
        if REGEX_CALC.is_match(&value) {
            value = format!("calc({})", value);
        };

        value
    }

    pub fn to_css(&self, ctx: &Figtok, replace_method: ReplaceMethod) -> String {
		match self {
			Token::Standard(_) | Token::Shadow(_) => {
				format!(
					"--{}: {};",
					css_stringify(&self.name()),
					self.value(ctx, replace_method, false)
				)
			}
			Token::Composition(_) => self.value(ctx, replace_method, false),
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
			let mut v = deref_token_value(self.value.to_string(), ctx, replace_method);
			
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
			}else {
				// If there is no handlebar reference in the value, just return the value as is.
				self.value.clone()
			}
        };

        value
    }
}

impl TokenDefinition<serde_json::Value> {
	/// For composition tokens, it constructs a css class containing all of the inner values so they can be applied at once.
	/// Figtok also treats Typography tokens as composition tokens after the parse step.
	/// 
	/// We can then write these to css separately from the variables.
    pub fn get_value(&self, ctx: &Figtok, replace_method: ReplaceMethod) -> String {
       	let mut class = String::new();

		class.push_str(format!(".{} {{", css_stringify(&self.name)).as_str());

		for (key, value) in self.value.as_object().unwrap() {
			// Here we call deref_token_value directly as the inner values of a composition token are not tokens in their own right, 
			//so don't already exist on ctx - but may still contain references to tokens.
			let token_value = deref_token_value(value.as_str().unwrap().to_string(), ctx, replace_method);
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

        deref_token_value(value.join(", "), ctx, replace_method)
    }
}

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
