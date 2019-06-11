use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coauthor {
    pub username: String,
    pub name: String,
    pub email: String,
}
