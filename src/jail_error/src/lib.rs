use std::fmt;

#[derive(Debug, Clone)]
pub struct LexerError{
    pub kind: String
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}



impl LexerError{
    pub fn new(kind: &str) -> Self{
        Self{kind: kind.to_string()}
    }
}

