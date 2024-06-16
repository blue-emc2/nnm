use std::io::stdout;

use crossterm::style::Print;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
};

use crate::entity;

pub struct Screen {}

impl Screen {
    pub fn new() -> Self {
        Screen {}
    }

    pub fn draw(&self, entities: &Vec<entity::Entity>) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))?;

        let (_width, height) = crossterm::terminal::size().unwrap();
        let mut row: u16 = 0;
        for (i, entity) in entities.iter().enumerate() {
            let (new_cursor_x, _new_cursor_y) = cursor::position()?;
            let title = entity.title.as_ref().unwrap();
            let description = format!(
                "Description: \t{}\r\n",
                entity.description.as_ref().unwrap()
            );
            let link = entity.link.as_ref().unwrap();

            execute!(
                stdout,
                Print(format!(
                    "--------- {} -- {} - {} --------------------------\r\n",
                    i, height, _new_cursor_y
                ))
            )
            .unwrap();

            execute!(stdout, Print(format!("Title: \t\t{}\r\n", title))).unwrap();
            execute!(stdout, Print(format!("{}", description))).unwrap();
            execute!(stdout, Print(format!("URL: \t\t{}\r\n", link))).unwrap();
        }

        loop {
            match event::read().unwrap() {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(())
    }
}
