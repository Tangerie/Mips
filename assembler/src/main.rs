mod definitions;
mod lexer;
mod reconstruct;

// Imports
use colored::Colorize;
use lexer::Token;
use reconstruct::reconstruct;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

// Type Aliases
pub type StringList = Vec<String>;

fn main() {
    let args: StringList = env::args().collect();
    let path_in = Path::new(&args[1]);
    let path_out = path_in.with_extension("s");

    println!("Assembling {:?} => {:?}", path_in, path_out);

    let contents = fs::read_to_string(path_in).expect("Error Reading File");

    println!("---- INPUT ----\n{}\n---- END INPUT ----\n", contents);

    match Token::lex(&contents) {
        Ok(tokens) => {
            match tokens_to_file(tokens, path_out) {
                Ok(_) => (),
                Err(err) => print!("\n---- RECONSTRUCT ERR ----\n{:?}\n", err.red()),
            }
        }
        Err(errs) => print!("\n---- LEX ERR ----\n{:?}\n", errs.join("\n").red())
    }
}

fn tokens_to_file(tokens : Vec<Token>, path_out : PathBuf) -> Result<(), String> {
    let output = reconstruct(tokens);
    match output {
        Ok(x) => {
            match fs::write(path_out, x) {
                Ok(_) => return Ok(()),
                Err(_) => return Err("Failed to write file".to_string()),
            }
        }
        Err(x) => Err(x)
    }
}