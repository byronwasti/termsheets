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
    cell_width: u16,
    cell_height: u16,
    data: Vec<Item>,
    scroll_offset: (usize, usize),
    block: Option<Block<'a>>,
}

impl<'a> Default for Spreadsheet<'a> {
    fn default() -> Self {
        Self {
            cell_width: 10,
            cell_height: 1,
            data: Vec::new(),
            scroll_offset: (0, 0),
            block: None,
        }
    }
}

impl<'a> Spreadsheet<'a> {
    pub fn new() -> Self {
        let mut data = Vec::new();
        data.push(Item {
            x: 2,
            y: 3,
            data: "Test__A".to_owned(),
        });
        Self { data, ..Self::default() }
    }

    pub fn block(mut self, block: Block<'a>) -> Spreadsheet<'a> {
        self.block = Some(block);
        self
    }

    pub fn draw_headers(&self, area: Rect, buf: &mut Buffer) {
        for i in 0..(area.height / (self.cell_height + 1)) {
            let y = area.top() + i*(self.cell_height+1) + 1 + self.cell_height/2;
            if y >= area.bottom() {
                continue;
            }
            buf.set_stringn(
                area.left(),
                y,
                format!("{}", i+1),
                3,
                Style::default(),
            );
        }

        for i in 0..(area.width / (self.cell_width + 1)) {
            let c = (i as u8 + 65) as char;
            let x = area.left() + i * (self.cell_width+1) + 3 + self.cell_width/2;
            if x >= area.right() {
                continue;
            }
            buf.set_stringn(
                x,
                area.top(),
                format!("{}", c),
                3,
                Style::default(),
            );
        }
    }

    pub fn draw_cells(&self, area: Rect, buf: &mut Buffer) {
        let line_all = "┼";

        let num_horizontal = (area.width - 1) / (self.cell_width + 1);
        for i in 0..num_horizontal+1 {
            let x = area.left() + (i * (self.cell_width+1)) as u16;
            if x >= area.right() {
                continue;
            }
            for y in area.top()..area.bottom() {
                buf.get_mut(x, y)
                    .set_symbol(line::VERTICAL)
                    .set_style(Style::default());
            }
        }

        let num_vertical = (area.height-1) / (self.cell_height + 1);
        for i in 0..num_vertical+1 {
            let y = area.top() + (i * (self.cell_height+1)) as u16;
            if y >= area.bottom() {
                continue;
            }
            for x in area.left()..area.right() {
                buf.get_mut(x, y)
                    .set_symbol(line::HORIZONTAL)
                    .set_style(Style::default());
            }
        }

        for j in 0..num_vertical+1 {
            let y = area.top() + (j * (self.cell_height+1)) as u16;
            if y >= area.bottom() {
                continue;
            }
            for i in 0..num_horizontal+1 {
                let x = area.left() + (i * (self.cell_width+1)) as u16;
                if x >= area.right() {
                    continue;
                }

                buf.get_mut(x, y)
                    .set_symbol(line_all)
                    .set_style(Style::default());
            }
        }

        if self.scroll_offset.0 == 0 {
            for j in 0..num_vertical+1 {
                let y = area.top() + (j * (self.cell_height+1)) as u16;
                if y >= area.bottom() {
                    continue;
                }
                buf.get_mut(area.left(), y)
                    .set_symbol(line::VERTICAL_RIGHT)
                    .set_style(Style::default());
            }
        }

        if self.scroll_offset.1 == 0 {
            for i in 0..num_horizontal+1 {
                let x = area.left() + (i * (self.cell_width+1)) as u16;
                if x >= area.right() {
                    continue;
                }
                buf.get_mut(x, area.top())
                    .set_symbol(line::HORIZONTAL_DOWN)
                    .set_style(Style::default());
            }
        }


        if self.scroll_offset == (0, 0) {
            buf.get_mut(area.left(), area.top())
                .set_symbol(line::TOP_LEFT)
                .set_style(Style::default());
        }
    }

    pub fn draw_data(&self, area: Rect, buf: &mut Buffer) {
        for d in &self.data {
            let x = d.x as u16;
            let y = d.y as u16;
            let data = &d.data;

            buf.set_stringn(
                area.left() + x * (self.cell_width + 1) + 1,
                area.top() + y * (self.cell_height + 1) + 1,
                data,
                self.cell_width as usize,
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

        let cells_width = table_area.width - 3;
        let cells_height = table_area.height - 1;
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
