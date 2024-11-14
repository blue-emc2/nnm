use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    Rdf, // RSS 1.0のこと
    Rss,
    Atom,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub fn set_fields(
        &mut self,
        title: String,
        link: Link,
        description: String,
        pub_date: Option<String>,
    ) {
        self.title = title;
        self.link = link.get_link();
        self.description = description;
        self.pub_date = pub_date;
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Link {
    #[serde(rename = "@href")]
    pub href: Option<String>,
    #[serde(rename = "$value")]
    pub field: Option<String>,
}

impl Link {
    pub fn get_link(&self) -> String {
        self.href
            .clone()
            .unwrap_or_else(|| self.field.clone().unwrap_or_else(|| "".to_string()))
    }
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
    _channel: RdfChannel,
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
