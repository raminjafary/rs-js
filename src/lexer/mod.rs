mod utils;
use utils::{is_new_line, is_number, is_whitespace};

#[derive(Debug)]
pub struct Number {
    pub typee: String,
    pub value: i32,
    pub loc: Location,
}

#[derive(Debug)]
pub struct Location {
    start: Loc,
    end: Loc,
}

#[derive(Debug)]
pub struct Loc {
    line: i32,
    col: i32,
}

#[derive(Debug)]
pub struct Lexer {
    str: String,
    cursor: usize,
    char: String,
    line: i32,
    col: i32,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            str: "".to_string(),
            cursor: 0,
            char: "".to_string(),
            line: 0,
            col: 0,
        }
    }

    pub fn input_stream(&mut self, input: String) -> &String {
        self.str = input;
        self.set_char();
        &self.str
    }

    pub fn position(&self) -> Loc {
        Loc {
            line: self.line,
            col: self.col,
        }
    }

    pub fn set_char(&mut self) {
        if let Some(char) = self.str.chars().nth(self.cursor) {
            self.char = char.to_string();
        }

        if self.cursor == self.str.len() {
            self.char = "".to_string();
        }
    }

    pub fn next(&mut self) -> () {
        self.cursor += 1;
        if is_new_line(&self.char) {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        self.set_char();
    }

    pub fn peek(&self) -> String {
        self.str.chars().last().unwrap().to_string()
    }

    pub fn eof(&self) -> bool {
        self.char.len() == 0 || self.cursor == self.str.len()
    }

    pub fn skip_whitespace(&mut self) -> bool {
        if !is_whitespace(&self.char) {
            return false;
        }

        self.next();

        while is_whitespace(&self.char) {
            self.next();
        }
        true
    }

    pub fn number(&mut self) -> Option<Number> {
        let mut value = "".to_string();
        let start = self.position();

        while let Ok(val) = is_number(&self.char) {
            value += &val.to_string();
            self.next();
        }

        if value.len() >= 1 {
            let end = self.position();

            return Some(Number {
                typee: "number".to_string(),
                value: value.parse::<i32>().unwrap(),
                loc: Location { start, end },
            });
        }

        None
    }

    pub fn next_token(&mut self) -> Option<Number> {
        let _ = &self.skip_whitespace();

        if self.eof() {
            return None;
        }

        self.number()
    }
}
