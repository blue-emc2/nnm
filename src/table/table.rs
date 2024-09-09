use core::fmt;
use crate::table::Row;

pub struct Table {
    width: Option<u16>,
    height: Option<u16>,
    rows: Vec<Row>,
    header: Option<Row>,
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
                if key == "title" {
                    let formatted_title = format!("{:>3} | {}", row_index, value);
                    lines.push(formatted_title);
                } else {
                    let formatted_text = format!("    | {}", value);
                    lines.push(formatted_text);
                }
            }
        }

        lines.into_iter()
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
