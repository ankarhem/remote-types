use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PkgJson<'a> {
    pub name: &'a str,
    version: serde_json::Value,
    description: serde_json::Value,
    repository: serde_json::Value,
    author: serde_json::Value,
    types: serde_json::Value,
    side_effects: serde_json::Value,
    files: serde_json::Value,
    peer_dependencies: serde_json::Value,
}
