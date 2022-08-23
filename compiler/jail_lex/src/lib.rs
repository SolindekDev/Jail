use jail_token::*;
use jail_error::*;

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
    last_token: Token,
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
            last_token: Token::new(TokenKind::None, "".to_string(), filename, 0, 0, NumberBase::None),
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
            } else if self.is_whitespace(self.current_char) {
                self.whitespace();
            } else if self.is_identifier_char(self.current_char) {
                self.identifier();
            } else {
                let kind = self.get_kind();
                match kind {
                    TokenKind::None => {
                        print_error(
                            ErrorKind::UnsupportedCharError, 
                            format!("{} ({}) unsupported char", 
                                self.current_char, 
                                self.current_char as u8),
                            true);
                    },
                    _ => {
                        self.tokens.push(Token::new(
                            kind,
                            "symbol".to_string(),
                            self.position.filename.clone(),
                            self.position.row,
                            self.position.col,
                            NumberBase::None
                        ));
                    }
                }
            }     
        }

        self.print_tokens();
    }

    pub fn get_last_token(&self) -> Token {
        if self.tokens.len() == 0 {
            return Token::new(TokenKind::None, "".to_string(), self.position.filename.clone(), 0, 0, NumberBase::None);
        } else {
            match self.tokens.get(self.tokens.len() - 1) {
                Some(token) => {
                    return token.copy();
                }
                None => {
                    return Token::new(TokenKind::None, "".to_string(), self.position.filename.clone(), 0, 0, NumberBase::None);
                }
            }
        }

    }

    pub fn identifier(&mut self) {
        if self.last_token.kind == TokenKind::None {
            self.tokens.push(Token::new(
                TokenKind::Identifier,
                self.current_char.to_string(),
                self.position.filename.clone(),
                self.position.row,
                self.position.col,
                NumberBase::None
            ));
        } else {
            if self.is_space == false {
                if self.last_token.kind == TokenKind::Identifier {
                    match self.tokens.last_mut() {
                        Some(t) => {
                            t.value.push(self.current_char);
                        },
                        None => {}
                    }
                } else {
                    self.tokens.push(Token::new(
                        TokenKind::Identifier,
                        self.current_char.to_string(),
                        self.position.filename.clone(),
                        self.position.row,
                        self.position.col,
                        NumberBase::None
                    ));
                }
            } else {
                self.tokens.push(Token::new(
                    TokenKind::Identifier,
                    self.current_char.to_string(),
                    self.position.filename.clone(),
                    self.position.row,
                    self.position.col,
                    NumberBase::None
                ));
            }
        }
        self.is_space = false;
    }

    // is char a identifier character
    pub fn is_identifier_char(&self, c: char) -> bool {
        return match c.to_lowercase() {
            'a'..'z' | '_' | '@' | '$' => true,
            _ => false,
        }
    }

    // is char a decimal digit
    pub fn is_digit(&self, c: char) -> bool {
        return match c {
            '0'..'9' => true,
            _ => false,
        }
    }

    // is char a hexadecimal digit
    pub fn is_x_digit(&self, c: char) -> bool {
        return match c.to_lowercase() {
            '0'..'9' | 'a'..'f' => true,
            _ => false,
        }
    }

    // is char a octal digit
    pub fn is_octal_digit(&self, c: char) -> bool {
        return match c {
            '0'..'7' => true,
            _ => false,
        }
    }

    // is char a binary digit
    pub fn is_binary_digit(&self, c: char) -> bool {
        return match c {
            '0'..'1' => true,
            _ => false,
        }
    }

    pub fn get_kind(&self) -> TokenKind {
        match self.current_char { 
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '{' => TokenKind::LeftCurlyBrackets,
            '}' => TokenKind::RightCurlyBrackets,
            '+' => {
                match self.next_char {
                    Some(cc) => {
                        let c = self.next_char
                            .unwrap();

                        if c == '=' {
                            return TokenKind::AddAssignment;
                        } else if c == '+' {
                            return TokenKind::Increment;
                        } else if self.is_whitespace(c) {
                            return TokenKind::Plus;
                        } else {
                            print_error(
                                ErrorKind::SyntaxError, 
                                format!("invalid syntax, unexpected use of {} after '+'", 
                                    c), 
                                true
                            );
                            return TokenKind::None;
                        }
                    }
                    None => {
                        return TokenKind::Plus;
                    }
                }
            },
            '-' => {
                match self.next_char {
                    Some(c) => {
                        if c == '=' {
                            return TokenKind::SubstractAssignment;
                        } else if c == '-' {
                            return TokenKind::Decrement;
                        } else if self.is_whitespace(c) {
                            return TokenKind::Minus;
                        } else {
                            print_error(
                                ErrorKind::SyntaxError, 
                                format!("invalid syntax, unexpected use of {} after '-'", 
                                    c), 
                                true
                            );
                            return TokenKind::None;
                        }
                    }
                    None => {
                        return TokenKind::Minus;
                    }
                }
            },
            '*' => {
                match self.next_char {
                    Some(c) => {
                        if c == '=' {
                            return TokenKind::MultiplyAssignment;
                        } else if self.is_whitespace(c) {
                            return TokenKind::Multiply;
                        } else {
                            print_error(
                                ErrorKind::SyntaxError, 
                                format!("invalid syntax, unexpected use of {} after '*'", 
                                    c), 
                                true
                            );
                            return TokenKind::None;
                        }
                    }
                    None => {
                        return TokenKind::Multiply;
                    }
                }
            },
            '/' => {
                match self.next_char {
                    Some(c) => {
                        if c == '=' {
                            return TokenKind::DivideAssignment;
                        } else if self.is_whitespace(c) {
                            return TokenKind::Divide;
                        } else {
                            print_error(
                                ErrorKind::SyntaxError, 
                                format!("invalid syntax, unexpected use of {} after '/'", 
                                    c), 
                                true
                            );
                            return TokenKind::None;
                        }
                    }
                    None => {
                        return TokenKind::Divide;
                    }
                }
            },
            '%' => {
                match self.next_char {
                    Some(c) => {
                        if c == '=' {
                            return TokenKind::ModulusAssignment;
                        } else if self.is_whitespace(c) {
                            return TokenKind::Modulus;
                        } else {
                            print_error(
                                ErrorKind::SyntaxError, 
                                format!("invalid syntax, unexpected use of {} after '%'", 
                                    c), 
                                true
                            );
                            return TokenKind::None;
                        }
                    }
                    None => {
                        return TokenKind::Modulus;
                    }
                }
            },
            '=' => {
                match self.next_char {
                    Some(c) => {
                        if c == '=' {
                            return TokenKind::Equals;
                        } else if self.is_whitespace(c) {
                            return TokenKind::Assignment;
                        } else {
                            print_error(
                                ErrorKind::SyntaxError, 
                                format!("invalid syntax, unexpected use of {} after '='", 
                                    c), 
                                true
                            );
                            return TokenKind::None;
                        }
                    }
                    None => {
                        return TokenKind::Assignment;
                    }
                }
            },
            '>' => {
                match self.next_char {
                    Some(c) => {
                        if c == '=' {
                            return TokenKind::BiggerThanOrEquals;
                        } else if self.is_whitespace(c) {
                            return TokenKind::BiggerThan;
                        } else {
                            print_error(
                                ErrorKind::SyntaxError, 
                                format!("invalid syntax, unexpected use of {} after '>'", 
                                    c), 
                                true
                            );
                            return TokenKind::None;
                        }
                    }
                    None => {
                        return TokenKind::BiggerThan;
                    }
                }
            },
            '<' => {
                match self.next_char {
                    Some(c) => {
                        if c == '=' {
                            return TokenKind::LessThanOrEquals;
                        } else if self.is_whitespace(c) {
                            return TokenKind::LessThan;
                        } else {
                            print_error(
                                ErrorKind::SyntaxError, 
                                format!("invalid syntax, unexpected use of {} after '<'", 
                                    c), 
                                true
                            );
                            return TokenKind::None;
                        }
                    }
                    None => {
                        return TokenKind::LessThan;
                    }
                }
            },
            '|' => TokenKind::Or,
            '&' => TokenKind::And,
            '!' => TokenKind::Bang,
            '?' => TokenKind::QuestionMark,
            ':' => TokenKind::Colon,
            _ => TokenKind::None,
        }
    }

    pub fn whitespace(&mut self) {
        self.is_space = true; 
        self.is_hexadecimal_opened = true;
        self.is_octal_opened = true;
        self.is_binary_opened = true;
    }

    //
    // This function is taken from rust compiler, thanks rust :D
    //
    // https://github.com/rust-lang/rust/blob/master/compiler/rustc_lexer/src/lib.rs#L258-L285
    //
    pub fn is_whitespace(&self, c: char) -> bool {
        return matches!(
            c,
            
            '\u{0009}'   // \t
            | '\u{000A}' // \n
            | '\u{000B}' // vertical tab
            | '\u{000C}' // form feed
            | '\u{000D}' // \r
            | '\u{0020}' // space
    
            // NEXT LINE from latin1
            | '\u{0085}'
    
            // Bidi markers
            | '\u{200E}' // LEFT-TO-RIGHT MARK
            | '\u{200F}' // RIGHT-TO-LEFT MARK
    
            // Dedicated whitespace characters from Unicode
            | '\u{2028}' // LINE SEPARATOR
            | '\u{2029}' // PARAGRAPH SEPARATOR
        );
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
            return Some(self.data.as_bytes()[self.index+1] as char);
        }
    }

    pub fn advance(&mut self, add_index: usize) {
        self.index += add_index;
        self.current_char = self.data.as_bytes()[self.index] as char;
        self.next_char = self.get_next_char();
        self.last_token = self.get_last_token();

        if self.current_char == '\n' {
            self.position.row += 1;
            self.position.col = 0;
        } else {
            self.position.col += 1;
        }
    }
}

