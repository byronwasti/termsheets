use tui::layout::Rect;

use crate::viewer::HEIGHT_LABEL_MARGIN;

// TODO: Composer, not planner
pub struct Planner {
    scroll_offset: (usize, usize),
    cursor_pos: (usize, usize),
    default_width: u16,
    default_height: u16,
    area: Option<Rect>,
}

impl Default for Planner {
    fn default() -> Self {
        Self {
            scroll_offset: (0, 0),
            cursor_pos: (0, 0),
            default_width: 10,
            default_height: 1,
            area: None,
        }
    }
}

impl Planner {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn set_area(&mut self, area: Rect) {
        self.area = Some(area);
    }

    pub fn get_widths(&self) -> Vec<u16> {
        (0..self.get_n_wide()).map(|_| self.default_width).collect()
    }

    pub fn get_heights(&self) -> Vec<u16> {
        (0..self.get_n_high()).map(|_| self.default_height).collect()
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
        return (width_labels, height_labels)
    }

    pub fn get_top_left(&self) -> (usize, usize) {
        self.scroll_offset
    }

    pub fn get_cursor(&self) -> (usize, usize) {
        self.cursor_pos
    }

    pub fn move_cursor(&mut self, (x, y): (i32, i32)) {
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

