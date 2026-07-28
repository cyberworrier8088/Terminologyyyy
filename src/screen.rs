pub struct Screen {
    pub lines: Vec<String>,

    pub cursor_x: usize,
    pub cursor_y: usize,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            lines: vec![
                String::from("TTerminology Terminal"),
                String::from(""),
            ],

            cursor_x: 0,
            cursor_y: 1,
        }
    }

    pub fn push_line(&mut self, text: impl Into<String>) {
        self.lines.push(text.into());
    }


    pub fn push_char(&mut self, ch: char) {
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }

        let line = self.lines.last_mut().unwrap();
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

    pub fn backspace(&mut self) {
        if let Some(line) = self.lines.last_mut() {
            line.pop();
            if self.cursor_x > 0 {
                self.cursor_x -= 1;
            }
        }
    }

    pub fn new_line(&mut self) {
        self.lines.push(String::new());

        self.cursor_y += 1;

        self.cursor_x = 0;
    }
}