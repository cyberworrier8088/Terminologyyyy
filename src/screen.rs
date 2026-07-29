// src/screen.rs
// this file is for creating a screen buffer

use glyphon::Color;

#[derive(Clone)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
}

pub struct Screen {
    pub lines: Vec<Vec<Cell>>,
    pub cursor_x: usize,
    pub cursor_y: usize,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            lines: vec![Vec::new()],
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn ensure_cursor_valid(&mut self) {
        while self.lines.len() <= self.cursor_y {
            self.lines.push(Vec::new());
        }
    }

    pub fn push_char_with_fg(&mut self, ch: char, fg: Color) {
        self.ensure_cursor_valid();
        let line = &mut self.lines[self.cursor_y];

        let cell = Cell { ch, fg };

        if self.cursor_x < line.len() {
            line[self.cursor_x] = cell;
        } else {
            while line.len() < self.cursor_x {
                line.push(Cell {
                    ch: ' ',
                    fg: Color::rgb(255, 255, 255),
                });
            }
            line.push(cell);
        }
        self.cursor_x += 1;
    }

    pub fn push_char(&mut self, ch: char) {
        self.push_char_with_fg(ch, Color::rgb(255, 255, 255));
    }

    pub fn carriage_return(&mut self) {
        self.cursor_x = 0;
    }

    const MAX_ROWS: usize = 30;

    pub fn new_line(&mut self) {
        self.cursor_y += 1;
        self.cursor_x = 0;
        

        if self.cursor_y >= Self::MAX_ROWS {
            self.lines.remove(0);
            self.lines.push(Vec::new());
            self.cursor_y = Self::MAX_ROWS - 1;
        } else {
            self.ensure_cursor_valid();
        }
    }

    pub fn cursor_left(&mut self, count: usize) {
        let count = if count == 0 { 1 } else { count };
        self.cursor_x = self.cursor_x.saturating_sub(count);
    }

    pub fn cursor_right(&mut self, count: usize) {
        let count = if count == 0 { 1 } else { count };
        self.cursor_x += count;
    }

    pub fn cursor_up(&mut self, count: usize) {
        let count = if count == 0 { 1 } else { count };
        self.cursor_y = self.cursor_y.saturating_sub(count);
        self.ensure_cursor_valid();
    }

    pub fn cursor_down(&mut self, count: usize) {
        let count = if count == 0 { 1 } else { count };
        self.cursor_y += count;
        self.ensure_cursor_valid();
    }

    pub fn erase_in_line(&mut self, mode: u32) {
        self.ensure_cursor_valid();
        let line = &mut self.lines[self.cursor_y];

        match mode {
            0 => {
                // Erase from cursor_x to end of line
                if self.cursor_x < line.len() {
                    line.truncate(self.cursor_x);
                }
            }
            1 => {
                // Erase from start of line to cursor_x
                if self.cursor_x < line.len() {
                    for cell in &mut line[..=self.cursor_x] {
                        cell.ch = ' ';
                    }
                }
            }
            2 => {
                // Erase entire line
                line.clear();
            }
            _ => {}
        }
    }

    pub fn clear_screen(&mut self) {
        self.lines.clear();
        self.lines.push(Vec::new());
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    pub fn to_text(&self) -> String {
        let mut out = String::new();

        for (y, line) in self.lines.iter().enumerate() {
            for cell in line {
                out.push(cell.ch);
            }

            if y + 1 != self.lines.len() {
                out.push('\n');
            }
        }

        out
    }
}