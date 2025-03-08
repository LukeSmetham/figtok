use serde::{Deserialize, Serialize};

use super::{document::Document, node::Node};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct FileMetadata {
    pub name: String,
    pub last_modified: String,
    pub thumbnail_url: String,
    pub version: String,
    pub role: String,
    pub editor_type: String,
    pub link_access: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct File {
    pub schema_version: u8,
	#[serde(flatten)]
    pub _metadata: FileMetadata,
	pub document: Document, 
	// pub components: IndexMap<String, Component>,
    // pub component_sets: IndexMap<String, ComponentSet>,
    // pub styles: IndexMap<String, Style>,
}