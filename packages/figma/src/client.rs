use reqwest::{header, Client as HttpClient};
use crate::error::FigmaError;

use crate::models::file::File;

pub struct FigmaClient {
    http_client: HttpClient,
    pub api_key: String,
	pub base_url: String,
}

impl FigmaClient {
	pub fn new(api_key: String) -> Self {
		let mut headers = header::HeaderMap::new();

		headers.insert("X-Figma-Token", header::HeaderValue::from_str(&api_key).unwrap());

		let http_client = HttpClient::builder()
			.default_headers(headers)
			.build()
			.map_err(FigmaError::RequestError)
			.unwrap();

		Self { 
			http_client, 
			api_key, 
			base_url: "https://api.figma.com/v1".to_string()
		}
	}

	pub async fn file(&self, file_id: &str) -> Result<File, FigmaError> {
		let url = format!("{}/files/{}", self.base_url, file_id);
		
		let response = self.http_client
			.get(&url)
			.send()
			.await
			.map_err(FigmaError::RequestError)?;
		
		if !response.status().is_success() {
			return Err(FigmaError::ApiError(response.status()));
		}
		
		response.json::<File>()
			.await
			.map_err(|e|FigmaError::RequestError(e))
	}
}

