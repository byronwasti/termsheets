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
mod planner;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    let mut planner = planner::Planner::new();

    loop {
        terminal.draw(|mut f| {
            let size = f.size();
            let chunks = Layout::default()
                .margin(0)
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(f.size());

            Block::default()
                .borders(Borders::ALL)
                .render(&mut f, chunks[1]);
            
            let area = chunks[0];
            planner.set_area(area);
            let widths = planner.get_widths();
            let heights = planner.get_heights();
            let (w_labels, h_labels) = planner.get_labels();

            viewer::SpreadsheetWidget::new(&[])
                .set_cell_widths(&widths, &w_labels)
                .set_cell_heights(&heights, &h_labels)
                .set_top_left((0, 0))
                .render(&mut f, chunks[0]);
        })?;

        match events.next()? {
            Key::Char('q') => {
                break;
            }
            Key::Down | Key::Char('j') => {
                //cursor_pos = (cursor_pos.0, cursor_pos.1 + 1)
            }
            Key::Up | Key::Char('k') => {
                //cursor_pos = (cursor_pos.0, cursor_pos.1 - 1)
            }
            Key::Left | Key::Char('h') => {
                //cursor_pos = (cursor_pos.0 - 1, cursor_pos.1)
            }
            Key::Right | Key::Char('l') => {
                //cursor_pos = (cursor_pos.0 + 1, cursor_pos.1)
            }
            Key::Char('J') => {
                //scroll_offset = (scroll_offset.0, scroll_offset.1 + 1)
            }
            Key::Char('K') => {
                //scroll_offset = (scroll_offset.0, scroll_offset.1 - 1)
            }
            Key::Char('H') => {
                //scroll_offset = (scroll_offset.0 - 1, scroll_offset.1)
            }
            Key::Char('L') => {
                //scroll_offset = (scroll_offset.0 + 1, scroll_offset.1)
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
