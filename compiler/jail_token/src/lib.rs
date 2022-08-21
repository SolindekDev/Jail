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
    ModulusAssignment,
    Increment,
    Decrement,
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
    Newline,
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

impl NumberBase {
    pub fn get_pretty(&self) -> String {
        match self {
            NumberBase::None => "None".to_string(),
            NumberBase::Binary => "Binary".to_string(),
            NumberBase::Octal => "Octal".to_string(),
            NumberBase::Decimal => "Decimal".to_string(),
            NumberBase::Hexadecimal => "Hexadecimal".to_string(),
        }
    } 
}

impl TokenKind {
    pub fn get_pretty(&self) -> String {
        match self {
            TokenKind::Identifier => "Identifier".to_string(),
            TokenKind::StringLiteral => "StringLiteral".to_string(),
            TokenKind::FloatLiteral => "FloatLiteral".to_string(),
            TokenKind::IntLiteral => "IntLiteral".to_string(),
            TokenKind::LeftParen => "LeftParen".to_string(),
            TokenKind::RightParen => "RightParen".to_string(),
            TokenKind::LeftCurlyBrackets => "LeftCurlyBrackets".to_string(),
            TokenKind::RightCurlyBrackets => "RightCurlyBrackets".to_string(),
            TokenKind::Plus => "Plus".to_string(),
            TokenKind::Minus => "Minus".to_string(), 
            TokenKind::Multiply => "Multiply".to_string(),
            TokenKind::Divide => "Divide".to_string(),
            TokenKind::Modulus => "Modulus".to_string(),
            TokenKind::Assignment => "Assignment".to_string(),
            TokenKind::AddAssignment => "AddAssignment".to_string(),
            TokenKind::SubstractAssignment => "SubstractAssignment".to_string(),
            TokenKind::MultiplyAssignment => "MultiplyAssignment".to_string(),
            TokenKind::DivideAssignment => "DivideAssignment".to_string(),
            TokenKind::ModulusAssignment => "ModulusAssignment".to_string(),
            TokenKind::Increment => "Increment".to_string(),
            TokenKind::Decrement => "Decrement".to_string(),
            TokenKind::BiggerThan => "BiggerThan".to_string(),
            TokenKind::LessThan => "LessThan".to_string(),
            TokenKind::Equals => "Equals".to_string(),
            TokenKind::NotEquals => "NotEquals".to_string(),
            TokenKind::BiggerThanOrEquals => "BiggerThanOrEquals".to_string(),
            TokenKind::LessThanOrEquals => "LessThanOrEquals".to_string(),
            TokenKind::Or => "Or".to_string(),
            TokenKind::And => "And".to_string(),
            TokenKind::Bang => "Bang".to_string(),
            TokenKind::QuestionMark => "QuestionMark".to_string(),
            TokenKind::Colon => "Colon".to_string(),
            TokenKind::Newline => "Newline".to_string(),
            TokenKind::None => "None".to_string(),
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

    pub fn to_string(&self) -> String {
        format!("{{\n\t'kind': '{}',\n\t'value': '{}',\n\t'base': '{}',\n\t'filename': '{}',\n\t'pos': {{\n\t\t'row': {},\n\t\t'col': {},\n\t}},\n}},"
            , self.kind.get_pretty(), self.value, 
            self.base.get_pretty(), self.filename, 
            self.pos.row, self.pos.col)
    }
}