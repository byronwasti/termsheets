use tui::layout::Rect;

use crate::viewer::{HEIGHT_LABEL_MARGIN, Item};
use crate::state::StateInfo;
use crate::data::Data;
use log::debug;

pub struct Compositor {
    scroll_offset: (usize, usize),
    cursor_pos: (usize, usize),
    real_cursor_pos: (usize, usize),
    default_width: u16,
    default_height: u16,
    area: Option<Rect>,
    drawable_data: Vec<((usize, usize), String)>,
    buffer: Option<String>,
}

impl Default for Compositor {
    fn default() -> Self {
        Self {
            scroll_offset: (0, 0),
            cursor_pos: (0, 0),
            real_cursor_pos: (0, 0),
            default_width: 10,
            default_height: 1,
            drawable_data: Vec::new(),
            buffer: None,
            area: None,
        }
    }
}

impl Compositor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_area(&mut self, area: Rect) {
        self.area = Some(area);
    }

    pub fn set_state(&mut self, state: StateInfo) {
        self.real_cursor_pos = state.cursor_pos;
        let global_pos_x = state.cursor_pos.0 as i32;
        let global_pos_y = state.cursor_pos.1 as i32;
        let cur_pos_x = (self.cursor_pos.0 + self.scroll_offset.0) as i32;
        let cur_pos_y = (self.cursor_pos.1 + self.scroll_offset.1) as i32;

        let move_x = global_pos_x - cur_pos_x;
        let move_y = global_pos_y - cur_pos_y;
        self.move_cursor((move_x, move_y));

        self.buffer = Some(state.buffer.to_owned());
    }

    pub fn set_data(&mut self, data: &Data) {
        let mut drawable_data = Vec::new();
        for x in (0..self.get_n_wide()).map(|x| x + self.scroll_offset.0) {
            for y in (0..self.get_n_high()).map(|y| y + self.scroll_offset.1) {
                if let Some(d) = data.get((x, y)) {
                    drawable_data.push(((x,y), d.clone()));
                }
            }
        }
        self.drawable_data = drawable_data;
    }

    pub fn get_widths(&self) -> Vec<u16> {
        (0..self.get_n_wide()).map(|_| self.default_width).collect()
    }

    pub fn get_heights(&self) -> Vec<u16> {
        (0..self.get_n_high())
            .map(|_| self.default_height)
            .collect()
    }

    pub fn get_labels(&self) -> (Vec<String>, Vec<String>) {
        let width_labels: Vec<_> = (0..self.get_n_wide())
            .map(|x| x + self.scroll_offset.0)
            .map(|x| (x as u8 + 65) as char)
            .map(|x| format!("{}", x))
            .collect();

        let height_labels: Vec<_> = (0..self.get_n_high())
            .map(|x| x + self.scroll_offset.1)
            .map(|x| format!("{}", x))
            .collect();
        return (width_labels, height_labels);
    }

    pub fn get_top_left(&self) -> (bool, bool) {
        (self.scroll_offset.0 == 0, self.scroll_offset.1 == 0)
    }

    pub fn get_cursor(&self) -> (usize, usize) {
        self.cursor_pos
    }

    pub fn get_drawable(&mut self) -> Vec<Item> {
        let scroll_offset = self.scroll_offset;
        let cursor_pos = self.real_cursor_pos;
        let mut draw_cursor = true;

        let mut items: Vec<Item> = self.drawable_data.drain(..)
            .map(|((x, y), v)| {
                let v = if (x, y) == cursor_pos {
                    draw_cursor = false;
                    let mut tmp = "> ".to_string();
                    tmp.push_str(&v);
                    tmp
                } else { v };
                let x = x as i32;
                let x = x - scroll_offset.0 as i32;
                let y = y as i32;
                let y = y - scroll_offset.1 as i32;
                Item {
                    position: (x as u16, y as u16),
                    data: v,
                }
            })
            .collect();

        if draw_cursor {
            let (x, y) = cursor_pos;
            let x = x as i32;
            let x = x - scroll_offset.0 as i32;
            let y = y as i32;
            let y = y - scroll_offset.1 as i32;
            items.push(Item {
                position: (x as u16, y as u16),
                data: ">".to_string(),
            });
        }

        return items;
    }

    fn move_cursor(&mut self, (x, y): (i32, i32)) {
        let cur_x = self.cursor_pos.0 as i32;
        let cur_y = self.cursor_pos.1 as i32;
        let mut x = cur_x + x;
        let mut y = cur_y + y;
        if x < 0 {
            if self.scroll_offset.0 > 0 {
                self.scroll_offset.0 -= 1;
            }
            x = 0;
        }
        if y < 0 {
            if self.scroll_offset.1 > 0 {
                self.scroll_offset.1 -= 1;
            }
            y = 0;
        }

        let mut x = x as usize;
        let mut y = y as usize;

        let n_wide = self.get_n_wide() - 1;
        let n_high = self.get_n_high() - 1;

        if x >= n_wide {
            self.scroll_offset.0 += 1;
            x = x - 1;
        }

        if y >= n_high {
            self.scroll_offset.1 += 1;
            y = y - 1;
        }

        self.cursor_pos = (x, y);
    }

    fn get_n_wide(&self) -> usize {
        if let Some(area) = self.area {
            ((area.width - HEIGHT_LABEL_MARGIN) / (self.default_width + 1)) as usize
        } else {
            0
        }
    }

    fn get_n_high(&self) -> usize {
        if let Some(area) = self.area {
            ((area.height - 1) / (self.default_height + 1)) as usize
        } else {
            0
        }
    }
}
