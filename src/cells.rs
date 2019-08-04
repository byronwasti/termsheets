use std::fmt::Display;
use std::iter::Iterator;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Style;
use tui::symbols::line;
use tui::widgets::{Block, Widget};

use crate::data::Item;

pub struct Spreadsheet<'a> {
    cell_width: u16,
    cell_height: u16,
    data: &'a [Item],
    scroll_offset: Option<&'a mut (usize, usize)>,
    cursor_pos: (usize, usize),
    block: Option<Block<'a>>,
}

impl<'a> Default for Spreadsheet<'a> {
    fn default() -> Self {
        Self {
            cell_width: 10,
            cell_height: 1,
            data: &[],
            scroll_offset: None,
            cursor_pos: (0, 0),
            block: None,
        }
    }
}

impl<'a> Spreadsheet<'a> {
    pub fn new(data: &'a [Item]) -> Self {
        Self { data, ..Self::default() }
    }

    pub fn block(mut self, block: Block<'a>) -> Spreadsheet<'a> {
        self.block = Some(block);
        self
    }

    pub fn set_cursor_pos(mut self, cursor_pos: (usize, usize)) -> Spreadsheet<'a> {
        self.cursor_pos = cursor_pos;
        self
    }

    pub fn set_scroll_offset(mut self, scroll_offset: &'a mut (usize, usize)) -> Spreadsheet<'a> {
        self.scroll_offset = Some(scroll_offset);
        self
    }

    pub fn deal_with_scroll(&self, area: Rect) {
        let num_horizontal = (area.width - 1) / (self.cell_width + 1);
        let num_vertical = (area.height - 1) / (self.cell_height + 1);
    }

    pub fn draw_headers(&self, area: Rect, buf: &mut Buffer) {
        if let Some(&mut scroll_offset) = self.scroll_offset {
            let num_horizontal = (area.width - 1) / (self.cell_width + 1);
            let num_vertical = (area.height-1) / (self.cell_height + 1);

            for j in 0..num_vertical+1 {
                let y = 2 + area.top() + (j * (self.cell_height+1)) as u16;
                if y >= area.bottom() {
                    continue;
                }
                buf.set_stringn(
                    area.left(),
                    y,
                    format!("{}", j+1 + scroll_offset.1 as u16),
                    3,
                    Style::default()
                );
            }

            for i in 0..num_horizontal+1 {
                let c = ((scroll_offset.0 as u16 + i) as u8 + 65) as char;
                let x = 3 + area.left() + (i * (self.cell_width+1)) as u16;
                let x = x + self.cell_width/2;
                if x >= area.right() {
                    continue;
                }
                buf.set_stringn(
                    x,
                    area.top(),
                    format!("{}", c),
                    3,
                    Style::default()
                );
            }
        }
    }

    pub fn draw_cells(&self, area: Rect, buf: &mut Buffer) {
        if let Some(&mut scroll_offset) = self.scroll_offset {
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

            if scroll_offset.0 == 0 {
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

            if scroll_offset.1 == 0 {
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


            if scroll_offset == (0, 0) {
                buf.get_mut(area.left(), area.top())
                    .set_symbol(line::TOP_LEFT)
                    .set_style(Style::default());
            }
        }
    }

    pub fn draw_data(&self, area: Rect, buf: &mut Buffer) {
        if let Some(&mut scroll_offset) = self.scroll_offset {
            for d in self.data {
                let x = d.x.checked_sub(scroll_offset.0);
                let y = d.y.checked_sub(scroll_offset.1);

                let (x, y) = match (x,y) {
                    (Some(x), Some(y)) => (x as u16, y as u16),
                    _ => {
                        continue;
                    }
                };

                let data = &d.data;

                let cursor_offset = if (x as usize, y as usize) == self.cursor_pos {
                    1
                } else {
                    0
                };

                let x = area.left() + x * (self.cell_width + 1) + 1 + cursor_offset;
                let y = area.top() + y * (self.cell_height + 1) + 1;
                if x >= area.right() || y >= area.bottom() {
                    continue;
                }

                buf.set_stringn(
                    x,
                    y,
                    data,
                    self.cell_width as usize - cursor_offset as usize,
                    Style::default(),
                );
            }

            let x = area.left() + self.cursor_pos.0 as u16 * (self.cell_width+1) + 1;
            let y = area.top() + self.cursor_pos.1 as u16 * (self.cell_height+1) + 1;
            buf.get_mut(x, y)
                .set_symbol(">")
                .set_style(Style::default());
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
