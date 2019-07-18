use std::fmt::Display;
use std::iter::Iterator;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Style;
use tui::symbols::line;
use tui::widgets::{Block, Widget};

pub struct Spreadsheet<'a> {
    width: u16,
    data: Vec<Vec<String>>,
    block: Option<Block<'a>>,
}

impl<'a> Spreadsheet<'a> {
    pub fn new() -> Self {
        let width = 10;
        let mut data = Vec::new();
        data.push(vec!["test".to_owned(), "one".to_owned(), "two".to_owned()]);
        data.push(vec!["hello".to_owned(), "world".to_owned(), "YEET".to_owned()]);
        let block = None;
        Self {
            width,
            data,
            block,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Spreadsheet<'a> {
        self.block = Some(block);
        self
    }
}

impl<'a> Widget for Spreadsheet<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let table_area = match self.block {
            Some(ref mut b) => {
                b.draw(area, buf);
                b.inner(area)
            }
            None => area,
        };

        let height = table_area.top() - table_area.bottom();
        let width = table_area.right() - table_area.left();

        let num_hor = ((height as f64) / 3.).floor() as usize;
        let num_ver = ((width as f64) / (self.width + 2) as f64).floor() as usize;
        for y in 0..num_hor {
            buf.set_stringn(0, 0, format!("{}, {}", num_hor, num_ver), 10, Style::default());
            for x in 0..num_ver {
            }
        }
        /*
        let mut x = 0;


        let mut x = 0;
        let mut widths = Vec::with_capacity(self.widths.len());
        for width in self.widths.iter() {
            if x + width < table_area.width {
                widths.push(*width);
            }
            x += *width;
        }

        let mut y = table_area.top();
        let remaining = (table_area.bottom() - y) as usize;
        for (i, row) in self.data.iter().take(remaining).enumerate() {
            x = table_area.left();
            for (w, elt) in widths.iter().zip(row) {
                buf.set_stringn(x, y+i as u16, format!("{}", elt), *w as usize, Style::default());
                buf.set_stringn(x + *w, y+i as u16, line::VERTICAL, 1, Style::default());
                x += *w + 1;
            }
        }
        */
    }
}
