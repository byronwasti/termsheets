use std::fmt::Display;
use std::iter::Iterator;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Style;
use tui::symbols::line;
use tui::widgets::{Block, Widget};

pub struct Item {
    pub x: usize,
    pub y: usize,
    pub data: String,
}

pub struct Spreadsheet<'a> {
    width: u16,
    data: Vec<Item>,
    block: Option<Block<'a>>,
}

impl<'a> Spreadsheet<'a> {
    pub fn new() -> Self {
        let width = 10;
        let mut data = Vec::new();
        data.push(Item {
            x: 2,
            y: 3,
            data: "Test".to_owned(),
        });
        let block = None;
        Self { width, data, block }
    }

    pub fn block(mut self, block: Block<'a>) -> Spreadsheet<'a> {
        self.block = Some(block);
        self
    }

    pub fn draw_headers(&self, area: Rect, buf: &mut Buffer) {
        let width = area.right() - area.left();
        let height = area.bottom() - area.top();
        for y in 0..(height / 2) {
            let y = y + 1;
            buf.set_stringn(
                area.left(),
                area.top() + y * 2,
                format!("{}", y),
                3,
                Style::default(),
            );
        }

        for x in 0..(width / self.width) {
            let x = x + 1;
            let c = (x as u8 + 64) as char;
            buf.set_stringn(
                area.left() + x * self.width - self.width / 4,
                area.top(),
                format!("{}", c),
                3,
                Style::default(),
            );
        }
    }

    pub fn draw_cells(&self, area: Rect, buf: &mut Buffer) {
        let width = area.right() - area.left();
        let height = area.bottom() - area.top();

        let num_ver = ((height as f64) / 3.).floor() as usize;
        let num_hor = ((width as f64) / (self.width + 2) as f64).floor() as usize;

        let line_all = "┼";
        for x in 0..(width / self.width) {
            for y in 0..height {
                buf.get_mut(area.left() + (x * self.width) as u16, area.top() + y)
                    .set_symbol(line::VERTICAL)
                    .set_style(Style::default());
            }
        }

        for y in 0..(height / 2) {
            for x in 0..width {
                if x % self.width == 0 {
                    buf.get_mut(area.left() + x, area.top() + (y * 2) as u16)
                        .set_symbol(line_all)
                        .set_style(Style::default());
                } else {
                    buf.get_mut(area.left() + x, area.top() + (y * 2) as u16)
                        .set_symbol(line::HORIZONTAL)
                        .set_style(Style::default());
                }
            }
        }

        for x in 0..(width / self.width) {
            buf.get_mut(area.left() + (x * self.width) as u16, area.top())
                .set_symbol(line::HORIZONTAL_DOWN)
                .set_style(Style::default());
        }

        for y in 0..(height / 2) {
            buf.get_mut(area.left(), area.top() + (y * 2) as u16)
                .set_symbol(line::VERTICAL_RIGHT)
                .set_style(Style::default());
        }

        buf.get_mut(area.left(), area.top())
            .set_symbol(line::TOP_LEFT)
            .set_style(Style::default());
    }

    pub fn draw_data(&self, area: Rect, buf: &mut Buffer) {
        for d in &self.data {
            let x = d.x as u16;
            let y = d.y as u16;
            let data = &d.data;

            buf.set_stringn(
                x * (self.width + 2) + 1,
                y * 3,
                data,
                self.width as usize,
                Style::default(),
            );
        }
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

        self.draw_headers(table_area, buf);

        let cells_width = table_area.right() - table_area.left() - 3;
        let cells_height = table_area.bottom() - table_area.top() - 1;
        let cells_area = Rect::new(
            table_area.left() + 3,
            table_area.top() + 1,
            cells_width,
            cells_height,
        );

        self.draw_cells(cells_area, buf);
        self.draw_data(cells_area, buf);
    }
}
