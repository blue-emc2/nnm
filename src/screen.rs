use std::io::stdout;

use crossterm::style::Print;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
};
use unicode_width::UnicodeWidthStr;

use crate::entity;

pub struct Screen {}

impl Screen {
    pub fn new() -> Self {
        Screen {}
    }

    pub fn draw(&self, entities: &Vec<entity::Entity>) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen).unwrap();

        let (width, height) = crossterm::terminal::size().unwrap();
        let mut row: u16 = 1;
        for (i, entity) in entities.iter().enumerate() {
            let title = entity.title.as_ref().unwrap();
            let description = format!(
                "Description: \t{}\r\n",
                entity.description.as_ref().unwrap()
            );
            let link = entity.link.as_ref().unwrap();

            execute!(
                stdout,
                Print(format!(
                    "{},{}--------------------------------------\r\n",
                    i, height
                ))
            )
            .unwrap();
            row += 1;
            if row >= height {
                break;
            }

            execute!(stdout, Print(format!("Title: \t\t{}\r\n", title))).unwrap();
            row += 1;
            if row >= height {
                break;
            }

            let count: u16 = Self::get_line_count(&description, width);
            execute!(stdout, Print(format!("{}, {}", count, description))).unwrap();
            row += count;
            if row >= height {
                break;
            }

            execute!(stdout, Print(format!("URL: \t\t{}, {}\r\n", link, row))).unwrap();
            row += 1;
            if row >= height {
                break;
            }
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

        execute!(stdout, LeaveAlternateScreen)?;

        Ok(())
    }

    fn get_line_count(s: &str, width: u16) -> u16 {
        let mut count = 0;

        let line_width = UnicodeWidthStr::width(s.to_string().as_str());
        if line_width < width as usize {
            count += 1;
        } else if line_width >= width as usize {
            count += 2;
        }
        count
    }
}
