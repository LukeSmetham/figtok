use serde::{Deserialize, Serialize};

use super::document::Document;

#[derive(Serialize, Deserialize)]
pub struct File {
	pub name: String,
	pub thumbnail_url: String,
	pub last_modified: String,
	pub version: String,
	pub document: Document,
}