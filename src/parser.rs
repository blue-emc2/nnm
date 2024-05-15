use crate::entity::{Entity, Rdf};
use quick_xml::Reader;
use quick_xml::events::Event;

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
        } else {
            Err(quick_xml::Error::UnexpectedToken("なんかエラー".to_string()))
        }
    }
}
