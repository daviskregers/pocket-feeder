use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PocketItem {
    pub given_url: String,
    pub given_title: String,
}
