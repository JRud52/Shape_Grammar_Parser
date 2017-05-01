use std::collections::VecDeque;

pub enum Ident {
    ParaLeft, ParaRight,
    OpPlus, OpMinus, OpMult, OpDiv, OpAssign,
    VarName, Method, IntLit,
    Polygon, Extrude, Slice, Split, Translate,
    Unknown,
}

pub struct Token {
    pub identifier: String,
    pub ident_type: Ident,
}

pub struct Lexer {

    buffer: VecDeque<char>,
    next_char: char,
    is_done: bool,
}

impl Lexer{
    pub fn new(buffer: String) -> Lexer {
        let mut char_buffer = buffer.chars().collect::<VecDeque<char>>();
        let next_char_var;

        match char_buffer.pop_front() {
            Some(x) => next_char_var = x,
            None => next_char_var = '0',
        }

        Lexer { buffer: char_buffer, next_char: next_char_var, is_done: false }
    }

}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {

        if self.is_done { return None }

        let mut token_string = String::new();
        let mut ident_type = Ident::Unknown;

        if self.next_char.is_whitespace() {
            while self.next_char.is_whitespace() {
                if let Some(x) = self.buffer.pop_front() {
                    self.next_char = x;
                } else {
                    return None;
                }
            }
        }

        if self.next_char.is_alphabetic() {
            token_string.push(self.next_char);
            if let Some(x) = self.buffer.pop_front() {
                self.next_char = x;
            } else {
                self.is_done = true;
                self.next_char = ' ';
            }

            while self.next_char.is_alphanumeric() {
                token_string.push(self.next_char);
                if let Some(x) = self.buffer.pop_front() {
                    self.next_char = x;
                } else {
                    self.is_done = true;
                    break;
                }
            }
            match token_string.as_ref() {
                "polygon"  => ident_type = Ident::Polygon,
                "extrude"  => ident_type = Ident::Extrude,
                "slice"  => ident_type = Ident::Slice,
                "split"  => ident_type = Ident::Split,
                "translate"  => ident_type = Ident::Translate,
                _          => ident_type = Ident::VarName,
            }
        }

        else if self.next_char.is_numeric() {
            token_string.push(self.next_char);
            if let Some(x) = self.buffer.pop_front() {
                self.next_char = x;
            } else {
                self.is_done = true;
                self.next_char = ' ';
            }

            while self.next_char.is_numeric(){
                token_string.push(self.next_char);
                if let Some(x) = self.buffer.pop_front() {
                    self.next_char = x;
                } else {
                    self.is_done = true;
                    break;
                }
            }
            ident_type = Ident::IntLit;
        }

        else {
            token_string.push(self.next_char);
            if let Some(x) = self.buffer.pop_front() {
                self.next_char = x;
            } else {
                self.is_done = true;
                self.next_char = ' ';
            }

            match token_string.as_ref() {
                "=" => ident_type = Ident::OpAssign,
                "+" => ident_type = Ident::OpPlus,
                "-" => ident_type = Ident::OpMinus,
                "*" => ident_type = Ident::OpMult,
                "/" => ident_type = Ident::OpDiv,
                "(" => ident_type = Ident::ParaLeft,
                ")" => ident_type = Ident::ParaRight,
                _   => ident_type = Ident::Unknown,
            }

        }

        Some(Token {identifier: token_string, ident_type: ident_type})
    }


}

