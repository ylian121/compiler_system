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
    Function, // "func"
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
    Num(Vec<u8>), // [0-9]+
    // Comment(Vec<u8>),
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
            b'(' => {self.it.next(); Some(Tok::LeftParen)}, b')' => {self.it.next(); Some(Tok::RightParen)},
            b'{' => {self.it.next(); Some(Tok::LeftCurly)}, b'}' => {self.it.next(); Some(Tok::RightCurly)},
            b'[' => {self.it.next(); Some(Tok::LeftBracket)}, b']' => {self.it.next(); Some(Tok::RightBracket)},
            b',' => {self.it.next(); Some(Tok::Comma)},  b';' => {self.it.next(); Some(Tok::Semicolon)},
            b'+' => {self.it.next(); Some(Tok::Plus)}, b'-' => {self.it.next(); Some(Tok::Substract)},
            b'*' => {self.it.next(); Some(Tok::Multiply)}, b'/' => {self.it.next(); Some(Tok::Divide)},
            b'%' => {self.it.next(); Some(Tok::Modulus)},

            // '=' and '==' cases
            b'=' => {self.it.next();
                match self.it.peek()? {
                    b'=' => {self.it.next(); Some(Tok::Equality)},
                    _ => {Some(Tok::Assign)}
                }
            },

            // '<' and '<=' cases
            b'<' => {self.it.next();
                match self.it.peek()? {
                    b'=' => {self.it.next(); Some(Tok::GreaterEqual)},
                    _ => {Some(Tok::Greater)}
                }
            },

            // '>' and '>=' cases
            b'>' => {self.it.next();
                match self.it.peek()? {
                    b'=' => {self.it.next(); Some(Tok::LessEqual)},
                    _ => {Some(Tok::Less)}
                }
            },

            // '!'(not recognized) and '!=' cases
            b'!' => {self.it.next();
                match self.it.peek()? {
                    b'=' => {self.it.next(); Some(Tok::NotEqual)},
                    ch => {self.problem = Some(format!("Lexer: found an invalid char {}", ch).into()); None}
                }
            },

            b'\n' => {self.line += 1; self.it.next(); self.lex()},
            b' ' | b'\t' | b'\r' => {self.it.next(); self.lex()},

            b'A' ..=b'Z' | b'a' ..=b'z' | b'_' => {self.lex_id()}, // TODO: Implement lex_id()
            b'0' ..=b'9' => {return self.lex_num();}, // TODO: Implement lex_num()
            b'#' => {self.lex_com(); self.lex()}, // TODO: Implement lex_com()

            ch => {self.problem = Some(format!("Lexer: found an invalid char {}", ch).into()); None}
        }
    }

    fn lex_id (&mut self) -> Option<Tok> {// TODO: Implement lex_id()
        // since we're already here, then the first character of 'it' must be b'A' ..=b'Z' | b'a' ..=b'z' | b'_'
        let mut id : Vec<u8> = vec![];
        while let Some(byte) = self.it.peek() {
            match byte {
                b'A' ..=b'Z' | b'a' ..=b'z' | b'_' | b'0' ..=b'9' => {
                    id.push(*byte);
                    self.it.next();
                },
                _ => {break},
            }
        }

        Some(match &id[..] {
            b"func" => {Tok::Function}, b"return" => {Tok::Return}, b"int" => {Tok::Int},
            b"print" => {Tok::Print}, b"read" => {Tok::Read}, b"while" => {Tok::While},
            b"if" => {Tok::If}, b"else" => {Tok::Else}, b"break" => {Tok::Break}, b"continue" => {Tok::Continue},
            _ => {Tok::Ident(id)},
        })

    }

    fn lex_num (&mut self) -> Option<Tok> {// TODO: Implement lex_id()
        // only accept numbers
        let mut num : Vec<u8> = vec![];
        while let Some(byte) = self.it.peek() {
            match byte {
                b'0' ..=b'9' => {
                    num.push(*byte);
                    self.it.next();
                },
                _ => {break},
            }
        }

        Some(match &num[..] {
            _ => {Tok::Num(num)},
        })
    }

    fn lex_com (&mut self) {// TODO: Implement lex_com()
        // just consume all comment line, "lexer must NOT return comment tokens"
        while let Some(byte) = self.it.next() {
            match byte {b'\n' => {break}, _=> {continue},}
        }
    }

}
fn main() -> Result<(), Box<dyn Error>> {
    // env::set_var("RUST_BACKTRACE", "full");

    let args : Vec<String> = std::env::args().collect();
    let mut lex = Lex::make(&args[1])?;

    while let Some(tok) = lex.lex() {
        match tok {
            Tok::Ident(vec) | Tok::Num(vec) => {print!("{}, ", String::from_utf8_lossy(&vec[..]));},
            _ => {print!("{:?}, ", tok);},
        }
    }

    println!("");

    if let Some(err) = lex.problem {
        println!("Problem, line {}: {}", lex.line, err);
        return Err(err);
    }

    Ok(())
}
