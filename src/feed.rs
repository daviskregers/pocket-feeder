use crate::item::Item;
use crate::source::Source;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {

    #[test]
    fn feed_filters_items() {
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feed {
    pub channel: Channel,
}

impl Feed {
    pub fn filtered_items(self: &Self, source: &Source) -> Vec<Item> {
        self.channel
            .item
            .clone()
            .into_iter()
            .filter(|item| source.item_included(item))
            .collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Channel {
    pub item: Vec<Item>,
}
