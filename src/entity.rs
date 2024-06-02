#[derive(Debug)]
pub enum EntityType {
    Rdf,    // RSS 1.0のこと
    Rss,
    Atom,
    Unknown
}

#[derive(Debug)]
pub struct Entity {
    pub entity_type: EntityType,
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
    pub pub_date: Option<String>,
}

impl Entity {
    pub fn new(entity_type: EntityType) -> Self {
        Entity {
            entity_type,
            title: None,
            link: None,
            description: None,
            pub_date: None,
        }
    }

    pub fn set_rds(&mut self, item: &Item) {
        self.title = item.title.clone();
        self.link = item.link.field.clone();
        self.description = item.description.clone();
    }

    pub fn set_rss(&mut self, item: &Item) {
        self.title = item.title.clone();
        self.link = item.link.field.clone();
        self.description = item.description.clone();
        self.pub_date = item.pub_date.clone();
    }

    pub fn set_atom(&mut self, item: &Item) {
        self.title = item.title.clone();
        self.link = item.link.href.clone();
        self.description = item.summary.clone();
        self.pub_date = item.pub_date.clone();
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
