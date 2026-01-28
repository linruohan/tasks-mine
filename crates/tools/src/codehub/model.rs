pub struct CodeHubTool {}
impl CodeHubTool {
    fn new() -> Self {
        Self {}
    }
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeRequest {
    pub id: String,
    pub title: String,
    pub author: String,
    pub created_at: String,
    pub add_lines: i32,
    pub del_lines: i32,
    pub status: String,
}
