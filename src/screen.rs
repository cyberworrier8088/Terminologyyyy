pub struct Screen {
    pub lines: Vec<String>,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            lines: vec![
                String::from("TTerminology Terminal"),
                String::from(""),
            ],
        }
    }

    pub fn push_line(&mut self, text: impl Into<String>) {
        self.lines.push(text.into());
    }


    pub fn push_char(&mut self, ch: char) {
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }

        self.lines.last_mut().unwrap().push(ch);
    }
}