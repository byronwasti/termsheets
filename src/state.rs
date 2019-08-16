use crate::data::Data;
use crate::position::CellPos;
use termion::event::Key;

pub struct StateInfo {
    pub cursor_pos: CellPos,
    pub mode: String,
    pub exit: bool,
    pub buffer: String,
}

pub struct State {
    val: StateVal,
    cursor_pos: CellPos,
    buffer: String,
    data_updates: Vec<(CellPos, String)>,
}

impl State {
    pub fn new() -> Self {
        Self {
            val: StateVal::Normal,
            cursor_pos: CellPos::default(),
            buffer: String::new(),
            data_updates: Vec::new(),
        }
    }

    pub fn get_info(&self) -> StateInfo {
        StateInfo {
            cursor_pos: self.cursor_pos,
            mode: "Normal".to_string(),
            exit: self.val == StateVal::Exit,
            buffer: self.buffer.clone(),
        }
    }

    pub fn update_data(&mut self, data: &mut Data) {
        for d in self.data_updates.drain(..) {
            data.insert(d.0, d.1);
        }
    }

    pub fn handle_event(&mut self, key: Key) {
        match self.val {
            StateVal::Normal => self.handle_event_normal(key),
            StateVal::Insert => self.handle_event_insert(key),
            _ => {}
        }
    }

    fn handle_event_normal(&mut self, key: Key) {
        match key {
            Key::Char('q') => self.val = StateVal::Exit,
            Key::Down | Key::Char('j') => {
                self.move_cursor_down();
            }
            Key::Up | Key::Char('k') => {
                self.move_cursor_up();
            }
            Key::Left | Key::Char('h') => {
                self.move_cursor_left();
            }
            Key::Right | Key::Char('l') => {
                self.move_cursor_right();
            }
            Key::Char('\n') | Key::Char('i') => {
                self.buffer = String::new();
                self.val = StateVal::Insert;
            }
            _ => {}
        }
    }

    fn handle_event_insert(&mut self, key: Key) {
        match key {
            Key::Char('\n') => {
                self.data_updates
                    .push((self.cursor_pos, self.buffer.clone()));
                self.val = StateVal::Normal;
            }
            Key::Char(x) => {
                self.buffer.push(x);
            }
            Key::Backspace => {
                let _ = self.buffer.pop();
            }
            Key::Esc => {
                self.val = StateVal::Normal;
            }
            _ => {}
        }
    }

    fn move_cursor_down(&mut self) {
        self.cursor_pos.y += 1;
    }

    fn move_cursor_up(&mut self) {
        self.cursor_pos.y = self.cursor_pos.y.saturating_sub(1);
    }

    fn move_cursor_left(&mut self) {
        self.cursor_pos.x = self.cursor_pos.x.saturating_sub(1);
    }

    fn move_cursor_right(&mut self) {
        self.cursor_pos.x += 1;
    }
}

#[derive(PartialEq)]
enum StateVal {
    Normal,
    Insert,
    Exit,
}
