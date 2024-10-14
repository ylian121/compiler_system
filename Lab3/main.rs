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
        Ident(Vec<u8>), // ([a-z]|[A-Z])([a-z]|[A-Z]|_|[0-9])*
        Num(Vec<u8>) // [0-9]+
}

impl Lex {
    fn make(file_path: &str) -> Result<Lex, Box<dyn Error>> {
        Ok(Lex{
            it:std::fs::read_to_string(file_path)?.into_bytes().into_iter().peekable(),
            line:0,
            problem:None,
        })
    }
    fn lex (&mut self) -> Option<Tok> {
        //let byte = self.it.peek()?;
        match self.it.peek()? {
            b '[' => { self.it.next(); Some(Tok::LeftBracket)},
            b ']' => { self.it.next(); Some(Tok::RightBracket)},
            b '(' => { self.it.next(); Some(Tok::LeftParen)},
            b ')' => { self.it.next(); Some(Tok::RightParen)},
            b '{' => { self.it.next(); Some(Tok::LeftCurly)},
            b '}' => { self.it.next(); Some(Tok::RightCurly)},
            b '+' => { self.it.next(); Some(Tok::Plus)},
            b '-' => { self.it.next(); Some(Tok::Subtract)},
            b ',' => { self.it.next(); Some(Tok::Comma)},
            b ';' => { self.it.next(); Some(Tok::Semicolon)},
            b '*' => { self.it.next(); Some(Tok::Multiply)},
            b '/' => { self.it.next(); Some(Tok::Divide)},
            b '%' => { self.it.next(); Some(Tok::Modulus)},
            b '=' => { self.it.next(); Some(Tok::Assign)},
            b '<' => { self.it.next(); Some(Tok::Less)},
            b '<=' => { self.it.next(); Some(Tok::LessEqual)},
            b '>' => { self.it.next(); Some(Tok::Greater)},
            b '>=' => { self.it.next(); Some(Tok::GreaterEqual)},
            b '==' => { self.it.next(); Some(Tok::Equality)},
            b '!=' => { self.it.next(); Some(Tok::NotEqual)},
            _ => 
        }
        //self.it.next();

        //Some(Tok::Ass)
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
