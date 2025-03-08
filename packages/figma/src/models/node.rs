use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Node {
	pub id: String,
}