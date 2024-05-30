#[derive(Debug)]
pub enum EntityType {
    Rss,
    Atom,
}

#[derive(Debug)]
pub struct Entity {
    pub entity_type: EntityType,
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: Option<String>,
}

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Link {
    #[serde(rename = "@href")]
    pub href: Option<String>,
    #[serde(rename = "$value")]
    pub field: Option<String>,
}

impl Link {
    pub fn get_href(&self) -> String {
        self.href.clone().unwrap_or("".to_string())
    }

    pub fn get_field(&self) -> String {
        self.field.clone().unwrap_or("".to_string())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Item {
    pub title: String,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub link: Link,
    #[serde(rename = "pubDate")]
    pub pub_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Rdf {
    channel: RdfChannel,
    #[serde(default)]
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
