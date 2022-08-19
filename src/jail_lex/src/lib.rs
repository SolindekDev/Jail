use jail_token::Token;
use jail_token::TokenKind;
use jail_error::LexerError;

// Data 

pub struct Data<T>{
    content: T
}

impl <T>Data<T>{
    pub fn new(content: T) -> Self{
        Self{content}
    }
}


// Lexer

struct LexerPosition{
    row: i32,
    col: i32,
}

impl LexerPosition{
    fn new(row: i32, col: i32) -> Self{
        Self{row, col}
    }
}

pub struct Lexer{
    data: Data<String>,
    position: LexerPosition,
}

impl Lexer{
    pub fn new(data: Data<String>) -> Self{
        Self{
            data: data, 
            position: LexerPosition::new(-1,-1),
        }
    }
    pub fn next_token(&mut self) -> Result<Token, LexerError>{
        self.position.col += 1;
        match self.data.content.as_bytes()[self.position as usize] as char{
            '\n' => {
                self.position.row += 1;
                Token::new(TokenKind::NewLine, "\n".to_string(), "main.rs", self.position.row, self.position.col, NumberBase::None)
            }
            _ => {Err(LexerError::new("Not implemented yet..."))}
        }        
    }
}

