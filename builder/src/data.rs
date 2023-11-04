use std::collections::HashMap;
use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize, Clone)]
pub struct Theme {
	pub colors: HashMap<String, ColorValue>,
	#[serde(alias = "tokenColors", rename = "tokenColors")]
	pub token_colors: Vec<TokenColor>,
}



#[derive(Serialize, Deserialize, Clone)]
pub struct TokenColor {
	pub name: String,
	pub scope: ScopeValue,
	pub settings: HashMap<String, ColorValue>,
}



#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ScopeValue {
	Single (String),
	Multiple (Vec<String>),
}



#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ColorValue {
	Inline (String),
	Diverge (HashMap<String, String>),
}
