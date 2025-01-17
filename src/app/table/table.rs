use core::fmt;
use std::collections::HashMap;
use unicode_width::UnicodeWidthStr;

use crate::app::table::row::{ContentKey, Row};

pub struct Table {
    width: Option<u16>,
    height: Option<u16>,
    rows: Vec<Row>,
    header: Option<Row>,
    options: HashMap<String, String>,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.lines().collect::<Vec<_>>().join("\n"))
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Table {
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            rows: Vec::new(),
            header: None,
            options: HashMap::new(),
        }
    }

    pub fn set_size(&mut self, width: u16, height: u16) -> &mut Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn set_header(&mut self, header: Row) -> &mut Self {
        self.header = Some(header);
        self
    }

    pub fn set_draw_options(&mut self, options: HashMap<String, String>) -> &mut Self {
        self.options = options;
        self
    }

    pub fn add_row(&mut self, row: Row) -> &mut Self {
        self.rows.push(row);
        self
    }

    fn lines(&self) -> impl Iterator<Item = String> {
        let mut lines: Vec<String> = Vec::new();
        let header = self.header.as_ref();
        let rows = &self.rows;

        // [
        //    No,""
        // ]
        if let Some(header) = header {
            let header_content = header.get_content();
            lines.push(format!("{:>3} | {}", header_content[0], header_content[1]));
        }

        let border = self.draw_border();
        lines.push(border.clone());

        // [
        //   title, description, link -> 0 | title
        //                                 | description
        //                                 | link
        //   title, description, link -> 1 | title
        //                                 | description
        //                                 | link
        // ]
        let row_iter = rows.iter().enumerate().peekable();
        for (row_index, row) in row_iter {
            let contents = row.get_content_key_pair();

            for (key, value) in contents {
                match key {
                    ContentKey::Title => {
                        let formatted_title = format!("{:>3} | {}", row_index, value);
                        lines.push(formatted_title);
                    }
                    _ => {
                        let formatted_text = self.draw_text(&value);
                        lines.push(formatted_text);
                    }
                }
            }

            lines.push(border.clone());
        }

        lines.into_iter()
    }

    fn draw_border(&self) -> String {
        let mut border = String::new();
        let w = self.width.unwrap_or_else(|| 0);
        for _ in 0..w {
            border.push('-');
        }
        border
    }

    /**
     * 引数の文字列が画面端に到達したら改行する
     *
     * | descri\n
     * | ption
     */
    fn draw_text(&self, text: &str) -> String {
        let formatted_text = format!("    | {}", text);
        let mut formatted_tmp_text = String::new();
        let mut line = String::new();
        let w = self.width.unwrap_or_else(|| 0);
        let size = UnicodeWidthStr::width_cjk(text) as u16;
        if size < w {
            return formatted_text.to_string();
        }

        let mut row_count = 0; // 画面を埋め尽くさないよう長くても2行ぐらいにする
        for c in formatted_text.chars() {
            let char_size = UnicodeWidthStr::width_cjk(c.to_string().as_str()) as u16;
            let line_size = UnicodeWidthStr::width_cjk(line.as_str()) as u16;

            if line_size + char_size >= w {
                if row_count >= 1 {
                    break;
                }

                formatted_tmp_text.push_str(&line);
                formatted_tmp_text.push('\n');
                line.clear();
                line.push_str("    | ");
                row_count += 1;
            }

            line.push(c);
        }

        formatted_tmp_text.push_str(&line);

        #[cfg(debug_assertions)]
        {
            formatted_tmp_text.push_str("\n");
            formatted_tmp_text.push_str(&format!("    | width: {}, size: {}", w, size));
        }

        formatted_tmp_text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_new() {
        let table = Table::new();
        assert_eq!(table.width, None);
        assert_eq!(table.height, None);
        assert_eq!(table.rows.len(), 0);
    }
}
