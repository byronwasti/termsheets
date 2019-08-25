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
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Terminal;

use log::debug;

mod compositor;
mod data;
mod logger;
mod parse;
mod position;
mod state;
mod viewer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    logger::init();

    let events = Events::new();

    let mut compositor = compositor::Compositor::new();
    let mut data = data::Data::new();
    let mut state = state::State::new();

    loop {
        let state_info = state.get_info();
        if state_info.mode == state::StateVal::Exit {
            break;
        }

        terminal.draw(|mut f| {
            let size = f.size();

            compositor.set_area(size);
            compositor.set_state(state_info);
            compositor.set_data(&data);

            let cell_area = compositor.get_area_cells();
            let widths = compositor.get_widths();
            let heights = compositor.get_heights();
            let (w_labels, h_labels) = compositor.get_labels();
            let top_left = compositor.get_top_left();
            let drawable_data = compositor.get_drawable();

            viewer::SpreadsheetWidget::new(&drawable_data[..])
                .set_cell_widths(&widths, &w_labels)
                .set_cell_heights(&heights, &h_labels)
                .set_top_left(top_left)
                .render(&mut f, cell_area);

            if let Some(edit_area) = compositor.get_area_edit() {
                let buffer = compositor.get_buffer();
                let texts = [Text::raw(buffer)];
                Paragraph::new(texts.iter())
                    .block(Block::default().title("Edit").borders(Borders::ALL))
                    .wrap(true)
                    .render(&mut f, edit_area); 
            }
        })?;

        state.handle_event(events.next()?);
        state.update_data(&mut data);
    }

    Ok(())
}

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
