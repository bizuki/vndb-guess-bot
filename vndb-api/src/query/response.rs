use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VndbQueryResponse<VndbModel> {
    pub results: Vec<VndbModel>,
    pub more: bool,
    pub count: Option<usize>,
    pub compact_filters: Option<String>,
    pub normalized_filters: Option<serde_json::Value>,
}
