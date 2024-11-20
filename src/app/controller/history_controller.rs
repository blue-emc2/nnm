use std::{collections::HashMap, io};

use crate::app::{file::File, history::History, screen};

pub struct HistoryController;

impl HistoryController {
    pub fn show(&self) {
        let screen = screen::Screen::new();
        let history: Result<History, io::Error> = History::new().load_from_file();
        match history {
            Ok(history) => {
                let entities = history.get_entities();
                screen.draw(&entities, HashMap::new());
            }
            Err(e) => {
                eprintln!("Error loading history: {:?}", e);
            }
        }
    }
}
