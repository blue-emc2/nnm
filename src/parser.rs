use crate::entity::{Atom, Entity, EntityType, Item, Rdf, Rss};
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

    pub fn parse(&self, body: String) -> Result<Vec<Entity>, quick_xml::Error> {
        let mut reader = Reader::from_str(&body);
        reader.trim_text(true);
        let mut buf = Vec::new();

        if body.contains("<rdf") {
            let rdf: Rdf = quick_xml::de::from_str(&body).unwrap();
            rdf.item.iter().for_each(|item| {
                let entiry = Entity {
                    entity_type: EntityType::Rss,
                    title: item.title.clone(),
                    link: item.link.get_field(),
                    description: item.description.clone().unwrap_or("".to_string()),
                    pub_date: None };
                buf.push(entiry);
            });
            Ok(buf)
        } else if body.contains("<rss") {
            let rss: Rss = quick_xml::de::from_str(&body).unwrap();
            rss.channel.item.iter().for_each(|item| {
                let entiry = Entity {
                    entity_type: EntityType::Rss,
                    title: item.title.clone(),
                    link: item.link.get_field(),
                    description: item.description.clone().unwrap_or("".to_string()),
                    pub_date: item.pub_date.clone() };
                buf.push(entiry);
            });

            Ok(buf)
        } else if body.contains("<feed") {
            let atom: Atom = quick_xml::de::from_str(&body).unwrap();
            atom.entry.iter().for_each(|item| {
                let entiry = Entity {
                    entity_type: EntityType::Atom,
                    title: item.title.clone(),
                    link: item.link.get_href(),
                    description: item.summary.clone().unwrap_or("".to_string()),
                    pub_date: item.pub_date.clone() };
                buf.push(entiry);
            });

            Ok(buf)
        } else {
            Err(quick_xml::Error::UnexpectedToken("なんかエラー".to_string()))
        }
    }
}
