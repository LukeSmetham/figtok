use serde::{Deserialize, Serialize};

use super::node::Node;

#[derive(Serialize, Deserialize)]
pub struct Document {
	pub id: String,
	pub name: String,
	#[serde(alias = "type")]
	pub kind: String,
	pub children: Vec<Node>,
}