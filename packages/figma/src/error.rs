use thiserror::Error;

#[derive(Error, Debug)]
pub enum FigmaError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
	#[error("API error: {0}")]
	ApiError(reqwest::StatusCode),
	#[error("Invalid data: {0}")]
	DataError(String),
}
