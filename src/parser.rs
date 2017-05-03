use lexer::Lexer;
use lexer::Token;
use lexer::Ident;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(buffer: String) -> Parser {
        let lexer = Lexer::new(buffer);
        Parser {lexer: lexer}
    }

    pub fn parse(&mut self){
        // loop through the buffer until all the tokens have been consumed
        loop {
            let token = match self.lexer.next(){
                None => { println!("End of file"); break; },
                Some(value) => value,
            };

            println!("{}", token.identifier );
        }
    }

}