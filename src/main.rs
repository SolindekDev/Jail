use jail_token::*;

fn main() {
    let number_token = Token::new(TokenKind::IntLiteral, "20".to_string(), "main.ja".to_string(), 0, 0, NumberBase::None);
    println!("{}, {}", number_token.value, number_token.kind.get_pretty_name());
}
