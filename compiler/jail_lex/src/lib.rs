use jail_token::*;
use jail_error::*;
use std::process::*;

const IDENTIFIER_CONSTANTS: &str = "abcdefghijklmnopqrstuvwxyABCDEFGHIJKLMNOPQRSTUVXYZ_@$";
const DECIMAL_DIGITS_CONSTANTS: &str = "1234567890";
const HEXADECIMAL_DIGITS_CONSTANTS: &str = "1234567890abcdefABCDEF";
const OCTALS_DIGITS_CONSTANTS: &str = "1234567890abcdefABCDEF";
const BINARY_DIGITS_CONSTANTS: &str = "01";
const SYMBOLS_CONSTANTS: &str = "(){}+-*/%=<>!|&?:";

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
    was_comment_open_in_last_tok: bool,
    is_hexadecimal_opened: bool,
    is_octal_opened: bool,
    is_binary_opened: bool,
    is_string_opened: bool,
}

impl Lexer {
    pub fn new(data: String, filename: String) -> Self{
        Self {
            data: data, 
            position: LexerPosition::new(1, 0, filename.clone()),
            tokens: vec![],
            last_token: Token::new(TokenKind::None, "".to_string(), filename, 0, 0, NumberBase::None),
            index: 0,
            current_char: '\0',
            next_char: None,
            is_error: false,
            is_space: false,
            is_comment_opened_inline: false,
            is_comment_opened_multiline: false,
            was_comment_open_in_last_tok: false,
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
            self.was_comment_open_in_last_tok = true;
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
            self.is_comment_opened_inline = true;
        }
    }

    pub fn start(&mut self) {
        while self.index < self.data.len() {
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
                self.index += 1;
                continue;
            } else if self.is_comment_opened_multiline == true {
                self.index += 1;
                continue;
            } else if self.current_char == '/'
                && self.next_char == Some('/')
                && self.is_comment_opened_inline == false {
                self.comment_inline_open();
            } else if self.is_comment_opened_inline == true {
                self.index += 1;
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
            } else if self.is_symbol(self.current_char) {
                self.symbol();
            } else if self.is_digit(self.current_char) {
                self.number();
            } else if self.current_char == '.' {
                self.float();
            } else {
                print_error(
                    ErrorKind::UnsupportedCharError, 
                    format!("{} ({}) unsupported char", 
                        self.current_char, 
                        self.current_char as u8),
                    true);
            }     

            self.index += 1;
        }

        self.print_tokens();
        self.is_error_exit();
    }

    pub fn is_error_exit(&self) {
        if self.is_error == true {
            exit(0x01);
        }
    }

    pub fn float(&mut self) {
        if self.last_token.kind == TokenKind::None {
            self.tokens.push(Token::new(
                TokenKind::FloatLiteral,
                "0.".to_string(),
                self.position.filename.clone(),
                self.position.row,
                self.position.col,
                NumberBase::Decimal
            ));
            self.is_space = false;
        } else {
            if self.last_token.kind == TokenKind::IntLiteral && self.is_space == false {
                let mut last_token = self.tokens.pop().unwrap();
                last_token.value.push(self.current_char);
                last_token.kind = TokenKind::FloatLiteral;
                self.tokens.push(last_token);
                self.is_space = false;
            } else {
                self.tokens.push(Token::new(
                    TokenKind::Dot,
                    self.current_char.to_string(),
                    self.position.filename.clone(),
                    self.position.row,
                    self.position.col,
                    NumberBase::Decimal
                ));
                self.is_space = false;
            }
        }
    }

    pub fn number(&mut self) {
        if self.last_token.kind == TokenKind::None {
            self.tokens.push(Token::new(
                TokenKind::IntLiteral,
                self.current_char.to_string(),
                self.position.filename.clone(),
                self.position.row,
                self.position.col,
                NumberBase::Decimal
            ));
            self.is_space = false;
        } else {

            if self.is_space == false {
                if self.last_token.kind == TokenKind::IntLiteral {
                    // self.tokens
                    //     .last()
                    //     .cloned()
                    //     .unwrap()
                    //     .value
                    //     .push(self.current_char);
                    let mut last_token = self.tokens.pop().unwrap();
                    last_token.value.push(self.current_char);
                    self.tokens.push(last_token);
                    self.is_space = false;
                } else if self.last_token.kind == TokenKind::FloatLiteral {
                    // self.tokens
                    //     .last()
                    //     .cloned()
                    //     .unwrap()
                    //     .value
                    //     .push(self.current_char);
                    let mut last_token = self.tokens.pop().unwrap();
                    last_token.value.push(self.current_char);
                    self.tokens.push(last_token);
                    self.is_space = false;
                } else if self.last_token.kind == TokenKind::Identifier && self.is_space == false {
                    // self.tokens
                    //     .last()
                    //     .cloned()
                    //     .unwrap()
                    //     .value
                    //     .push(self.current_char);
                    let mut last_token = self.tokens.pop().unwrap();
                    last_token.value.push(self.current_char);
                    last_token.kind = TokenKind::Identifier;
                    self.tokens.push(last_token);
                    self.is_space = false;
                } else if self.last_token.kind == TokenKind::Dot {
                    let mut last_token = self.tokens.pop().unwrap();
                    last_token.value.push(self.current_char);
                    last_token.kind = TokenKind::FloatLiteral;
                    self.tokens.push(last_token);
                    self.is_space = false;
                } else {
                    self.tokens.push(Token::new(
                        TokenKind::IntLiteral,
                        self.current_char.to_string(),
                        self.position.filename.clone(),
                        self.position.row,
                        self.position.col,
                        NumberBase::Decimal
                    ));
                    self.is_space = false; 
                }
            } else {
                self.tokens.push(Token::new(
                    TokenKind::IntLiteral,
                    self.current_char.to_string(),
                    self.position.filename.clone(),
                    self.position.row,
                    self.position.col,
                    NumberBase::Decimal
                ));
                self.is_space = false; 
            }
        }
    }

    pub fn get_last_token(&self) -> Token {
        if self.tokens.len() == 0 {
            return Token::new(TokenKind::None, "".to_string(), self.position.filename.clone(), 0, 0, NumberBase::None);
        } else {
            match self.tokens.get(self.tokens.len() - 1) {
                Some(token) => {
                    return token.clone();
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
        return IDENTIFIER_CONSTANTS
            .to_string()
            .find(c)
            .is_some();
    }

    // is char a decimal digit
    pub fn is_digit(&self, c: char) -> bool {
        return DECIMAL_DIGITS_CONSTANTS
            .to_string()
            .find(c)
            .is_some();
    }

    // is char a hexadecimal digit
    pub fn is_x_digit(&self, c: char) -> bool {
        return HEXADECIMAL_DIGITS_CONSTANTS
            .to_string()
            .find(c)
            .is_some();
    }

    // is char a octal digit
    pub fn is_octal_digit(&self, c: char) -> bool {
        return OCTALS_DIGITS_CONSTANTS
            .to_string()
            .find(c)
            .is_some();
    }

    // is char a binary digit
    pub fn is_binary_digit(&self, c: char) -> bool {
        return BINARY_DIGITS_CONSTANTS
            .to_string()
            .find(c)
            .is_some();
    }

    // is char a operator char
    pub fn is_symbol(&self, c: char) -> bool {
        return SYMBOLS_CONSTANTS
            .to_string()
            .find(c)
            .is_some();
    }

    pub fn push_symbol_token(&mut self, kind: TokenKind, value: &str) {
        self.tokens.push(Token::new(
            kind,
            value.to_string(),
            self.position.filename.clone(),
            self.position.row,
            self.position.col,
            NumberBase::None
        ));
    }

    pub fn symbol(&mut self) {
        match self.current_char {
            '(' => self.push_symbol_token(TokenKind::LeftParen, "("),
            ')' => self.push_symbol_token(TokenKind::RightParen, ")"),
            '{' => self.push_symbol_token(TokenKind::LeftCurlyBrackets, "{"),
            '}' => self.push_symbol_token(TokenKind::RightCurlyBrackets, "}"),
            '+' => {
                if self.next_char.is_some() {
                    if self.is_symbol(self.next_char.unwrap()) {
                        if self.next_char.unwrap() == '+' {
                            self.push_symbol_token(TokenKind::Increment, "++");
                            self.advance(1);
                        } else if self.next_char.unwrap() == '=' {
                            self.push_symbol_token(TokenKind::AddAssignment, "+=");
                            self.advance(1);
                        } else {
                            print_error_with_pos(
                                ErrorKind::SyntaxError, 
                                format!("unexpected use of {} after {}",
                                    self.next_char.unwrap(),
                                    self.current_char),
                                TokenPos {
                                    row: self.position.row,
                                    col: self.position.col,
                                },
                                self.position.filename.clone(),
                                false
                            );
                        }
                    } else {
                        self.push_symbol_token(TokenKind::Plus, "+");
                    }
                } else {
                    self.push_symbol_token(TokenKind::Plus, "+");
                }
            },
            '-' => {
                if self.next_char.is_some() {
                    if self.is_symbol(self.next_char.unwrap()) {
                        if self.next_char.unwrap() == '-' {
                            self.push_symbol_token(TokenKind::Decrement, "--");
                            self.advance(1);
                        } else if self.next_char.unwrap() == '=' {
                            self.push_symbol_token(TokenKind::SubstractAssignment, "-=");
                            self.advance(1);
                        } else {
                            print_error_with_pos(
                                ErrorKind::SyntaxError, 
                                format!("unexpected use of {} after {}",
                                    self.next_char.unwrap(),
                                    self.current_char),
                                TokenPos {
                                    row: self.position.row,
                                    col: self.position.col,
                                },
                                self.position.filename.clone(),
                                false
                            );
                        }
                    } else {
                        self.push_symbol_token(TokenKind::Minus, "-");
                    }
                } else {
                    self.push_symbol_token(TokenKind::Minus, "-");
                }
            },
            '*' => {
                if self.next_char.is_some() {
                    if self.is_symbol(self.next_char.unwrap()) {
                        if self.next_char.unwrap() == '=' {
                            self.push_symbol_token(TokenKind::MultiplyAssignment, "*=");
                            self.advance(1);
                        } else {
                            print_error_with_pos(
                                ErrorKind::SyntaxError, 
                                format!("unexpected use of {} after {}",
                                    self.next_char.unwrap(),
                                    self.current_char),
                                TokenPos {
                                    row: self.position.row,
                                    col: self.position.col,
                                },
                                self.position.filename.clone(),
                                false
                            );
                        }
                    } else {
                        self.push_symbol_token(TokenKind::Multiply, "*");
                    }
                } else {
                    self.push_symbol_token(TokenKind::Multiply, "*");
                }
            }, 
            '/' => {
                if self.next_char.is_some() {
                    if self.is_symbol(self.next_char.unwrap()) {
                        if self.next_char.unwrap() == '=' {
                            self.push_symbol_token(TokenKind::DivideAssignment, "/=");
                            self.advance(1);
                        } else {
                            print_error_with_pos(
                                ErrorKind::SyntaxError, 
                                format!("unexpected use of {} after {}",
                                    self.next_char.unwrap(),
                                    self.current_char),
                                TokenPos {
                                    row: self.position.row,
                                    col: self.position.col,
                                },
                                self.position.filename.clone(),
                                false
                            );
                        }
                    } else {
                        self.push_symbol_token(TokenKind::Divide, "/");
                    }
                } else {
                    self.push_symbol_token(TokenKind::Divide, "/");
                }
            },
            '%' => {
                if self.next_char.is_some() {
                    if self.is_symbol(self.next_char.unwrap()) {
                        if self.next_char.unwrap() == '=' {
                            self.push_symbol_token(TokenKind::ModulusAssignment, "%=");
                            self.advance(1);
                        } else {
                            print_error_with_pos(
                                ErrorKind::SyntaxError, 
                                format!("unexpected use of {} after {}",
                                    self.next_char.unwrap(),
                                    self.current_char),
                                TokenPos {
                                    row: self.position.row,
                                    col: self.position.col,
                                },
                                self.position.filename.clone(),
                                false
                            );
                        }
                    } else {
                        self.push_symbol_token(TokenKind::Modulus, "%");
                    }
                } else {
                    self.push_symbol_token(TokenKind::Modulus, "%");
                }
            },
            '=' => {
                if self.next_char.is_some() {
                    if self.is_symbol(self.next_char.unwrap()) {
                        if self.next_char.unwrap() == '=' {
                            self.push_symbol_token(TokenKind::Equals, "==");
                            self.advance(1);
                        } else {
                            print_error_with_pos(
                                ErrorKind::SyntaxError, 
                                format!("unexpected use of {} after {}",
                                    self.next_char.unwrap(),
                                    self.current_char),
                                TokenPos {
                                    row: self.position.row,
                                    col: self.position.col,
                                },
                                self.position.filename.clone(),
                                false
                            );
                        }
                    } else {
                        self.push_symbol_token(TokenKind::Assignment, "=");
                    }
                } else {
                    self.push_symbol_token(TokenKind::Assignment, "=");
                }
            },
            '<' => {
                if self.next_char.is_some() {
                    if self.is_symbol(self.next_char.unwrap()) {
                        if self.next_char.unwrap() == '=' {
                            self.push_symbol_token(TokenKind::LessThanOrEquals, "<=");
                            self.advance(1);
                        } else {
                            print_error_with_pos(
                                ErrorKind::SyntaxError, 
                                format!("unexpected use of {} after {}",
                                    self.next_char.unwrap(),
                                    self.current_char),
                                TokenPos {
                                    row: self.position.row,
                                    col: self.position.col,
                                },
                                self.position.filename.clone(),
                                false
                            );
                        }
                    } else {
                        self.push_symbol_token(TokenKind::LessThan, "<");
                    }
                } else {
                    self.push_symbol_token(TokenKind::LessThan, "<");
                }
            }
            '>' => {
                if self.next_char.is_some() {
                    if self.is_symbol(self.next_char.unwrap()) {
                        if self.next_char.unwrap() == '=' {
                            self.push_symbol_token(TokenKind::BiggerThanOrEquals, ">=");
                            self.advance(1);
                        } else {
                            print_error_with_pos(
                                ErrorKind::SyntaxError, 
                                format!("unexpected use of {} after {}",
                                    self.next_char.unwrap(),
                                    self.current_char),
                                TokenPos {
                                    row: self.position.row,
                                    col: self.position.col,
                                },
                                self.position.filename.clone(),
                                false
                            );
                        }
                    } else {
                        self.push_symbol_token(TokenKind::BiggerThan, ">");
                    }
                } else {
                    self.push_symbol_token(TokenKind::BiggerThan, ">");
                }
            },
            '!' => {
                if self.next_char.is_some() {
                    if self.is_symbol(self.next_char.unwrap()) {
                        if self.next_char.unwrap() == '=' {
                            self.push_symbol_token(TokenKind::NotEquals, "!=");
                            self.advance(1);
                        } else {
                            print_error_with_pos(
                                ErrorKind::SyntaxError, 
                                format!("unexpected use of {} after {}",
                                    self.next_char.unwrap(),
                                    self.current_char),
                                TokenPos {
                                    row: self.position.row,
                                    col: self.position.col,
                                },
                                self.position.filename.clone(),
                                false
                            );
                        }
                    } else {
                        self.push_symbol_token(TokenKind::Bang, "!");
                    }
                } else {
                    self.push_symbol_token(TokenKind::Bang, "!");
                }
            },
            '|' => {
                if self.next_char.is_some() {
                    if self.is_symbol(self.next_char.unwrap()) {
                        print_error_with_pos(
                            ErrorKind::SyntaxError, 
                            format!("unexpected use of {} after {}",
                                self.next_char.unwrap(),
                                self.current_char),
                            TokenPos {
                                row: self.position.row,
                                col: self.position.col,
                            },
                            self.position.filename.clone(),
                            false
                        );
                    } else {
                        self.push_symbol_token(TokenKind::Or, "|");
                    }
                } else {
                    self.push_symbol_token(TokenKind::Or, "|");
                }
            },
            '&' => {
                if self.next_char.is_some() {
                    if self.is_symbol(self.next_char.unwrap()) {
                        print_error_with_pos(
                            ErrorKind::SyntaxError, 
                            format!("unexpected use of {} after {}",
                                self.next_char.unwrap(),
                                self.current_char),
                            TokenPos {
                                row: self.position.row,
                                col: self.position.col,
                            },
                            self.position.filename.clone(),
                            false
                        );
                    } else {
                        self.push_symbol_token(TokenKind::And, "&");
                    }
                } else {
                    self.push_symbol_token(TokenKind::And, "&");
                }
            },
            '?' => {
                if self.next_char.is_some() {
                    if self.is_symbol(self.next_char.unwrap()) {
                        print_error_with_pos(
                            ErrorKind::SyntaxError, 
                            format!("unexpected use of {} after {}",
                                self.next_char.unwrap(),
                                self.current_char),
                            TokenPos {
                                row: self.position.row,
                                col: self.position.col,
                            },
                            self.position.filename.clone(),
                            false
                        );
                    } else {
                        self.push_symbol_token(TokenKind::QuestionMark, "?");
                    }
                } else {
                    self.push_symbol_token(TokenKind::QuestionMark, "?");
                }
            },
            ':' => {
                if self.next_char.is_some() {
                    if self.is_symbol(self.next_char.unwrap()) {
                        print_error_with_pos(
                            ErrorKind::SyntaxError, 
                            format!("unexpected use of {} after {}",
                                self.next_char.unwrap(),
                                self.current_char),
                            TokenPos {
                                row: self.position.row,
                                col: self.position.col,
                            },
                            self.position.filename.clone(),
                            false
                        );
                    } else {
                        self.push_symbol_token(TokenKind::Colon, ":");
                    }
                } else {
                    self.push_symbol_token(TokenKind::Colon, ":");
                }
            }
            _ => {},
            // _ => unimplemented!(),
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
        if self.data.len() == self.index+1 {
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

