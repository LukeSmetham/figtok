use once_cell::sync::Lazy;

use regex::{Regex};

/// Stores a Regex to find handlebars syntax ( i.e. {variable.property} )
pub static REGEX_HB: Lazy<Regex> = Lazy::new(|| {
	Regex::new(r"\{([x[^\{\}]]*)\}.*?").unwrap()
});

/// Stores a Regex to find valid CSS arithmetic expressions
pub static REGEX_CALC: Lazy<Regex> = Lazy::new(|| {
	Regex::new(r"^( )?(var\(--.*\)|[\d\.]+(%|vh|vw|vmin|vmax|em|rem|px|cm|ex|in|mm|pc|pt|ch|q|deg|rad|grad|turn|s|ms|hz|khz))\s[+\-\*/]\s(\-)?(var\(--.*\)|[\d\.]+(%|vh|vw|vmin|vmax|em|rem|px|cm|ex|in|mm|pc|pt|ch|q|deg|rad|grad|turn|s|ms|hz|khz))( )?$").unwrap()
});