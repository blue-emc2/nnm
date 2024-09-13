use std::collections::HashMap;
use std::io::stdout;
use std::vec;

use crossterm::style::Print;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
};
use unicode_width::UnicodeWidthStr;

use crate::entity;
use crate::table::{Table, Row};

pub struct Screen {}

impl Screen {
    pub fn new() -> Self {
        Screen {}
    }

    pub fn draw2(&self, entities: &Vec<entity::Entity>, options: HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>>  {
        let (width, height) = crossterm::terminal::size().unwrap_or_else(|_| (80, 24));
        let mut table = Table::new();
        let header = Row::from(vec!["No".to_string(), "Body".to_string()]);
        table
            .set_size(width, height)
            .set_header(header)
            .set_draw_options(options);
        for entity in entities.iter() {
            let title = entity.title.clone().unwrap_or_default();
            let description = entity.description.clone().unwrap_or_default();
            let link = entity.link.clone().unwrap_or_default();

            let row = Row::from(vec![title, description, link]);
            table.add_row(row);
        }

        println!("{}", table);

        Ok(())
    }

    pub fn draw(&self, entities: &Vec<entity::Entity>) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen).unwrap();

        let (width, height) = crossterm::terminal::size().unwrap();
        let mut row: u16 = 1;
        for (_i, entity) in entities.iter().enumerate() {
            let title = entity.title.as_ref().unwrap();
            let description = format!("| {}\r\n", entity.description.as_ref().unwrap());
            let link = entity.link.as_ref().unwrap();

            Self::print_border();
            row += 1;
            if row >= height {
                break;
            }

            let title_width = UnicodeWidthStr::width_cjk(title.as_str());
            let space_count = if width as usize <= title_width {
                0
            } else {
                width as usize - title_width
            };
            let fill_space = " ".repeat(space_count - 4);
            execute!(stdout, Print(format!("| {}{} |\r\n", title, fill_space))).unwrap();
            row += 1;
            if row >= height {
                break;
            }

            let count: u16 = Self::get_line_count(&description, width);
            execute!(stdout, Print(format!("{}", description))).unwrap();
            row += count;
            if row >= height {
                break;
            }

            let link_width = UnicodeWidthStr::width_cjk(link.as_str());
            let space_count = if width as usize <= link_width {
                0
            } else {
                width as usize - link_width
            };
            let fill_space = " ".repeat(space_count - 4);
            execute!(stdout, Print(format!("| {}{} |\r\n", link, fill_space))).unwrap();
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

    fn print_border() {
        let mut stdout = stdout();
        let (width, _height) = crossterm::terminal::size().unwrap();
        let border = "-".repeat(width as usize - 4);
        execute!(stdout, Print(format!("+ {} +", border))).unwrap();
    }
}
