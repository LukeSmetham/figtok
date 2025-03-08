use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
	pub id: String,
	pub name: String,
	#[serde(alias = "type")]
	pub kind: NodeKind,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub children: Option<Vec<Node>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NodeKind {
	Document,
	Canvas,
	Frame,
	Group,
	Section,
	Vector,
	BooleanOperation,
	Star,
	Line,
	Ellipse,
	RegularPolygon,
	Rectangle,
	Table,
	TableCell,
	Text,
	Slice,
	Component,
	ComponentSet,
	Instance,
	Sticky,
	ShapeWithText,
	Connector,
	WashiTape,
}