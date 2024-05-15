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

#[derive(Debug, Deserialize)]
pub struct Rdf {
    channel: Channel,
    #[serde(default)]
    item: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct Channel {}

#[derive(Debug, Deserialize)]
struct Item {
    title: String,
    link: String,
    description: String,
}
