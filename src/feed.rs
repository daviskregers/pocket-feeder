use crate::item::Item;
use crate::source::Source;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    use crate::feed::{Feed, Channel};
    use crate::item::Item;
    use crate::source::{Source, ExcludeRules};

    #[test]
    fn feed_filters_items() {
        let feed = Feed {
            channel: Channel {
                item: vec![
                    Item {
                        author: Some(vec!["some author".to_string()]),
                        category: Some(vec!["some category".to_string()]),
                        description: "Some description".to_string(),
                        link: "some link".to_string(),
                        pubDate: "some date".to_string(),
                        timestamp: None,
                        title: "some title".to_string(),
                        utc: None,
                    },
                    Item {
                        author: Some(vec!["some author".to_string()]),
                        category: Some(vec!["excluded category".to_string()]),
                        description: "Some description".to_string(),
                        link: "some link".to_string(),
                        pubDate: "some date".to_string(),
                        timestamp: None,
                        title: "some title".to_string(),
                        utc: None,
                    },
                    Item {
                        author: Some(vec!["excluded author".to_string()]),
                        category: Some(vec!["some category".to_string()]),
                        description: "Some description".to_string(),
                        link: "some link".to_string(),
                        pubDate: "some date".to_string(),
                        timestamp: None,
                        title: "some title".to_string(),
                        utc: None,
                    },
                ]
            }
        };

        println!("GOT HERE!");
        let source = Source {
            src: "somewhere".to_string(),
            name: "some name".to_string(),
            exclude: Some(ExcludeRules {
                author: Some(vec!["excluded author".to_string()]),
                category: Some(vec!["excluded category".to_string()]),
            })
        };

        let actual = feed.filtered_items(&source);

        println!("got actual: {:?}", actual);

        assert_eq!(vec![feed.channel.item[0].clone()], actual);
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
