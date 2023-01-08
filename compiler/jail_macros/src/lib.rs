use proc_macro2::*;
use proc_macro::{TokenStream,Ident};
use quote::quote;

#[proc_macro]
pub fn doublesign_expression(stream: TokenStream) -> TokenStream{
    let string_stream = stream.to_string();
    let stream_array = string_stream.split(' ').collect::<Vec<&str>>();
    let first: proc_macro2::TokenStream = stream_array[0].parse().unwrap();
    let second: proc_macro2::TokenStream = stream_array[1].parse().unwrap();
    let all: proc_macro2::TokenStream::Literal = format!("{}{}", stream_array[0], stream_array[1]).parse().unwrap();
    let first_token_kind: syn::Expr = syn::parse_str(format!("{}{}{}", stream_array[2], stream_array[3], stream_array[4]).as_str()).unwrap();
    let second_token_kind: syn::Expr = syn::parse_str(format!("{}{}{}", stream_array[5], stream_array[6], stream_array[7]).as_str()).unwrap();
    let tokens = quote!{
        if self.next_char.is_some() {
            if self.is_symbol(self.next_char.unwrap()) {
                if self.next_char.unwrap() == #second {
                    self.push_symbol_token(#second_token_kind, #all);
                    self.advance(1);
                } else {
                    self.is_error = true; print_error_with_line_and_pos(
                        ErrorKind::SyntaxError, 
                        format!("unexpected use of {} after {}",
                            self.next_char.unwrap(),
                            self.current_char),
                        TokenPos {
                            row: self.position.row,
                            col: self.position.col,
                        }, self.position.filename.clone(), 
                        lines[(self.position.row - 1) as usize].to_string(), false)
                }
            } else {
                self.push_symbol_token(#first_token_kind, #first);
            }
        } else {
            self.push_symbol_token(#first_token_kind, #first);
        }
    };
    TokenStream::from(tokens)
}
