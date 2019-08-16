use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Style, Modifier, Color};
use tui::symbols::line;
use tui::widgets::Widget;

pub const HEIGHT_LABEL_MARGIN: u16 = 3;

pub struct Item {
    pub position: (u16, u16),
    pub data: String,
}

pub struct SpreadsheetWidget<'a> {
    cell_widths: &'a [u16],
    cell_heights: &'a [u16],
    width_labels: &'a [String],
    height_labels: &'a [String],
    data: &'a [Item],
    top_left: (bool, bool),
}

impl<'a> Default for SpreadsheetWidget<'a> {
    fn default() -> Self {
        Self {
            cell_widths: &[],
            cell_heights: &[],
            width_labels: &[],
            height_labels: &[],
            data: &[],
            top_left: (true, true),
        }
    }
}

impl<'a> SpreadsheetWidget<'a> {
    pub fn new(data: &'a [Item]) -> Self {
        Self {
            data,
            ..SpreadsheetWidget::default()
        }
    }


    pub fn set_cell_widths(
        mut self,
        cell_widths: &'a [u16],
        labels: &'a [String],
    ) -> SpreadsheetWidget<'a> {
        self.cell_widths = cell_widths;
        self.width_labels = labels;
        self
    }

    pub fn set_cell_heights(
        mut self,
        cell_heights: &'a [u16],
        labels: &'a [String],
    ) -> SpreadsheetWidget<'a> {
        self.cell_heights = cell_heights;
        self.height_labels = labels;
        self
    }

    pub fn set_top_left(mut self, top_left: (bool, bool)) -> SpreadsheetWidget<'a> {
        self.top_left = top_left;
        self
    }

    fn draw_headers(&self, area: Rect, buf: &mut Buffer) {
        let mut offset = HEIGHT_LABEL_MARGIN + 1;
        for (width, label) in self.cell_widths.iter().zip(self.width_labels) {
            let x = offset + (width.saturating_sub(label.len() as u16) / 2);
            if x >= area.right() {
                break;
            }

            offset += width + 1;
            buf.set_stringn(x, area.top(), label, *width as usize, Style::default());
        }

        let mut offset = 2;
        for (height, label) in self.cell_heights.iter().zip(self.height_labels) {
            let y = offset + (height / 2);
            if y >= area.bottom() {
                break;
            }

            offset += height + 1;
            buf.set_stringn(
                area.left(),
                y,
                label,
                HEIGHT_LABEL_MARGIN as usize,
                Style::default(),
            );
        }
    }

    fn draw_cells(&self, area: Rect, buf: &mut Buffer) {
        let line_all = "┼";
        // Draw VERTICAL lines
        let mut offset = 0;
        for width in self.cell_widths {
            let x = area.left() + offset;
            if x >= area.right() {
                break;
            }
            offset += width + 1;
            for y in area.top()..area.bottom() {
                buf.get_mut(x, y)
                    .set_symbol(line::VERTICAL)
                    .set_style(Style::default());
            }
        }

        // Draw HORIZONTAL lines
        let mut offset = 0;
        for height in self.cell_heights {
            let y = area.top() + offset;
            if y >= area.bottom() {
                break;
            }
            offset += height + 1;
            for x in area.left()..area.right() {
                buf.get_mut(x, y)
                    .set_symbol(line::HORIZONTAL)
                    .set_style(Style::default());
            }
        }

        // Draw intersections
        let mut x_offset = 0;
        for width in self.cell_widths {
            let mut y_offset = 0;
            for height in self.cell_heights {
                let x = area.left() + x_offset;
                let y = area.top() + y_offset;
                if x >= area.right() || y >= area.bottom() {
                    break;
                }
                buf.get_mut(x, y).set_symbol(line_all);
                y_offset += height + 1;
            }
            x_offset += width + 1;
        }

        // Fix top if needed
        match self.top_left {
            (true, true) => {
                let mut offset = 0;
                for width in self.cell_widths {
                    let x = area.left() + offset;
                    if x >= area.right() {
                        break;
                    }
                    offset += width + 1;
                    buf.get_mut(x, area.top())
                        .set_symbol(line::HORIZONTAL_DOWN)
                        .set_style(Style::default());
                }

                let mut offset = 0;
                for height in self.cell_heights {
                    let y = area.top() + offset;
                    if y >= area.bottom() {
                        break;
                    }
                    offset += height + 1;
                    buf.get_mut(area.left(), y)
                        .set_symbol(line::VERTICAL_RIGHT)
                        .set_style(Style::default());
                }

                buf.get_mut(area.left(), area.top())
                    .set_symbol(line::TOP_LEFT);
            }
            (true, _) => {
                let mut offset = 0;
                for height in self.cell_heights {
                    let y = area.top() + offset;
                    if y >= area.bottom() {
                        break;
                    }
                    offset += height + 1;
                    buf.get_mut(area.left(), y)
                        .set_symbol(line::VERTICAL_RIGHT)
                        .set_style(Style::default());
                }
            }
            (_, true) => {
                let mut offset = 0;
                for width in self.cell_widths {
                    let x = area.left() + offset;
                    if x >= area.right() {
                        break;
                    }
                    offset += width + 1;
                    buf.get_mut(x, area.top())
                        .set_symbol(line::HORIZONTAL_DOWN)
                        .set_style(Style::default());
                }
            }
            _ => {}
        }

        // Draw Data
        for Item {position: (x, y), data: v} in self.data {
            let x1 = x;
            let x = area.left()
                + self.cell_widths[0..*x as usize]
                    .iter()
                    .map(|x| x + 1)
                    .sum::<u16>()
                + 1;
            let y = area.top()
                + self.cell_heights[0..*y as usize]
                    .iter()
                    .map(|y| y + 1)
                    .sum::<u16>()
                + 1;
            buf.set_stringn(x, y,
                            v,
                            self.cell_widths[*x1 as usize] as usize,
                            Style::default()
                )
        }
    }
}

impl<'a> Widget for SpreadsheetWidget<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        self.draw_headers(area, buf);

        let cells_width = area.width - HEIGHT_LABEL_MARGIN;
        let cells_height = area.height - 1;
        let cells_area = Rect::new(
            area.left() + HEIGHT_LABEL_MARGIN,
            area.top() + 1,
            cells_width,
            cells_height,
        );
        self.draw_cells(cells_area, buf);
    }
}
