pub struct Screen {
    pub lines: Vec<String>,
    pub cursor_x: usize,
    pub cursor_y: usize,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn ensure_cursor_valid(&mut self) {
        while self.lines.len() <= self.cursor_y {
            self.lines.push(String::new());
        }
    }

    pub fn push_char(&mut self, ch: char) {
        self.ensure_cursor_valid();
        let line = &mut self.lines[self.cursor_y];

        let char_count = line.chars().count();
        if self.cursor_x < char_count {
            let mut new_line = String::with_capacity(line.len());
            for (i, c) in line.chars().enumerate() {
                if i == self.cursor_x {
                    new_line.push(ch);
                } else {
                    new_line.push(c);
                }
            }
            *line = new_line;
        } else {
            while line.chars().count() < self.cursor_x {
                line.push(' ');
            }
            line.push(ch);
        }
        self.cursor_x += 1;
    }

    pub fn carriage_return(&mut self) {
        self.cursor_x = 0;
    }

    pub fn new_line(&mut self) {
        self.cursor_y += 1;
        self.cursor_x = 0;
        self.ensure_cursor_valid();
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
        let char_count = line.chars().count();

        match mode {
            0 => {
                // Erase from cursor_x to end of line
                if self.cursor_x < char_count {
                    let new_line: String = line.chars().take(self.cursor_x).collect();
                    *line = new_line;
                }
            }
            1 => {
                // Erase from start of line to cursor_x
                if self.cursor_x < char_count {
                    let new_line: String = line.chars().enumerate().map(|(i, c)| {
                        if i <= self.cursor_x { ' ' } else { c }
                    }).collect();
                    *line = new_line;
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
        self.lines.push(String::new());
        self.cursor_x = 0;
        self.cursor_y = 0;
    }
}