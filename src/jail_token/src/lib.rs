pub enum TokenKind {
    Identifier,
    StringLiteral,
    FloatLiteral,
    IntLiteral,
    LeftParen,
    RightParen,
    LeftCurlyBrackets,
    RightCurlyBrackets,
    Plus,
    Minus, 
    Multiply,
    Divide,
    Modulus,
    Assignment,
    AddAssignment,
    SubstractAssignment,
    MultiplyAssignment,
    DivideAssignment,
    Increment,
    Dectrement,
    BiggerThan,
    LessThan,
    Equals,
    NotEquals,
    BiggerThanOrEquals,
    LessThanOrEquals,
    Or,
    And,
    Bang,
    QuestionMark,
    Colon,
    None
}

pub enum NumberBase {
    None,
    Binary,
    Octal,
    Decimal, 
    Hexadecimal,
}

pub struct TokenPos {
    pub row: i32,
    pub col: i32,
}

pub struct Token {
    pub kind: TokenKind,
    pub pos: TokenPos,
    pub base: NumberBase,
    pub value: String,
    pub filename: String,
}

impl TokenKind {
    pub fn get_pretty_name(&self) -> String {
        match self {
            TokenKind::Identifier => "Identifier".to_string(),
            TokenKind::StringLiteral => "StringLiteral".to_string(),
            TokenKind::FloatLiteral => "FloatLiteral".to_string(),
            TokenKind::IntLiteral => "IntLiteral".to_string(),
            TokenKind::None => "None".to_string(),
            _ => "UnknownKind".to_string()
        }
    }
}

impl Token {
    pub fn new(kind: TokenKind, value: String, filename: String, row_pos: i32, col_pos: i32, base: NumberBase) -> Token {
        let pos = TokenPos{ 
            row: row_pos,
            col: col_pos,
        };

        Token{ 
            kind, 
            pos, 
            value, 
            filename, 
            base 
        }
    }
}