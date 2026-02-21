use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Contribution {
    pub authors: Vec<String>,
    pub published_date: String,
    pub licenses: Vec<String>,
}
