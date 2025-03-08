use reqwest::{Client as HttpClient, header};
use crate::error::FigmaError;

pub struct FigmaClient {
    http_client: HttpClient,
    api_key: String,
	base_url: String,
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
}
