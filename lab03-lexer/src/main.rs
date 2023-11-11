use crate::tokens::Token;

pub mod parser;
pub mod tokens;

#[macro_use]
extern crate nom;
#[macro_use]
extern crate nom_locate;

fn main() {
    println!("{}", Token::EqualEqual);
}
