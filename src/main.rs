use std::io;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Layout, Direction};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Row, Table, Widget, Text, List};
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
                .margin(0)
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(70),
                        Constraint::Percentage(30),
                    ].as_ref()
                )
                .split(f.size());
            let lower_chunks = Layout::default()
                .margin(0)
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(10),

                        Constraint::Percentage(90),
                    ].as_ref()
                )
                .split(chunks[1]);

            Block::default()
                .title("Configs")
                .borders(Borders::ALL)
                .render(&mut f, lower_chunks[1]);

            let text = ["Main", "Settings"].iter().map(|x| {
                Text::styled(x.to_owned(), Style::default())
            });
            List::new(text)
                .block(Block::default().title("Tabs").borders(Borders::ALL))
                .render(&mut f, lower_chunks[0]);

            cells::Spreadsheet::new()
                .render(&mut f, chunks[0]);
        })?;

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
