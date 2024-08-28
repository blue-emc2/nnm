use std::vec;

#[derive(Debug)]
pub struct Row {
    content: Vec<String>,
}

impl Row {
    pub fn get_content(&self) -> Vec<String> {
        self.content.clone()
    }

    pub fn get_content_key_pair(&self) -> Vec<(String, String)> {
        let mut key_pairs: Vec<(String, String)> = Vec::new();
        let keys = vec![String::from("title"), String::from("description"), String::from("link")];

        for (i, key) in keys.iter().enumerate() {
            key_pairs.push((key.clone(), self.content[i].clone()));
        }

        key_pairs
    }
}

impl From<Vec<String>> for Row {
    fn from(v: Vec<String>) -> Self {
        Row {
            content: v,
        }
    }
}
