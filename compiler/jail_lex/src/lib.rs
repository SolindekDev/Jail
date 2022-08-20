use jail_token::*;

pub struct LexerPosition{
    pub filename: String,
    pub row: i32,
    pub col: i32,
}

impl LexerPosition{
    pub fn get_pretty(&self) -> String {
        return format!("{}:{}:{}", self.filename, self.row, self.row);
    }

    pub fn new(row: i32, col: i32, filename: String) -> Self{
        Self{
            filename,
            row, 
            col
        }
    }
}

pub struct Lexer{
    data: String,
    position: LexerPosition,
    tokens: Vec<Token>,
    index: usize,
    current_char: char,
    next_char: Option<char>,
    is_error: bool,
}

impl Lexer{
    pub fn new(data: String, filename: String) -> Self{
        Self {
            data: data, 
            position: LexerPosition::new(-1, -1, filename),
            tokens: vec![],
            index: 0,
            current_char: '\0',
            next_char: None,
            is_error: false
        }
    }
    
    pub fn start(&self) {
        println!("{}", self.data);
    }

    pub fn get_next_char() -> Option<char> {
        if self.data.len == self.index {
            return None;
        } else {
            return Some(self.data[self.index]);
        }
    }

    pub fn advance(&self, add_index: usize) {
        self.index += add_index;
        self.current_char = self.data[self.index];
        self.next_char = self.get_next_char();

        if self.current_char == '\n' {
            self.position.row += 1;
            self.position.col = 0;
        } else {
            self.position.col += 1;
        }
    }
}

