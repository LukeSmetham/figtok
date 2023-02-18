pub mod helpers;

use crate::Figtok;
use convert_case::{Case, Casing};
use regex::Captures;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

use self::helpers::{REGEX_CALC, REGEX_HB};

#[derive(Clone, Copy)]
pub enum ReplaceMethod {
    CssVariables,
    StaticValues,
}

fn to_variable_name(name: String) -> String {
    format!("var(--{})", name.replace(".", "-"))
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

fn deref_token_value(input: String, ctx: &Figtok, replace_method: ReplaceMethod) -> String {
    REGEX_HB
        .replace_all(&input, |caps: &Captures| {
            // Get the reference (dot-notation) from the input string without the surrounding curly brackets and use it to retrieve the referenced value.
            let ref_name = &caps[1];

            // Find the referenced token
            if let Some(t) = ctx.tokens.values().find(|t| t.name() == ref_name) {
                
				// Get the value of the referenced token.
				let replacement = match replace_method {

                    ReplaceMethod::CssVariables => to_variable_name(t.name()),
                    ReplaceMethod::StaticValues => {
                        // when returning a static value, we recursively call get_token_value to ensure we have
                        // unfurled any tokens that depend on other tokens, and may be indefinitely "nested" in this way.
                        t.value(ctx, replace_method, true)
                    }
                };

                REGEX_HB.replace(&caps[0], replacement).to_string()
            } else {
				input.clone()
            }
        })
        .to_string()
}

#[derive(Serialize, Deserialize, Debug)]
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
            // If there is no handlebar reference in the value, just return the value as is.
            self.value.clone()
        };

        value
    }
}
impl TokenDefinition<serde_json::Value> {
    pub fn get_value(&self, ctx: &Figtok, replace_method: ReplaceMethod, nested: bool) -> String {
		println!("{:?}", self.value);
        String::from("composition value")
    }
}
impl TokenDefinition<TypographyValue> {
    pub fn get_value(&self, ctx: &Figtok, replace_method: ReplaceMethod, nested: bool) -> String {
        String::from("typography value")
    }
}
impl TokenDefinition<ShadowValue> {
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
pub struct TypographyValue {
    #[serde(alias = "fontFamily")]
    font_family: Option<String>,
    #[serde(alias = "fontWeight")]
    font_weight: Option<String>,
    #[serde(alias = "lineHeight")]
    line_height: Option<String>,
    #[serde(alias = "fontSize")]
    font_size: Option<String>,
    #[serde(alias = "letterSpacing")]
    letter_spacing: Option<String>,
}
impl std::fmt::Display for TypographyValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"TypographyValue(font-family={}, font-weight={}, font-size={}, line-height={}, letter-spacing={})",
			self.font_family.clone().unwrap_or(String::from("inherit")),
			self.font_weight.clone().unwrap_or(String::from("inherit")),
			self.font_size.clone().unwrap_or(String::from("inherit")),
			self.line_height.clone().unwrap_or(String::from("inherit")),
			self.letter_spacing.clone().unwrap_or(String::from("inherit")),
		)
	}
}
impl IntoIterator for TypographyValue {
    type Item = String;
    type IntoIter = TypographyValueIter;

    fn into_iter(self) -> Self::IntoIter {
        TypographyValueIter {
            value: self,
            index: 0,
        }
    }
}

pub struct TypographyValueIter {
    value: TypographyValue,
    index: usize,
}
impl Iterator for TypographyValueIter {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let result = match self.index {
            0 => self.value.font_size.clone(),
            1 => self.value.line_height.clone(),
            2 => self.value.font_family.clone(),
            _ => None,
        };

        self.index += 1;
        result
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ShadowLayerKind {
    #[serde(alias = "innerShadow")]
    InnerShadow,
    #[serde(alias = "dropShadow")]
    DropShadow,
}
impl std::fmt::Display for ShadowLayerKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ShadowLayerKind::InnerShadow => write!(f, "InnerShadow"),
			ShadowLayerKind::DropShadow => write!(f, "DropShadow"),
		}
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
impl std::fmt::Display for ShadowLayer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"ShadowLayer(color={}, type={}, x={}, y={}, blur={}, spread={})",
			self.color,
			self.kind,
			self.x,
			self.y,
			self.blur,
			self.spread
		)
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShadowValue(pub Vec<ShadowLayer>);

impl std::fmt::Display for ShadowValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let shadow_layers = self
			.0
            .iter()
            .map(|shadow_layer| shadow_layer.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "[{}]", shadow_layers)
    }
}

pub enum Token {
    Standard(TokenDefinition<String>),
    Composition(TokenDefinition<serde_json::Value>),
    Typography(TokenDefinition<TypographyValue>),
    Shadow(TokenDefinition<ShadowValue>),
}
impl Token {
    pub fn name(&self) -> String {
        match self {
            Token::Standard(t) => t.name.clone(),
            Token::Composition(t) => t.name.clone(),
            Token::Typography(t) => t.name.clone(),
            Token::Shadow(t) => t.name.clone(),
        }
    }

    pub fn id(&self) -> String {
        match self {
            Token::Standard(t) => t.id.clone(),
            Token::Composition(t) => t.id.clone(),
            Token::Typography(t) => t.id.clone(),
            Token::Shadow(t) => t.id.clone(),
        }
    }

    pub fn kind(&self) -> TokenKind {
        match self {
            Token::Standard(t) => t.kind,
            Token::Composition(t) => t.kind,
            Token::Typography(t) => t.kind,
            Token::Shadow(t) => t.kind,
        }
    }

    pub fn value(&self, ctx: &Figtok, replace_method: ReplaceMethod, nested: bool) -> String {
        let mut value = match self {
            Token::Standard(t) => t.get_value(ctx, replace_method, nested),
            Token::Composition(t) => t.get_value(ctx, replace_method, nested),
            Token::Typography(t) => t.get_value(ctx, replace_method, nested),
            Token::Shadow(t) => t.get_value(ctx, replace_method),
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
        format!(
            "--{}: {};",
            self.name().replace(".", "-").to_case(Case::Kebab),
            self.value(ctx, replace_method, false)
        )
    }
}

pub type TokenSet = Vec<String>;
pub type TokenSets = HashMap<String, TokenSet>;
pub type Tokens = HashMap<String, Token>;
pub type Theme = HashMap<String, String>;
pub type Themes = HashMap<String, Theme>;
