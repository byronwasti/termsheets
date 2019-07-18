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


        let height = table_area.bottom() - table_area.top();
        let width = table_area.right() - table_area.left();

        let num_ver = ((height as f64) / 3.).floor() as usize;
        let num_hor = ((width as f64) / (self.width + 2) as f64).floor() as usize;
        //buf.set_string(table_area.left(), table_area.top(), format!("{}, {}", width, height), Style::default());
        //buf.set_string(table_area.left(), table_area.top()+1, format!("{}, {}", num_hor, num_ver), Style::default());

        let line_all = "┼";
        for x in 0..(width/self.width + 1) {
            for y in 0..height {
                buf.get_mut(
                    table_area.left() + (x*self.width) as u16,
                    table_area.top() + y,
                )
                    .set_symbol(line::VERTICAL)
                    .set_style(Style::default());
            }
        }

        for y in 0..(height/2 + 1) {
            for x in 0..width {
                if x % self.width == 0 {
                    buf.get_mut(
                        table_area.left() + x,
                        table_area.top() + (y*2) as u16,
                    )
                        .set_symbol(line_all)
                        .set_style(Style::default());
                } else {
                    buf.get_mut(
                        table_area.left() + x,
                        table_area.top() + (y*2) as u16,
                    )
                        .set_symbol(line::HORIZONTAL)
                        .set_style(Style::default());
                }
            }
        }

        for x in 0..(width/self.width + 1) {
            buf.get_mut(
                table_area.left() + (x*self.width) as u16,
                table_area.top(),
            )
                .set_symbol(line::HORIZONTAL_DOWN)
                .set_style(Style::default());
        }

        /*
        for y in 0..num_ver {
            for x in 0..num_hor {
                buf.get_mut(
                    table_area.left() + (x as u16 * self.width) as u16,
                    table_area.top() + (y*3) as u16,
                )
                    .set_symbol(line::TOP_LEFT)
                    .set_style(Style::default());
                for z in 1..(self.width+1) {
                    buf.get_mut(
                        table_area.left() + (x as u16 * self.width) as u16 + z,
                        table_area.top() + (y*3) as u16,
                    )
                        .set_symbol(line::HORIZONTAL)
                        .set_style(Style::default());
                }
                buf.get_mut(
                    table_area.left() + (x as u16 * self.width) as u16 + self.width+1,
                    table_area.top() + (y*3) as u16,
                )
                    .set_symbol(line::TOP_RIGHT)
                    .set_style(Style::default());
            }
        }
        */


        /*
        let widths_ = [10, 10, 10];
        let mut x = 0;
        let mut widths = Vec::with_capacity(widths_.len());
        for width in widths_.iter() {
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
