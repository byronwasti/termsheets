use std::io;
use std::sync::mpsc;
use std::thread;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, Row, Table, Text, Widget};
use tui::Terminal;

mod cells;
mod data;
mod viewer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    let mut cursor_pos = (0, 0);
    let mut scroll_offset = (0, 0);
    let mut data = Vec::new();
    data.push(data::Item {
        x: 2,
        y: 3,
        data: "Test__A".to_owned(),
    });

    loop {
        terminal.draw(|mut f| {
            // Figure out Layout
            let size = f.size();
            let chunks = Layout::default()
                .margin(0)
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(f.size());

            // Calculate offsets + etc.

            // Render Components
            Block::default()
                .borders(Borders::ALL)
                .title(&format!("{:?}", &scroll_offset))
                .render(&mut f, chunks[1]);
            
            viewer::SpreadsheetWidget::new(&[])
                .set_cell_widths(&[5, 10, 10], &["123456", "1234567890", "A"])
                .set_cell_heights(&[1, 3], &["1", "2"])
                .set_top_left((0, 0))
                .render(&mut f, chunks[0]);
            /*
            cells::Spreadsheet::new(&data[..])
                .set_cursor_pos(cursor_pos)
                .set_scroll_offset(&mut scroll_offset)
                .render(&mut f, chunks[0]);
                */
        })?;

        match events.next()? {
            Key::Char('q') => {
                break;
            }
            Key::Down | Key::Char('j') => {
                cursor_pos = (cursor_pos.0, cursor_pos.1 + 1)
            }
            Key::Up | Key::Char('k') => {
                cursor_pos = (cursor_pos.0, cursor_pos.1 - 1)
            }
            Key::Left | Key::Char('h') => {
                cursor_pos = (cursor_pos.0 - 1, cursor_pos.1)
            }
            Key::Right | Key::Char('l') => {
                cursor_pos = (cursor_pos.0 + 1, cursor_pos.1)
            }
            Key::Char('J') => {
                scroll_offset = (scroll_offset.0, scroll_offset.1 + 1)
            }
            Key::Char('K') => {
                scroll_offset = (scroll_offset.0, scroll_offset.1 - 1)
            }
            Key::Char('H') => {
                scroll_offset = (scroll_offset.0-1, scroll_offset.1)
            }
            Key::Char('L') => {
                scroll_offset = (scroll_offset.0+1, scroll_offset.1)
            }
            _ => {}
        };
    }

    Ok(())
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Key>,
}

impl Events {
    pub fn new() -> Events {
        let (tx, rx) = mpsc::channel();
        let tx = tx.clone();
        thread::spawn(move || {
            let stdin = io::stdin();
            for evt in stdin.keys() {
                match evt {
                    Ok(key) => {
                        if let Err(_) = tx.send(key) {
                            return;
                        }
                    }
                    Err(_) => {}
                }
            }
        });

        Events { rx }
    }

    pub fn next(&self) -> Result<Key, mpsc::RecvError> {
        self.rx.recv()
    }
}
