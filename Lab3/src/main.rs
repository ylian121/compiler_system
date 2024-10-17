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
        Func, // "function"
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
        Id(Vec<u8>), // ([a-z]|[A-Z])([a-z]|[A-Z]|_|[0-9])*
        Num(Vec<u8>), // [0-9]+
        Com(Vec<u8>)
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
            b "<=" => { self.it.next(); Some(Tok::LessEqual)},
            b '>' => { self.it.next(); Some(Tok::Greater)},
            b ">=" => { self.it.next(); Some(Tok::GreaterEqual)},
            b "==" => { self.it.next(); Some(Tok::Equality)},
            b "!=" => { self.it.next(); Some(Tok::NotEqual)},
            //ignoring newline and space so it will be called recursively
            b '\n' => { self.line += 1; self.it.next(); self.lex()},
            b ' ' => { self.it.next(); self.lex()},
            b '0' ..=b '9' => { return self.lex_num(); },
            b 'A' ..=b 'Z' | b 'a' ..=b 'z' | b '_' => { return self.lex_id(); },
            b '#' => { return self.lex_com(); },
            _ => { self.problem = Some(format!("Lexer: found invalid char {}," _).into()); None }
        }
        //self.it.next();

        //Some(Tok::Ass)
    }
    fn lex_id (&mut self) -> Option<Tok> {
        //let byte = self.it.peek()?;
        let mut id : Vec<u8> = vec![];
        while let Some(byte) = self.it.peek(){
            match byte {
                b 'A' ..=b 'Z' | b 'a' ..=b 'z' | b '_' | b '0' ..=b '9' => { 
                    id.push(*byte);
                    self.it.next();
                },
                _ => { break },
            }
        }

        Some(match &id[..] {
            b "fn" => {Tok::Func},
            b "return" => {Tok::Return},
            b "int" => {Tok::Int},
            b "print" => {Tok::Print},
            b "read" => {Tok::Read},
            b "while" => {Tok::While},
            b "if" => {Tok::If},
            b "else" => {Tok::Else},
            b "break" => {Tok::Break},
            b "continue" => {Tok::Continue},
            _ => { Tok::Id(id) },
        })
    }

    fn lex_num (&mut self) -> Option<Tok> {
        //let byte = self.it.peek()?;
        let mut num : Vec<u8> = vec![];
        while let Some(byte) = self.it.peek(){
            match byte {
                b '0' ..=b '9' => { 
                    num.push(*byte);
                    self.it.next();
                },
                _ => { break },
            }
        }

        Some(match &num[..] {
            _ => { Tok::Num(num) },
        })
    }

    fn lex_com (&mut self) -> Option<Tok> {
        //let byte = self.it.peek()?;
        let mut com : Vec<u8> = vec![];
        while let Some(byte) = self.it.peek(){
            match byte {
                b '#' => { 
                    com.push(*byte);
                    self.it.next();
                },
                _ => { break },
            }
        }
        None
    }

    
}



fn main() -> Result<(), Box<dyn Error>> {
    // env::set_var("RUST_BACKTRACE", "full");

    let args : Vec<String> = std::env::args().collect();
    let mut lex = Lex::make(&args[1])?;

    while let Some(token) = lex.lex() {
        match token {
            Tok::Id(vec) | Tok::Num(vec) => { print!("{}, ", String::from_utf8_lossy(&vec[..])); },
            _ => { print!("{:?}, ", token); },

        }
    }

    println!("");

    /*
    if let Some(err) = lex.probelm {
        println!("Problem, line {}: {}", lex.line, err);
        return Err(err);
    }
    */

    Ok(())
}
