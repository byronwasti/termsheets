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

    fn get_n_wide(&self) -> usize {
        if let Some(area) = self.area {
            ((area.width - HEIGHT_LABEL_MARGIN) / self.default_width + 1) as usize
        } else {
            0
        }
    }

    fn get_n_high(&self) -> usize {
        if let Some(area) = self.area {
            ((area.height - 1) / self.default_height + 1) as usize
        } else { 
            0
        }
    }
}

