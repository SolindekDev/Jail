use jail_token::*;
use jail_lex::*;

fn main() {
    let mut lexer = Lexer::new(
        Data::new(
            "fn main() {

            }".to_string())    
    );
    let number_token = Token::new(TokenKind::IntLiteral, "20".to_string(), "main.ja".to_string(), 0, 0, NumberBase::None);
    println!("{}, {}", number_token.value, number_token.kind.get_pretty_name());
}
