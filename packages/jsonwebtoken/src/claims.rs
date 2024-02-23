use indexmap::IndexMap;
use serde_json::Value;

pub type Claims = IndexMap<String, Value>;
