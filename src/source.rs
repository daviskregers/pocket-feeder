use serde::{Deserialize, Serialize};
use crate::item::Item;
use crate::feed::Feed;
use serde_xml_rs::from_str;

#[cfg(test)]
mod tests {
    use crate::source::{Source, ExcludeRules};
    use crate::item::Item;

    #[test]
    fn source_item_included_author_and_category_none() {
        let source = Source {
            src: "example.org".to_string(),
            name: "some source".to_string(),
            exclude: None,
        };

        let item = Item {
            author: None,
            category: None,
            description: "...".to_string(),
            link: "...".to_string(),
            pubDate: "...".to_string(),
            timestamp: None,
            title: "...".to_string(),
            utc: None,
        };

        assert!(source.item_included(&item) == true);
    }

    #[test]
    fn source_item_included_author_given_but_doesnt_match() {
        let source = Source {
            src: "example.org".to_string(),
            name: "some source".to_string(),
            exclude: None,
        };

        let item = Item {
            author: Some(vec!["smth".to_string()]),
            category: None,
            description: "...".to_string(),
            link: "...".to_string(),
            pubDate: "...".to_string(),
            timestamp: None,
            title: "...".to_string(),
            utc: None,
        };

        println!("got: {:?}", source.item_included(&item));

        assert!(source.item_included(&item) == true);
    }

    #[test]
    fn source_item_included_categories_given_but_doesnt_match() {
        let source = Source {
            src: "example.org".to_string(),
            name: "some source".to_string(),
            exclude: None,
        };

        let item = Item {
            author: None,
            category: Some(vec!["smth".to_string()]),
            description: "...".to_string(),
            link: "...".to_string(),
            pubDate: "...".to_string(),
            timestamp: None,
            title: "...".to_string(),
            utc: None,
        };

        assert!(source.item_included(&item) == true);
    }

    #[test]
    fn source_item_included_author_matches() {
        let source = Source {
            src: "example.org".to_string(),
            name: "some source".to_string(),
            exclude: Some(ExcludeRules {
                author: None,
                category: Some(vec!["smth".to_string()])
            }),
        };

        let item = Item {
            author: None,
            category: Some(vec!["smth".to_string()]),
            description: "...".to_string(),
            link: "...".to_string(),
            pubDate: "...".to_string(),
            timestamp: None,
            title: "...".to_string(),
            utc: None,
        };

        assert!(source.item_included(&item) == false);
    }

    #[test]
    fn source_item_included_category_matches() {
        let source = Source {
            src: "example.org".to_string(),
            name: "some source".to_string(),
            exclude: Some(ExcludeRules {
                author: Some(vec!["some author".to_string()]),
                category: None,
            }),
        };

        let item = Item {
            author: Some(vec!["some author".to_string()]),
            category: None,
            description: "...".to_string(),
            link: "...".to_string(),
            pubDate: "...".to_string(),
            timestamp: None,
            title: "...".to_string(),
            utc: None,
        };

        assert!(source.item_included(&item) == false);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    pub src: String,
    pub exclude: Option<ExcludeRules>,
    pub name: String,
}

impl Source {
    pub fn item_included(self: &Self, item: &Item) -> bool {
        let mut included = true;

        if let Some(author) = &item.author {
            if let Some(exclude) = &self.exclude {
                if let Some(excl_authors) = &exclude.author {
                    for item_author in author {
                        if excl_authors.contains(&item_author) {
                            included = false;
                            break;
                        }
                    }
                }
            }
        }

        if let Some(category) = &item.category {
            if let Some(exclude) = &self.exclude {
                if let Some(excl_categorys) = &exclude.category {
                    for item_category in category {
                        if excl_categorys.contains(&item_category) {
                            included = false;
                            break;
                        }
                    }
                }
            }
        }

        included
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExcludeRules {
    pub category: Option<Vec<String>>,
    pub author: Option<Vec<String>>,
}

pub fn get_feed(source: &Source) -> Feed {
    let feed: String = reqwest::blocking::get(source.src.as_str())
        .expect(format!("Error getting feed from {}", source.src).as_str())
        .text()
        .expect(format!("Error parsing response from {}", source.src).as_str());

    from_str(feed.as_str())
        .expect(format!("Could not parse the RSS response for {}", source.src).as_str())
}
