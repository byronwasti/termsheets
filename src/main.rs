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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    // Input
    loop {
        terminal.draw(|mut f| {
            let size = f.size();
            let chunks = Layout::default()
                .margin(0)
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(f.size());
            let lower_chunks = Layout::default()
                .margin(0)
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
                .split(chunks[1]);

            Block::default()
                .borders(Borders::ALL)
                .render(&mut f, lower_chunks[1]);

            let text = ["Main", "Settings"]
                .iter()
                .map(|x| Text::styled(x.to_owned(), Style::default()));
            List::new(text)
                .block(Block::default().borders(Borders::ALL))
                .render(&mut f, lower_chunks[0]);

            cells::Spreadsheet::new()
                //.block(Block::default().borders(Borders::ALL))
                .render(&mut f, chunks[0]);
        })?;

        match events.next()? {
            Key::Char('q') => {
                break;
            }
            Key::Down | Key::Char('j') => {}
            Key::Up | Key::Char('k') => {}
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
