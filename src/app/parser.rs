use crate::app::entity::{Atom, Entity, EntityType, Rdf, Rss};
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rdf() {
        let parser = Parser::new();
        let body = r#"
            <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
                <channel rdf:about="https://b.hatena.ne.jp/entrylist/it">
                    <title>Example title in channel</title>
                    <link>https://example.com</link>
                    <description>Example description in channel</description>
                    <items>
                        <rdf:Seq>
                            <rdf:li rdf:resource="https://example.com"/>
                        </rdf:Seq>
                    </items>
                </channel>
                <item rdf:about="https://example.com">
                    <title>Example title</title>
                    <link>https://example.com</link>
                    <description>Example description</description>
                </item>
                <item rdf:about="https://example2.com">
                    <title>Example title2</title>
                    <link>https://example.com2</link>
                    <description>Example description2</description>
                </item>
            </rdf:RDF>
        "#
        .to_string();

        let result = parser.parse(body).unwrap();
        assert_eq!(result.len(), 2);

        assert_eq!(
            result.get(0).unwrap().title.as_deref(),
            Some("Example title")
        );
        assert_eq!(
            result.get(0).unwrap().link.as_deref(),
            Some("https://example.com")
        );
        assert_eq!(
            result.get(0).unwrap().description.as_deref(),
            Some("Example description")
        );

        assert_eq!(
            result.get(1).unwrap().title.as_deref(),
            Some("Example title2")
        );
        assert_eq!(
            result.get(1).unwrap().link.as_deref(),
            Some("https://example.com2")
        );
        assert_eq!(
            result.get(1).unwrap().description.as_deref(),
            Some("Example description2")
        );
    }

    #[test]
    fn test_parse_rss() {
        let parser = Parser::new();
        let body = r#"
            <rss version="2.0">
                <channel>
                    <title>Example title</title>
                    <link>https://example.com</link>
                    <description>Example description</description>
                    <item>
                        <title>Example title 1</title>
                        <link>https://example1.com</link>
                        <description>Example description 1</description>
                        <pubDate>Fri, 24 May 2024 18:00:08 +0900</pubDate>
                    </item>
                    <item>
                        <title>Example title 2</title>
                        <link>https://example2.com</link>
                        <description>Example description 2</description>
                        <pubDate>Fri, 24 May 2024 18:00:08 +0900</pubDate>
                    </item>
                </channel>
            </rss>
        "#
        .to_string();

        let result = parser.parse(body).unwrap();
        assert!(result.len() == 2);

        assert_eq!(
            result.get(0).unwrap().title.as_deref(),
            Some("Example title 1")
        );
        assert_eq!(
            result.get(0).unwrap().link.as_deref(),
            Some("https://example1.com")
        );
        assert_eq!(
            result.get(0).unwrap().description.as_deref(),
            Some("Example description 1")
        );

        assert_eq!(
            result.get(1).unwrap().title.as_deref(),
            Some("Example title 2")
        );
        assert_eq!(
            result.get(1).unwrap().link.as_deref(),
            Some("https://example2.com")
        );
        assert_eq!(
            result.get(1).unwrap().description.as_deref(),
            Some("Example description 2")
        );
    }

    #[test]
    fn test_parse_atom() {
        let parser = Parser::new();
        let body = r#"
            <feed xmlns="http://www.w3.org/2005/Atom">
                <title>Example title</title>
                <link href="https://example.com"/>
                <updated>2024-05-24T18:00:08+09:00</updated>
                <entry>
                    <title>Example title 1</title>
                    <link href="https://example1.com"/>
                    <summary>Example description 1</summary>
                    <pubDate>2024-05-24T18:00:08+09:00</pubDate>
                </entry>
                <entry>
                    <title>Example title 2</title>
                    <link href="https://example2.com"/>
                    <summary>Example description 2</summary>
                    <pubDate>2024-05-24T18:00:08+09:00</pubDate>
                </entry>
            </feed>
        "#
        .to_string();

        let result = parser.parse(body).unwrap();
        assert!(result.len() == 2);

        assert_eq!(
            result.get(0).unwrap().title.as_deref(),
            Some("Example title 1")
        );
        assert_eq!(
            result.get(0).unwrap().link.as_deref(),
            Some("https://example1.com")
        );
        assert_eq!(
            result.get(0).unwrap().description.as_deref(),
            Some("Example description 1")
        );

        assert_eq!(
            result.get(1).unwrap().title.as_deref(),
            Some("Example title 2")
        );
        assert_eq!(
            result.get(1).unwrap().link.as_deref(),
            Some("https://example2.com")
        );
        assert_eq!(
            result.get(1).unwrap().description.as_deref(),
            Some("Example description 2")
        );
    }
}
