use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct History {
    last_fetched_date: String,
}

impl History {
    pub fn new() -> Self {
        History {
            last_fetched_date: "".to_string(),
        }
    }

    // メソッドをここに追加
}
