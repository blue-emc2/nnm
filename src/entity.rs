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
    pub pub_date: String,
}

use serde::Deserialize;
#[derive(Clone, Debug, Deserialize)]
pub struct Item {
    pub title: String,
    pub description: String,
    pub link: String,
    #[serde(rename = "pubDate")]
    pub pub_date: String,
}

#[derive(Debug, Deserialize)]
pub struct Rdf {
    channel: RdfChannel,
    #[serde(default)]
    item: Vec<Item>,
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
