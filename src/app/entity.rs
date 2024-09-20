#[derive(Debug)]
pub enum EntityType {
    Rdf, // RSS 1.0のこと
    Rss,
    Atom,
    Unknown,
}

#[derive(Debug)]
pub struct Entity {
    pub entity_type: EntityType,
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: Option<String>,
}

impl Entity {
    pub fn new(entity_type: EntityType) -> Self {
        Entity {
            entity_type,
            title: String::new(),
            link: String::new(),
            description: String::new(),
            pub_date: None,
        }
    }

    pub fn set_fields(&mut self, title: String, link: Link, description: String, pub_date: Option<String>) {
        self.title = title;
        self.link = link.field.unwrap_or_else(|| "".to_string());
        self.description = description;
        self.pub_date = pub_date;
    }
}

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Link {
    #[serde(rename = "@href")]
    pub href: Option<String>,
    #[serde(rename = "$value")]
    pub field: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Item {
    pub title: Option<String>,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub link: Link,
    #[serde(rename = "pubDate")]
    pub pub_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Rdf {
    channel: RdfChannel,
    pub item: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct RdfChannel {}

#[derive(Debug, Deserialize)]
pub struct Rss {
    pub channel: RssChannel,
}

#[derive(Debug, Deserialize)]
pub struct RssChannel {
    pub item: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Atom {
    pub entry: Vec<Item>,
}
