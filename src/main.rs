#![allow(dead_code)]

use ru::lexer::Lexer;
use ru::parser::Parser;

fn main() {
    let mut lexer = Lexer::new();

    lexer.input_stream(String::from(
        "
    74540  5  6


    50 ",
    ));

    let mut parser = Parser::new(&mut lexer);

    println!("{:#?}", parser.parse());

    // println!("{:#?}", lexer.next_token());

    // let chars_vec = lexer.str.chars().collect::<Vec<char>>();

    // for _ in chars_vec.iter() {
    //     let is = lexer.skip_whitespace();

    //     if is {
    //         continue;
    //     }

    //     if lexer.eof() {
    //         break;
    //     }

    //     println!("{:#?}", lexer.number());
    // }

    // println!("chars_vec {:?}", chars_vec);
    println!("{:?}!", lexer);
}
