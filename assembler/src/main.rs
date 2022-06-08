mod lexer;
mod reconstruct;
mod definitions;

// Imports
use std::env;
use std::path::Path;
use std::fs;
use colored::Colorize;
use lexer::{Token};
use reconstruct::reconstruct;

// Type Aliases
pub type StringList = Vec<String>;

fn main() {
    let args : StringList = env::args().collect();
    let path_in = Path::new(&args[1]);
    let path_out = path_in.with_extension("s");

    println!("Assembling {:?} => {:?}", path_in, path_out);

    let contents = fs::read_to_string(path_in)
                        .expect("Error Reading File");

    println!("---- INPUT ----\n{}\n---- END INPUT ----\n", contents);

    match Token::lex(&contents) {
        Ok(tokens) => {
            let len = tokens.len();
            let output = reconstruct(tokens);
            match output {
                Ok(x) => {
                    fs::write(path_out, x);
                },
                Err(x) => {
                    print!("\n---- ERROR ----\n{}\n", x.red());
                },
            }
        },
        Err(_) => {}
    }

}
