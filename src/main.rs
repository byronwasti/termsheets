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

mod data;
mod viewer;
mod compositor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    let mut compositor = compositor::Compositor::new();

    loop {
        terminal.draw(|mut f| {
            let size = f.size();
            let chunks = Layout::default()
                .margin(0)
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(f.size());

            
            compositor.set_area(chunks[0]);
            let widths = compositor.get_widths();
            let heights = compositor.get_heights();
            let (w_labels, h_labels) = compositor.get_labels();
            let cursor_pos = compositor.get_cursor();
            let top_left = compositor.get_top_left();

            Block::default()
                .borders(Borders::ALL)
                .title(&format!("{}, {}", widths.len(), heights.len()))
                .render(&mut f, chunks[1]);

            viewer::SpreadsheetWidget::new(&[])
                .set_cell_widths(&widths, &w_labels)
                .set_cell_heights(&heights, &h_labels)
                .set_cursor_pos(cursor_pos)
                .set_top_left(top_left)
                .render(&mut f, chunks[0]);
        })?;

        match events.next()? {
            Key::Char('q') => {
                break;
            }
            Key::Down | Key::Char('j') => {
                compositor.move_cursor((0, 1));
            }
            Key::Up | Key::Char('k') => {
                compositor.move_cursor((0, -1));
            }
            Key::Left | Key::Char('h') => {
                compositor.move_cursor(( -1, 0));
            }
            Key::Right | Key::Char('l') => {
                compositor.move_cursor((1, 0));
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
