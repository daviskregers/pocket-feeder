use crate::PocketItem;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PocketItemResponse {
    pub status: u8,
    pub complete: u8,
    pub list: HashMap<String, PocketItem>,
}

impl PocketItemResponse {
    pub fn has_link(self: Self, link: String) -> bool {
        for (_k, v) in self.list {
            if v.given_url == link {
                return true;
            }
        }
        return false;
    }
}
