use figma::client::FigmaClient;
use dotenv::dotenv;
use std::env;
use tokio;

#[test]
fn test_client_initialization() {
    let client = FigmaClient::new("test_api_key".to_string());
    assert_eq!(client.api_key, "test_api_key");
}

#[tokio::test]
async fn test_file() {
	dotenv().ok();
	let api_key = env::var("FIGMA_API_KEY")
		.expect("FIGMA_API_KEY must be set in .env file for integration tests");
	let file_id = env::var("FIGMA_TEST_FILE_ID")
		.expect("FIGMA_TEST_FILE_ID must be set in .env file for integration tests");

	let client = FigmaClient::new(api_key);
	let file = client.file(&file_id).await.unwrap();
	println!("{}", serde_json::to_string_pretty(&file).unwrap());
}
