use std::fmt::Display;
use std::iter::Iterator;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::{Block, Widget};

pub struct Spreadsheet<'a> {
    widths: Vec<u16>,
    data: Vec<Vec<String>>,
    block: Option<Block<'a>>,
}

impl<'a> Spreadsheet<'a> {
    pub fn new() -> Self {
        let widths = vec![10, 10, 10];
        let mut data = Vec::new();
        data.push(vec!["test".to_owned(), "one".to_owned(), "two".to_owned()]);
        data.push(vec!["hello".to_owned(), "world".to_owned(), "YEET".to_owned()]);
        let block = None;
        Self {
            widths,
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
                    x += *w + 1;
                }
            }
    }
}
