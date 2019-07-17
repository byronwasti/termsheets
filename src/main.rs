use std::io;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Layout, Direction};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Row, Table, Widget, Text};
use tui::Terminal;
use std::sync::mpsc;
use std::thread;
use termion::input::TermRead;

mod cells;


struct App<'a> {
    items: Vec<Vec<&'a str>>,
    selected: usize,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            items: vec![
                vec!["Row11", "Row12", "Row13"],
                vec!["Row21", "Row22", "Row23"],
                vec!["Row21", "Row22", "Row23"],
                vec!["Row21", "Row22", "Row23"],
                vec!["Row21", "Row22", "Row23"],
                vec!["Row21", "Row22", "Row23"],
                vec!["Row21", "Row22", "Row23"],
                vec!["Row21", "Row22", "Row23"],
                vec!["Row21", "Row22", "Row23"],
                vec!["Row21", "Row22", "Row23"],
                vec!["Row21", "Row22", "Row23"],
                vec!["Row21", "Row22", "Row23"],
                vec!["Row31", "Row32", "Row33"],
                vec!["Row41", "Row42", "Row43"],
                vec!["Row51", "Row52", "Row53"],
                vec!["Row61", "Row62", "Row63"],
            ],
            selected: 0,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    // App
    let mut app = App::new();

    // Input
    loop {
        terminal.draw(|mut f| {
            let size = f.size();
            let chunks = Layout::default()
                .margin(1)
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(70),
                        Constraint::Percentage(30),
                    ].as_ref()
                )
                .split(f.size());

            cells::Spreadsheet::new()
                .block(Block::default().borders(Borders::ALL))
                .render(&mut f, chunks[0]);
        })?;
        /*
        terminal.draw(|mut f| {
            let selected_style = Style::default().fg(Color::Red).modifier(Modifier::BOLD);
            let normal_style = Style::default().fg(Color::White);
            let header = ["Header1", "Header2", "Header3"];
            let rows = app.items.iter().enumerate().map(|(i, item)| {
                if i == app.selected {
                    Row::StyledData(item.into_iter(), selected_style)
                } else {
                    Row::StyledData(item.into_iter(), normal_style)
                }
            });

            let rects = Layout::default()
                .constraints([Constraint::Percentage(100)].as_ref())
                .margin(0)
                .split(f.size());
            Table::new(header.into_iter(), rows)
                .block(Block::default().borders(Borders::ALL).title("Table"))
                .widths(&[10, 10, 10])
                .render(&mut f, rects[0]);
        })?;
        */

        match events.next()? {
            Key::Char('q') => {
                break;
            }
            Key::Down | Key::Char('j') => {
                app.selected += 1;
                if app.selected > app.items.len() - 1 {
                    app.selected = 0;
                }
            }
            Key::Up | Key::Char('k') => {
                if app.selected > 0 {
                    app.selected -= 1;
                } else {
                    app.selected = app.items.len() - 1;
                }
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

        Events {
            rx,
        }
    }

    pub fn next(&self) -> Result<Key, mpsc::RecvError> {
        self.rx.recv()
    }
}
