pub mod helpers;

use std::collections::HashMap;
use convert_case::{Case, Casing};
use serde_derive::{Deserialize, Serialize};
use crate::Figtok;
use regex::Captures;

use self::helpers::{REGEX_HB, REGEX_CALC};

#[derive(Clone, Copy)]
pub enum ReplaceMethod {
	CssVariables,
	StaticValues,
}

fn to_variable_name(name: &String) -> String {
	format!("var(--{})", name.replace(".", "-"))
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
        let mut value = if REGEX_HB.is_match(&self.value) {
            REGEX_HB
                .replace_all(&self.value, |caps: &Captures| {
                    // this will run for each occurrence per string. (i.e. multiple tokens multiplied together)
                    // Get the ref string without the surrounding curly brackets and use it to retrieve the referenced token
                    let ref_name = &caps[1];

                    // Find the token using the ref_name.
                    match ctx.tokens.values().find(|t| {
						let name = match t {
							Token::Standard(t) => t.name.clone(),
							Token::Composition(t) => t.name.clone(),
						};

						name == ref_name
					}) {
                        Some(t) => {

							let token_name = match t {
								Token::Standard(t) => t.name.clone(),
								Token::Composition(t) => t.name.clone(),
							};

                            // If we find a token
                            // Replace the handlebar ref with a css variable that points to the relevant variable for the referenced self.

                            let replacement = match replace_method {
                                ReplaceMethod::CssVariables => to_variable_name(&token_name),
                                ReplaceMethod::StaticValues => {
                                    // when returning a static value, we recursively call get_token_value to ensure we have
                                    // unfurled any tokens that depend on other tokens, and may be indefinitely "nested" in this way.
                                    self.get_value(ctx, replace_method, true)
                                }
                            };

                            let mut value_str = REGEX_HB.replace(&caps[0], replacement).to_string();

                            if !nested {
                                // If we are not nested in this iteration, check the self.kind value and apply any
                                // final transformations. e.g. We convert all colors to rgb/rgba when parsing, so any
                                // color token that doesn't already start with RGB should be wrapped with `rgb()`
                                value_str = match &self.kind {
                                    TokenKind::Color => {
                                        if !self.value.starts_with("rgb") {
                                            value_str = format!("rgb({})", value_str);
                                        }
                                        value_str
                                    }
                                    _ => value_str,
                                }
                            }

                            value_str
                        }
                        None => {
                            let replacement = match replace_method {
                                ReplaceMethod::CssVariables => {
                                    to_variable_name(&String::from(ref_name))
                                }
                                ReplaceMethod::StaticValues => String::from("ERR_NOT_FOUND"),
                            };

                            let mut value_str = REGEX_HB
                                .replace_all(&self.value.to_string(), replacement)
                                .to_string();

                            if !nested
                                && !self.value.starts_with("rgb")
                                && self.kind == TokenKind::Color
                            {
                                value_str = format!("rgb({})", value_str);
                            }
                            value_str
                        }
                    }
                })
                .to_string()
        } else {
            // If there is no handlebar reference in the string, just return the value as is.
            self.value.clone()
        };

        // We check a regex for a css arithmetic expression and if we have a match,
        // then we wrap the value in calc() so CSS can do the actual calculations for us,
        // and we still keep the references to token variables alive.
        if REGEX_CALC.is_match(&value) {
            value = format!("calc({})", value);
        };

        value
    }
}
impl TokenDefinition<serde_json::Value> {
	pub fn get_value(&self, ctx: &Figtok, replace_method: ReplaceMethod, nested: bool) -> String {
		String::from("unimpl.")
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
pub struct ShadowLayer {
    color: String,
    #[serde(alias = "type")]
    kind: String,
    x: String,
    y: String,
    blur: String,
    spread: String,
}

pub enum Token {
	Standard(TokenDefinition<String>),
	Composition(TokenDefinition<serde_json::Value>),
	// Typography(TokenDefinition<TypographyValue>),
	// Shadow(TokenDefinition<Vec<ShadowLayer>>),
}
impl Token {
	pub fn to_css(&self, ctx: &Figtok) -> String {

		let name: String = match self {
			Token::Standard(t) => t.name.clone(),
			Token::Composition(t) => t.name.clone(),
		};
		
		let value: String = match self {
			Token::Standard(t) => t.get_value(ctx, ReplaceMethod::CssVariables, false),
			Token::Composition(t) => t.get_value(ctx, ReplaceMethod::CssVariables, false),
		};

        format!("--{}: {};", name.replace(".", "-").to_case(Case::Kebab), value)
	}
}

pub type TokenSet = Vec<String>;
pub type TokenSets = HashMap<String, TokenSet>;
pub type Tokens = HashMap<String, Token>;
pub type Theme = HashMap<String, String>;
pub type Themes = HashMap<String, Theme>;
