use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod lexer;
mod parser;
use lexer::Ident;
use lexer::Token;

fn main() {
    let mut buffer = String::new();

    { // scope for file
        // Create a path to the desired file
        let path = Path::new("gram/shape.txt");
        let display = path.display();

        // Open the path in read-only mode, returns Result<File>
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display,
                               why.description()),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns Result<usize>
        match file.read_to_string(&mut buffer) {
            Err(why) => panic!("couldn't read {}: {}", display,
                               why.description()),
            Ok(_) => println!("{} successfully loaded", display),
        }

    } // file goes out of scope, and gets closed

    //let mut lex = lexer::Lexer::new(buffer);
    let mut parser = parser::Parser::new(buffer);

    parser.parse();

}