use std::vec;

#[derive(Debug, Clone)]
pub enum ContentKey {
    Title,
    Description,
    Link,
}

#[derive(Debug)]
pub struct Row {
    content: Vec<String>,
}

impl Row {
    pub fn get_content(&self) -> Vec<String> {
        self.content.clone()
    }

    pub fn get_content_key_pair(&self) -> Vec<(ContentKey, String)> {
        let mut key_pairs: Vec<(ContentKey, String)> = Vec::new();
        let keys = vec![ContentKey::Title, ContentKey::Description, ContentKey::Link];

        for (i, key) in keys.iter().enumerate() {
            key_pairs.push((key.clone(), self.content[i].clone()));
        }

        key_pairs
    }
}

impl From<Vec<String>> for Row {
    fn from(v: Vec<String>) -> Self {
        Row { content: v }
    }
}
