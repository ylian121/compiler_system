// TODO01: create Par struct
// TODO02: create Par impl and its functions
// TODO03: move lexing procedure to par's fn make()
// TODO04: modify main function to call parser instead of lexer
// TODO05: deal with temp values in expressions function
// TODO06: Add the rest of the grammar rules into the parser impl

use std::iter::Peekable;
use std::vec;
use std::error::Error;
use slice_deque::SliceDeque;
// use std::env;

struct Par {
    lex: Lex, // generate token
    toks: SliceDeque<Tok>, // buffers tokens
    problem: Option<Box<dyn Error>>,
    // temp names --V
    t_count: usize,
    l_count: usize,
}

impl Par {
    fn make(file_path: &str) -> Result<Par, Box<dyn Error>> {
        Ok(Par{
            lex: Lex::make(file_path)?, toks: SliceDeque::new(), problem:None,
            t_count: 0, l_count: 0,
        })
    }
    fn tokens(&mut self, amt: usize) -> &mut [Tok] { // buffers token

        while self.toks.len() < amt {self.toks.push_back(self.lex.next());}

        &mut self.toks[0..amt]
    }

    fn consume (&mut self, amt: usize) {for _ in 0..amt {self.toks.pop_front();}} // reduce/consume tokens

    fn temp_name(&mut self) -> Vec<u8> { // deals with naming temporary values
        let mut res = Vec::from(b"temp");
        res.extend_from_slice(&self.l_count.to_string().into_bytes());
        self.t_count += 1;
        res
    }

    fn temp_label(&mut self) -> Vec<u8> { // helps label temporary names
        let mut res = Vec::from(b"label");
        res.extend_from_slice(&self.l_count.to_string().into_bytes());
        self.l_count += 1;
        res
    }
    // TODO: Grammar stuff starts here ------------------------------------------------------------------------------
    // prog:
    // | func prog
    fn parse(&mut self) -> Option<()> {
        match self.tokens(1) {
            &mut [Tok::Function] => {self.func()},
            &mut [Tok::Empty] => {None},
            _ => {self.problem = Some(format!("Parsing Error: program").into()); None},
        }
    }
    
    fn expect (&mut self, t:Tok) -> Option<()> { // helper function thanks to Josue
        if std::mem::discriminant(&t) == std::mem::discriminant(&self.tokens(1)[0]){
            self.consume(1);
            return Some(());
        } else {
            self.problem = Some(format!("Parsing Error: Expected {:?}...", t).into());
            return None;
        }
        
    }

    // func: Func Ident LeftParen param_list RightParen block
    // param_list:
    //           | params
    // params: Int Ident Comma params
    //       | Int Ident
    fn func(&mut self) -> Option<()> {
        let name = match self.tokens(3) { // Func Ident LeftParen?
            &mut [Tok::Function, Tok::Ident(ref mut id), Tok::LeftParen] => {
                let name = std::mem::take(id);
                self.consume(3); // we matched 3 tokens, no need for them anymore
                name
            },
            _ => {
                self.problem = Some(format!("Parsing Error: Function").into());
                return None;
            },
        };
        
        print!("function header: {}", String::from_utf8_lossy(&name)); // output function header
        
        let mut first = true;


        loop {

            if let Tok::RightParen = self.tokens(1)[0] { // what if ')'?
                self.consume(1);
                print!("\n"); // finished reading params, newline
                break;
            }
            if !first{
                self.expect(Tok::Comma)?; // remember '?'
                // if let Tok::Comma = self.tokens(1)[0] { // what if 'int'?
                //     self.consume(1);
                // } else {
                //     self.problem = Some(format!("Parsing Error: Expected Comma...").into());
                //     return None;
                // }
            }
            self.expect(Tok::Int)?;
            // if let Tok::Int = self.tokens(1)[0] { // what if 'int'?
            //     self.consume(1);
            // } else {
            //     self.problem = Some(format!("Parsing Error: Expected int...").into());
            //     return None;
            // }
            if let Tok::Ident(ref mut id) = self.tokens(1)[0] { // what if 'int'?
                let arg = std::mem::take(id);
                self.consume(1);

                print!("{} ", String::from_utf8_lossy(&arg));
            } else {
                self.problem = Some(format!("Parsing Error: Expected int...").into());
                return None;
            }
            
            first = false;
            
        }

        // TODOTODO NEED TO CONTINUE WITH func
        self.block();
    }

    // block: LeftCurly stmts RightCurly

    // stmts: 
    //     | stmt stmts
    fn block(&mut self) -> Option<()> {{
        if let Tok::LeftCurly = self.tokens(1)[0]
                self.consume(1);
            },
            _ => {
                self.problem = Some(format!("Parsing Error: block").into());
                return None;
            },
        };


        loop {

            if let Tok::RightCurly = self.tokens(1)[0] { // what if ')'?
                self.consume(1);
                print!("\n"); // finished , newline
                break;
            }
            
            self.stmts()
            
        }
    }


// stmt: Int LeftBracket Num RightBracket Ident Semicolon
// | Int Ident Semicolon
// | Int Ident Assign exp Semicolon
// | Ident Assign exp Semicolon
// | Ident LeftBracket exp RightBracket Assign exp Semicolon
// | While bool_exp block Semicolon
// | If bool_exp block
// | If bool_exp block Else bool_exp block
// | Print LeftParen exp RightParen Semicolon
// | Read LeftParen Ident RightParen Semicolon
// | Read LeftParen Ident LeftBracket exp RightBracket RightParen Semicolon
// | Return Semicolon
// | Return exp Semicolon
// | Break Semicolon
// | Continue Semicolon

fn stmt(&mut self) -> Option<()> {
    match self.tokens(1)[0] {
        // &mut [Tok::Function] => {self.func()},
        &mut [Tok::Return] => {
            self.consume(1);
            self.expect(Tok::Semicolon)?;

        },
        &mut [Tok::Break] => {
            self.consume(1);
            self.expect(Tok::Semicolon)?;

        },
        &mut [Tok::Continue] => {
            self.consume(1);
            self.expect(Tok::Semicolon)?;

        },
        _ => {self.problem = Some(format!("Parsing Error: stmt").into()); None},
    }
}

// args:
// | exp
// | exp Comma args

// exp: exp Equality boolexp
// | exp NotEqual boolexp
// | boolexp

// boolexp: boolexp Less addexp
//    | boolexp LessEqual addexp
//    | boolexp Greater addexp
//    | boolexp GreaterEqual addexp
//    | addexp

// addexp: addexp Plus multexp
//   | addexp Substract multexp
//   | multexp

// multexp: multexp Multiply numexp
//    | multexp Divide numexp
//    | multexp Modulus numexp
//    | baseexp

// baseexp: Num
//    | Ident
//    | Ident LeftBracket exp RightBracket
//    | Ident LeftParen args RightBracket
//    | LeftParen exp RightParen







}


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
