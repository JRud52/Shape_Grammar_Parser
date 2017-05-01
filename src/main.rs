use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod lexer;
use lexer::Ident;
use lexer::Token;

fn main() {
    let mut buffer = String::new();

    {
        // Create a path to the desired file
        let path = Path::new("gram/shape.txt");
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display,
                               why.description()),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        match file.read_to_string(&mut buffer) {
            Err(why) => panic!("couldn't read {}: {}", display,
                               why.description()),
            Ok(_) => println!("{} successfully loaded", display),
        }
        // `file` goes out of scope, and the input file gets closed
    }

    let mut lex = lexer::Lexer::new(buffer);

    loop {
        let token = match lex.next(){
            None => {println!("End of file"); break;},
            Some(value) => value,
        };

        println!("{}", token.identifier );
    }
}