#[cfg(test)]

use nnm::parser::Parser;

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
        "#.to_string();

        let result = parser.parse(body);
        assert!(result.is_ok());
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
        "#.to_string();

        let result = parser.parse(body);
        assert!(result.is_ok());
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
        "#.to_string();

        let result = parser.parse(body);
        assert!(result.is_ok());
    }
}
