use tui::layout::Rect;

use crate::data::Data;
use crate::position::CellPos;
use crate::state::{StateInfo, StateVal};
use crate::viewer::{Item, HEIGHT_LABEL_MARGIN};
use log::debug;

pub struct Compositor {
    scroll_offset: CellPos,
    cursor_pos: CellPos,
    default_width: u16,
    default_height: u16,
    area: Option<Rect>,
    drawable_data: Vec<(CellPos, String)>,
    state: Option<StateInfo>,
}

impl Default for Compositor {
    fn default() -> Self {
        Self {
            scroll_offset: CellPos::default(),
            cursor_pos: CellPos::default(),
            default_width: 12,
            default_height: 1,
            drawable_data: Vec::new(),
            state: None,
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
        self.cursor_pos = state.cursor_pos;
        self.handle_scrolling();
        self.state = Some(state);
    }

    pub fn set_data(&mut self, data: &Data) {
        let mut drawable_data = Vec::new();
        for x in (0..self.get_n_wide()).map(|x| x + self.scroll_offset.x) {
            for y in (0..self.get_n_high()).map(|y| y + self.scroll_offset.y) {
                let cell_pos = CellPos::new(x, y);
                if let Some(d) = data.get(cell_pos) {
                    drawable_data.push((cell_pos, d.clone()));
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
            .map(|x| x + self.scroll_offset.x)
            .map(|x| (x as u8 + 65) as char)
            .map(|x| format!("{}", x))
            .collect();

        let height_labels: Vec<_> = (0..self.get_n_high())
            .map(|x| x + self.scroll_offset.y)
            .map(|x| format!("{}", x))
            .collect();
        return (width_labels, height_labels);
    }

    pub fn get_top_left(&self) -> (bool, bool) {
        (self.scroll_offset.x == 0, self.scroll_offset.y == 0)
    }

    pub fn get_drawable(&mut self) -> Vec<Item> {
        let scroll_offset = self.scroll_offset;
        let cursor_pos = self.cursor_pos;
        let mut draw_cursor = true;
        let mut drawable_data = self.drawable_data.to_owned();

        let mut items: Vec<Item> = drawable_data
            .drain(..)
            .map(|(pos, val)| {
                if pos == cursor_pos {
                    draw_cursor = false;
                    self.get_drawable_cursor_cell(Some((pos, val)))
                } else {
                    let pos = pos - scroll_offset;
                    Item {
                        position: (pos.x as u16, pos.y as u16),
                        data: val,
                    }
                }
            })
            .collect();

        if draw_cursor {
            let item = self.get_drawable_cursor_cell(None);
            items.push(item);
        }

        return items;
    }

    pub fn get_area_cells(&self) -> Rect {
        if let (Some(state), Some(area)) = (&self.state, self.area) {
            if state.mode == StateVal::Insert {
                Rect::new(area.left(), area.top(), area.width, area.height - 10)
            } else {
                area
            }
        } else {
            Rect::new(0, 0, 0, 0)
        }
    }

    pub fn get_area_edit(&self) -> Option<Rect> {
        if let (Some(state), Some(area)) = (&self.state, self.area) {
            if state.mode == StateVal::Insert {
                Some(Rect::new(area.left(), area.bottom() - 10, area.width, 10))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_buffer(&self) -> &str {
        if let Some(state) = &self.state {
            &state.buffer
        } else {
            &""
        }
    }

    fn get_drawable_cursor_cell(&self, data: Option<(CellPos, String)>) -> Item {
        if let Some(state) = &self.state {
            if state.mode == StateVal::Insert {
                let pos = self.cursor_pos - self.scroll_offset;
                return Item {
                    position: (pos.x as u16, pos.y as u16),
                    data: "> ".to_string() + &state.buffer,
                };
            }
        }

        if let Some((pos, val)) = data {
            let val = "> ".to_string() + &val;
            let pos = pos - self.scroll_offset;
            Item {
                position: (pos.x as u16, pos.y as u16),
                data: val,
            }
        } else {
            let pos = self.cursor_pos - self.scroll_offset;
            Item {
                position: (pos.x as u16, pos.y as u16),
                data: ">".to_string(),
            }
        }
    }

    fn handle_scrolling(&mut self) {
        let cursor_pos = self.cursor_pos;
        let scroll_offset = self.scroll_offset;
        let n_wide = self.get_n_wide() - 1;
        let n_high = self.get_n_high() - 1;

        if cursor_pos.x > scroll_offset.x + n_wide {
            self.scroll_offset.x += cursor_pos.x - (scroll_offset.x + n_wide);
        } else if cursor_pos.x < scroll_offset.x {
            self.scroll_offset.x -= scroll_offset.x - cursor_pos.x
        }

        if cursor_pos.y > scroll_offset.y + n_high {
            self.scroll_offset.y += cursor_pos.y - (scroll_offset.y + n_high);
        } else if cursor_pos.y < scroll_offset.y {
            self.scroll_offset.y -= scroll_offset.y - cursor_pos.y
        }
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
