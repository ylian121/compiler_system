use std::iter::Peekable;
use std::vec;
use std::error::Error;
// use std::env;

struct Lex {
    it: Peekable<vec::IntoIter<u8>>,
    line: usize,
    problem: Option<Box<dyn Error>>,
}
#[derive(Debug)]
enum Tok {
    Return, // "return"
    Int, // "int"
    Print, // "print"
    Read, // "read"
    While, // "while"
    If, // "if"
    Else, // "else"
    Break, // "break"
    Continue, // "continue"
    LeftParen, // "("
    RightParen, // ")"
    LeftCurly, // "{"
    RightCurly, // "}"
    LeftBracket, // "["
    RightBracket, // "]"
    Comma, // ","
    Semicolon, // ";"
    Plus, // "+"
    Substract, // "-"
    Multiply, // "*"
    Divide, // "/"
    Modulus, // "%"
    Assign, // "="
    Less, // "<"
    LessEqual, // "<="
    Greater, // ">"
    GreaterEqual, // ">="
    Equality, // "=="
    NotEqual, // "!="
    Ident, // ([a-z]|[A-Z])([a-z]|[A-Z]|_|[0-9])*
    Num, // [0-9]+
}

impl Lex {
    fn make(file_path: &str) -> Result<Lex, Box<dyn Error>> {
        Ok(Lex{
            it:std::fs::read_to_string(file_path)?.into_bytes().into_iter().peekable(),
            line:0,
            problem:None,
        })
    }
    // fn lex (&mut self) -> Option<Tok> {
    //     let byte = self.it.peek()?;
    //     self.it.next();

    //     Some(Tok::Ass)
    // }

    fn lex (&mut self) -> Option<Tok> {
        // let byte = self.it.peek()?;
        match self.it.peek()? {
            b'[' => {self.it.next(); Some(Tok::LeftBracket)}, b']' => {self.it.next(); Some(Tok::RightBracket)},
            b'(' => {self.it.next(); Some(Tok::LeftParen)}, b')' => {self.it.next(); Some(Tok::RightParen)},
            b'{' => {self.it.next(); Some(Tok::LeftCurly)}, b'}' => {self.it.next(); Some(Tok::RightCurly)},
            b'+' => {self.it.next(); Some(Tok::Plus)}, b'-' => {self.it.next(); Some(Tok::Substract)},
            b'=' => {self.it.next(); Some(Tok::Assign)},
            b'\n' => {self.line += 1; self.it.next(); self.lex()},
            b' ' | b'\t' | b'\r' => {self.it.next(); self.lex()},
            b'A' ..=b'Z' | b'a' ..=b'z' | b'_' => {self.lex_id()}, // TODO: Implement lex_id()
            b'0' ..=b'9' => {return self.lex_num();}, // TODO: Implement lex_num()
            b'#' => {return self.lex_com();}, // TODO: Implement lex_com()

            ch => {self.problem = Some(format!("Lexer: found an invalid char {}", ch).into()); None}
        }
    }

    fn lex_id (&mut self) -> Option<Tok> {// TODO: Implement lex_id()
        match self.it.peek()? {
            
        }
    }

    fn lex_num (&mut self) -> Option<Tok> {// TODO: Implement lex_num()
        match self.it.peek()? {
            
        }
    }

    fn lex_com (&mut self) -> Option<Tok> {// TODO: Implement lex_com()
        match self.it.peek()? {
            
        }
    }

}
fn main() -> Result<(), Box<dyn Error>> {
    // env::set_var("RUST_BACKTRACE", "full");

    let args : Vec<String> = std::env::args().collect();
    let mut lex = Lex::make(&args[1])?;

    while let Some(tok) = lex.lex() {
        println!("{:?}", tok);
    }

    Ok(())
}
