use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(row.to_vec());
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.headers.is_empty() && self.body.is_empty() {
            return Ok(());
        }

        // Calculate the width of each column
        let mut column_widths = Vec::new();
        for (i, header) in self.headers.iter().enumerate() {
            let max_body_width = self
                .body
                .iter()
                .map(|row| row.get(i).map_or(0, |cell| cell.len()))
                .max()
                .unwrap_or(0);
            column_widths.push(std::cmp::max(header.len(), max_body_width));
        }

        // Helper function to left-align text
        fn center_header_text(text: &str, width: usize) -> String {
            format!("{:width$}", text, width = width)
        }

        // Helper function to center text
        fn center_text(text: &str, width: usize) -> String {
            let padding = width.saturating_sub(text.len());
            let left_padding = padding / 2;
            let right_padding = padding - left_padding;
            format!(
                "{:left$}{}{:right$}",
                "",
                text,
                "",
                left = left_padding,
                right = right_padding
            )
        }

        // Print the headers
        writeln!(
            f,
            "| {} |",
            self.headers
                .iter()
                .enumerate()
                .map(|(i, h)| center_header_text(h, column_widths[i]))
                .collect::<Vec<String>>()
                .join(" | ")
        )?;

        // Print the separator
        writeln!(
            f,
            "|-{}-|",
            column_widths
                .iter()
                .map(|w| "-".repeat(*w))
                .collect::<Vec<String>>()
                .join("-+-")
        )?;

        // Print the body rows
        for row in &self.body {
            writeln!(
                f,
                "| {} |",
                row.iter()
                    .enumerate()
                    .map(|(i, cell)| center_text(cell, column_widths[i]))
                    .collect::<Vec<String>>()
                    .join(" | ")
            )?;
        }

        Ok(())
    }
}
