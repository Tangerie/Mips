// Imports
use std::env;
use std::path::Path;

// Type Aliases
pub type StringList = Vec<String>;

fn main() {
    let args : StringList = env::args().collect();
    let path_in = Path::new(&args[1]);
    let path_out = path_in.with_extension("txt");

    println!("Opening {:?} => {:?}", path_in, path_out);
}
