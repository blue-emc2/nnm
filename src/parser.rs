use crate::entity::{Entity, EntityType, Item, Rdf, Rss};
use quick_xml::Reader;

pub struct Parser {
    // Add your fields here
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            // Add your fields here
        }
    }

    pub fn parse(&self, body: String) -> Result<Entity, quick_xml::Error> {
        let mut reader = Reader::from_str(&body);
        reader.trim_text(true);

        if body.contains("rdf") {
            let item: Rdf = quick_xml::de::from_str(&body).unwrap();
            println!("{:?}", item);

            let entiry = Entity { entity_type: todo!(), title: todo!(), link: todo!(), description: todo!(), pub_date: todo!() };
            Ok(entiry)
        } else if body.contains("rss") {
            let rss: Rss = quick_xml::de::from_str(&body).unwrap();
            let item: Item = rss.channel.item[0].clone();
            let entiry = Entity { entity_type: EntityType::Rss, title: item.title, link: item.link, description: item.description, pub_date: item.pub_date };
            Ok(entiry)
        } else {
            Err(quick_xml::Error::UnexpectedToken("なんかエラー".to_string()))
        }
    }
}
