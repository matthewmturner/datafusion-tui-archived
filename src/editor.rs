use std::cmp;
use std::io;

use unicode_width::UnicodeWidthStr;

/// Single line of text in SQL Editor and cursor over it
pub struct Line {
    // text: String,
    text: io::Cursor<String>,
}

impl Default for Line {
    fn default() -> Line {
        Line {
            text: io::Cursor::new(String::new()),
        }
    }
}

/// All lines in SQL Editor
pub struct Input {
    pub lines: Vec<Line>,
    /// Current line in editor
    pub cursor_row: u16,
    /// Current column in editor
    pub cursor_column: u16,
}

impl Default for Input {
    fn default() -> Input {
        Input {
            lines: Vec::<Line>::new(),
            cursor_row: 0,
            cursor_column: 0,
        }
    }
}

impl Input {
    pub fn combine_lines(&self) -> String {
        let text: Vec<&str> = self
            .lines
            .iter()
            // Replace tabs with spaces
            .map(|line| line.text.get_ref().as_str())
            .collect();
        text.join("")
    }

    pub fn append_char(&mut self, c: char) {
        if self.lines.is_empty() {
            let line = Line::default();
            self.lines.push(line)
        }
        // self.lines[self.cursor_row as usize].text.get_mut().push(c);
        match c {
            '\n' => {
                self.lines[self.cursor_row as usize].text.get_mut().push(c);
                let line = Line::default();
                self.lines.push(line);
                self.cursor_row += 1;
                self.cursor_column = 0
            }
            '\t' => {
                self.lines[self.cursor_row as usize]
                    .text
                    .get_mut()
                    .push_str("    ");
                self.cursor_column += 4
            }
            _ => {
                self.lines[self.cursor_row as usize].text.get_mut().push(c);
                self.cursor_column += 1;
            }
        }
    }

    pub fn pop(&mut self) -> Option<char> {
        self.lines[self.cursor_row as usize].text.get_mut().pop()
    }

    pub fn up_row(&mut self) {
        // println!("Up row");
        if self.cursor_row > 0 {
            self.cursor_row = cmp::max(self.cursor_row - 1, 0);
        }
        self.cursor_column = self.lines[self.cursor_row as usize].text.get_ref().width() as u16
    }

    pub fn backspace(&mut self) {
        // TODO: Handle deleting at cursor location
        match self.lines[self.cursor_row as usize]
            .text
            .get_ref()
            .is_empty()
        {
            true => {
                self.up_row();
                // Pop newline character
                self.pop();
            }
            false => {
                let last = self.lines[self.cursor_row as usize].text.get_mut().pop();
                match last {
                    Some('\n') => self.up_row(),
                    Some('\t') => self.cursor_column -= 4,
                    Some(_) => self.cursor_column -= 1,
                    None => {}
                }
            }
        }
    }

    pub fn clear(&mut self) {
        let lines = Vec::<Line>::new();
        self.lines = lines;
        self.cursor_row = 0;
        self.cursor_column = 0;
    }

    pub fn tab(&mut self) {
        self.append_char('\t')
    }
}

/// The entire editor and it's state
pub struct Editor {
    /// Current value of the input box
    pub input: Input,
    /// Flag if SQL statement was terminated with ';'
    pub sql_terminated: bool,
}
impl Default for Editor {
    fn default() -> Editor {
        let mut line_lengths = Vec::new();
        line_lengths.push(0);
        let input = Input::default();
        Editor {
            input,
            sql_terminated: false,
        }
    }
}

impl Editor {
    pub fn get_cursor_row(&self) -> u16 {
        self.input.cursor_row
    }

    pub fn get_cursor_column(&self) -> u16 {
        self.input.cursor_column
    }
}
