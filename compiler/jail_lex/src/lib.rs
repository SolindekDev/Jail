use jail_token::*;

pub struct LexerPosition {
    pub filename: String,
    pub row: i32,
    pub col: i32,
}

impl LexerPosition {
    pub fn get_pretty(&self) -> String {
        return format!("{}:{}:{}", self.filename, self.row, self.col);
    }

    pub fn new(row: i32, col: i32, filename: String) -> Self{
        Self{
            filename,
            row, 
            col
        }
    }
}

pub struct Lexer {
    data: String,
    position: LexerPosition,
    tokens: Vec<Token>,
    index: usize,
    current_char: char,
    next_char: Option<char>,
    is_error: bool,
    is_space: bool,
    is_comment_opened_inline: bool,
    is_comment_opened_multiline: bool,
    is_hexadecimal_opened: bool,
    is_octal_opened: bool,
    is_binary_opened: bool,
    is_string_opened: bool,
}

impl Lexer {
    pub fn new(data: String, filename: String) -> Self{
        Self {
            data: data, 
            position: LexerPosition::new(1, 0, filename),
            tokens: vec![],
            index: 0,
            current_char: '\0',
            next_char: None,
            is_error: false,
            is_space: false,
            is_comment_opened_inline: false,
            is_comment_opened_multiline: false,
            is_hexadecimal_opened: false,
            is_octal_opened: false,
            is_binary_opened: false,
            is_string_opened: false,
        }
    }
    
    pub fn comment_multiline_open(&mut self) {
        if self.is_string_opened == true {
            match self.tokens.last_mut() {
                Some(t) => {
                    t.value.push(self.current_char);
                },
                None => {}
            }
        } else {
            self.is_comment_opened_multiline = true;
        }
    }

    pub fn comment_multiline_close(&mut self) {
        if self.is_string_opened == true {
            match self.tokens.last_mut() {
                Some(t) => {
                    t.value.push(self.current_char);
                },
                None => {}
            }
        } else {
            self.is_comment_opened_multiline = false;
            self.advance(1);
        }
    }

    pub fn comment_inline_open(&mut self) {
        if self.is_string_opened == true {
            match self.tokens.last_mut() {
                Some(t) => {
                    t.value.push(self.current_char);
                },
                None => {}
            }
        } else {
            self.is_comment_opened_inline = false;
        }
    }

    pub fn start(&mut self) {
        for index in 0..self.data.len() {
            self.index = index;
            self.advance(0);

            if self.current_char == '\0' {
                break;
            } else if self.current_char == '\n' {
                self.newline();
            } else if self.current_char == '/' 
                && self.next_char == Some('*') 
                && self.is_comment_opened_inline == false 
                && self.is_comment_opened_multiline == false {
                self.comment_multiline_open();
            } else if self.current_char == '*' 
                && self.next_char == Some('/') 
                && self.is_comment_opened_inline == false 
                && self.is_comment_opened_multiline == true {
                self.comment_multiline_close();
            } else if self.is_comment_opened_multiline == true {
                continue;
            } else if self.current_char == '/'
                && self.next_char == Some('/')
                && self.is_comment_opened_inline == false {
                self.comment_inline_open();
            } else if self.is_comment_opened_inline == true {
                continue;
            } else if self.current_char == '"' {
                self.string_creator();
            } else if self.is_string_opened == true {
                match self.tokens.last_mut() {
                    Some(t) => {
                        t.value.push(self.current_char);
                    },
                    None => {}
                }
            }
        }

        self.print_tokens();
    }

    pub fn string_creator(&mut self) {
        if self.is_string_opened == true {
            self.is_string_opened = false;
        } else {
            self.tokens.push(Token::new(
                TokenKind::StringLiteral,
                "".to_string(),
                self.position.filename.clone(),
                self.position.row,
                self.position.col,
                NumberBase::None
            ));
            self.is_string_opened = true;
        }
    }

    pub fn print_tokens(&mut self) {
        for token in &self.tokens {
            println!("{}", token.to_string());
        }
    }

    pub fn newline(&mut self) {
        self.is_space = true;
        self.is_comment_opened_inline = false;
        self.is_hexadecimal_opened = false;
        self.is_octal_opened = false;
        self.is_binary_opened = false;

        if self.is_comment_opened_multiline == false {
            // self.tokens.push(Token::new(
            //     TokenKind::Newline,
            //     "newline".to_string(),
            //     self.position.filename.clone(),
            //     self.position.row,
            //     self.position.col,
            //     NumberBase::None
            // ));
        }
    }

    pub fn get_next_char(&mut self) -> Option<char> {
        if self.data.len() == self.index {
            return None;
        } else {
            return Some(self.data.as_bytes()[self.index] as char);
        }
    }

    pub fn advance(&mut self, add_index: usize) {
        self.index += add_index;
        self.current_char = self.data.as_bytes()[self.index] as char;
        self.next_char = self.get_next_char();

        if self.current_char == '\n' {
            self.position.row += 1;
            self.position.col = 0;
        } else {
            self.position.col += 1;
        }
    }
}

