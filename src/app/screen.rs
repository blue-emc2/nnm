use std::collections::HashMap;
use std::vec;
use crate::app::entity;
use crate::app::table::table::Table;
use crate::app::table::row::Row;

pub struct Screen {}

impl Screen {
    pub fn new() -> Self {
        Screen {}
    }

    pub fn draw(&self, entities: &Vec<entity::Entity>, options: HashMap<String, String>) {
        let (width, height) = crossterm::terminal::size().unwrap_or_else(|_| (80, 24));
        let mut table = Table::new();
        let header = Row::from(vec!["No".to_string(), "Body".to_string()]);
        table
            .set_size(width, height)
            .set_header(header)
            .set_draw_options(options);
        for entity in entities.iter() {
            let title = entity.title.clone();
            let description = entity.description.clone();
            let link = entity.link.clone();

            let row = Row::from(vec![title, description, link]);
            table.add_row(row);
        }

        println!("{}", table);
    }
}
