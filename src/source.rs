use serde::{Deserialize, Serialize};
use crate::item::Item;

#[cfg(test)]
mod tests {
    #[test]
    fn source_item_included_author_and_category_none() {}
    fn source_item_included_author_given_but_doesnt_match() {}
    fn source_item_included_categories_given_but_doesnt_match() {}
    fn source_item_included_author_matches() {}
    fn source_item_included_category_matches() {}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    pub src: String,
    pub exclude: Option<ExcludeRules>,
    pub name: String,
}

impl Source {
    pub fn item_included(self: &Self, item: &Item) -> Option<bool> {
        let mut excluded = false;

        match &item.author {
            Some(author) => {
                for item_author in author {
                    if self
                        .exclude
                        .as_ref()?
                        .author
                        .as_ref()?
                        .contains(&item_author)
                    {
                        excluded = true;
                        break;
                    }
                }
            },
            None => {},
        }

        for item_category in &item.category.clone()? {
            if self
                .exclude
                .as_ref()?
                .category
                .as_ref()?
                .contains(&item_category)
            {
                excluded = true;
                break;
            }
        }

        Some(!excluded)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExcludeRules {
    category: Option<Vec<String>>,
    author: Option<Vec<String>>,
}
