use crate::entity::{Atom, Entity, EntityType, Rdf, Rss};
use quick_xml::{events::Event, name::QName, Reader};

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
        let mut buf = Vec::new();
        let entity_type = self.get_rss_feed_type(&body);

        match entity_type {
            EntityType::Rss => {
                let rss: Rss = quick_xml::de::from_str(&body).unwrap();
                rss.channel.item.iter().for_each(|item| {
                    let mut entity = Entity::new(EntityType::Rss);
                    entity.set_rss(item);
                    buf.push(entity);
                });

                Ok(buf)
            }
            EntityType::Rdf => {
                let rdf: Rdf = quick_xml::de::from_str(&body).unwrap();
                rdf.item.iter().for_each(|item| {
                    let mut entity = Entity::new(EntityType::Rdf);
                    entity.set_rds(item);
                    buf.push(entity);
                });

                Ok(buf)
            }
            EntityType::Atom => {
                let atom: Atom = quick_xml::de::from_str(&body).unwrap();
                atom.entry.iter().for_each(|item| {
                    let mut entity = Entity::new(EntityType::Atom);
                    entity.set_atom(item);
                    buf.push(entity);
                });

                Ok(buf)
            }
            _ => Err(quick_xml::Error::UnexpectedToken(
                "なんかエラー".to_string(),
            )),
        }
    }

    fn get_rss_feed_type(&self, body: &str) -> EntityType {
        let mut reader = Reader::from_str(body);
        reader.trim_text(true);

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    if e.name() == QName(b"rss") {
                        return EntityType::Rss;
                    } else if e.name() == QName(b"rdf:RDF") {
                        return EntityType::Rdf;
                    } else if e.name() == QName(b"feed") {
                        return EntityType::Atom;
                    } else {
                        return EntityType::Unknown;
                    }
                }
                Ok(Event::Eof) => (),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
        }
    }
}
