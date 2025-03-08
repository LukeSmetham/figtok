use serde::{Deserialize, Serialize};

use super::node::Node;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
	#[serde(flatten)]
	pub _node: Node
}