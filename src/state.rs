use crate::data::Data;
use termion::event::Key;

pub struct StateInfo {
    pub cursor_pos: (usize, usize),
    pub mode: String,
    pub exit: bool,
    pub buffer: String,
}

pub struct State {
    val: StateVal,
    cursor_pos: (usize, usize),
    buffer: String,
    data_updates: Vec<((usize, usize), String)>,
}

impl State {
    pub fn new() -> Self {
        Self {
            val: StateVal::Normal,
            cursor_pos: (0, 0),
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
                self.move_cursor((0, 1));
            }
            Key::Up | Key::Char('k') => {
                self.move_cursor((0, -1));
            }
            Key::Left | Key::Char('h') => {
                self.move_cursor((-1, 0));
            }
            Key::Right | Key::Char('l') => {
                self.move_cursor((1, 0));
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
                    .push((self.cursor_pos.clone(), self.buffer.clone()));
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

    fn move_cursor(&mut self, (x, y): (i32, i32)) {
        let cur_x = self.cursor_pos.0 as i32;
        let cur_y = self.cursor_pos.1 as i32;
        let mut x = cur_x + x;
        let mut y = cur_y + y;
        if x < 0 {
            x = 0;
        }
        if y < 0 {
            y = 0;
        }

        self.cursor_pos = (x as usize, y as usize);
    }
}

#[derive(PartialEq)]
enum StateVal {
    Normal,
    Insert,
    Exit,
}
