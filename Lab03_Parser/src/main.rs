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

    // stmts: 
    //     | stmt stmts
    fn stmts(&mut self) -> Option<()> {{
        self.stmt()?;
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
    match self.peek_many(8){
        &mut [Tok::Int, Tok::LeftBracket, Tok::Num(ref mut num), Tok::RightBracket, Tok::Ident(ref mut id), Tok::Semicolon, _, _] => {
            let name = std::mem::take(id);
            let size = std::mem::take(num)
            self.consume(6);
            name
        },

        &mut [Tok::Int, Tok::Ident(ref mut id), Tok::Semicolon, _, _, _, _,_] => {
            let name = std::mem::take(id);
            self.consume(3);
            name
        },

        &mut [Tok::Int, Tok::Ident(ref mut id), Tok::Assign, _, _, _, _,_] => {
            let name = std::mem::take(id);
            self.consume(3);
            name
            self.exp()?;
            if let Tok::Semicolon = self.tokens(1)[0]
                self.consume(1);
            },

        },

        &mut [Tok::Ident(ref mut id), Tok::Assign, _, _, _, _, _, _] => {
            let name = std::mem::take(id);
            self.consume(2);
            name
            self.exp()?;
            if let Tok::Semicolon = self.tokens(1)[0]
                self.consume(1);
            },

        },

        // | Ident LeftBracket exp RightBracket Assign exp Semicolon
        &mut [Tok::Ident(ref mut id), Tok::LeftBracket, _, _, _, _, _, _] => {
            let name = std::mem::take(id);
            self.consume(2);
            name
            self.exp()?;
            if let Tok::RightBracket = self.tokens(1)[0]
                self.consume(1);
            },
            if let Tok::Assign = self.tokens(1)[0]
                self.consume(1);
            },
            self.exp()?;
            if let Tok::Semicolon = self.tokens(1)[0]
                self.consume(1);
            },

        },

        // | While bool_exp block Semicolon
        &mut [Tok::While, _, _, _, _, _, _, _] => {
            // let name = std::mem::take(id);
            self.consume(1);
            // name
            self.bool_exp()?;
            self.block()?;
            if let Tok::Semicolon = self.tokens(1)[0]
                self.consume(1);
            },

        },
// | If bool_exp block
        &mut [Tok::If, _, _, _, _, _, _, _] => {
            // let name = std::mem::take(id);
            self.consume(1);
            // name
            self.bool_exp()?;
            self.block()?;
            // if let Tok::Semicolon = self.tokens(1)[0]
            //     self.consume(1);
            // },

        },
// | If bool_exp block Else bool_exp block
        &mut [Tok::If, _, _, _, _, _, _, _] => {
            // let name = std::mem::take(id);
            self.consume(1);
            // name
            self.bool_exp()?;
            self.block()?;
            if let Tok::Else = self.tokens(1)[0]
                self.consume(1);
            },
            self.bool_exp()?;
            self.block()?;


        },
// | Print LeftParen exp RightParen Semicolon
        &mut [Tok::Print,Tok::LeftParen, _, _, _, _, _, _] => {
            // let name = std::mem::take(id);
            self.consume(2);
            // name
            self.exp()?;
            // self.block()?;
            if let Tok::RightParen = self.tokens(1)[0]
                self.consume(1);
            },
            if let Tok::Semicolon = self.tokens(1)[0]
                self.consume(1);
            },


        },
// | Read LeftParen Ident RightParen Semicolon
        &mut [Tok::Read, Tok::LeftParen, Tok::Ident(ref mut id), Tok::RightParen, Tok::Semicolon, _, _, _] => {
            let name = std::mem::take(id);
            self.consume(5);
            name

        },
// | Read LeftParen Ident LeftBracket exp RightBracket RightParen Semicolon
        &mut [Tok::Read, Tok::LeftParen, Tok::Ident(ref mut id), Tok::LeftBracket, _, _, _, _] => {
            let name = std::mem::take(id);
            self.consume(4);
            name
            self.exp()?;
            if let Tok::RightParen = self.tokens(1)[0]
                self.consume(1);
            },
            if let Tok::Semicolon = self.tokens(1)[0]
                self.consume(1);
            },


        },


        &mut [Tok::Return, Tok::Semicolon, _, _, _, _, _, _] => {
            // let name = std::mem::take(id);
            self.consume(2);
            // name

        },

        // | Return exp Semicolon

        &mut [Tok::Return, _, _, _, _, _, _] => {
            // let name = std::mem::take(id);
            self.consume(1);
            // name
            self.exp()?;
            if let Tok::Semicolon = self.tokens(1)[0]
                self.consume(1);
            },

        },

        &mut [Tok::Break, Tok::Semicolon, _, _, _, _, _, _] => {
            // let name = std::mem::take(id);
            self.consume(2);
            // name

        },

        &mut [Tok::Continue, Tok::Semicolon, _, _, _, _, _, _] => {
            // let name = std::mem::take(id);
            self.consume(2);
            // name

        },

        _ => {
            self.problem = Some(format!("Parsing Error: stmt").into());
            return None;
        }
    }


///////
    // let name = match self.tokens(6) { 
    //     &mut [Tok::Int, Tok::LeftBracket, Tok::Num, Tok::RightBracket, Tok::Ident(ref mut id), Tok::Semicolon] => {
    //         let name = std::mem::take(id);
    //         self.consume(6);
    //         name
    //     },
    // };

    // let name = match self.tokens(3) { 
    //     &mut [Tok::Int, Tok::Ident(ref mut id), Tok::Semicolon] => {
    //         let name = std::mem::take(id);
    //         self.consume(3);
    //         name
    //     },
    // };

    // let name = match self.tokens(3) { 
    //     &mut [Tok::Int, Tok::Ident(ref mut id), Tok::Assign] => {
    //         let name = std::mem::take(id);
    //         self.consume(3);
    //         name
    //         self.exp()?;
    //         if let Tok::Semicolon = self.tokens(1)[0]
    //             self.consume(1);
    //         },

    //     },
    // };

    // let name = match self.tokens(2) { 
    //     &mut [Tok::Ident(ref mut id), Tok::Assign] => {
    //         let name = std::mem::take(id);
    //         self.consume(3);
    //         name
    //         self.exp()?;
    //         if let Tok::Semicolon = self.tokens(1)[0]
    //             self.consume(1);
    //         },

    //     },
    // };

    // let name = match self.tokens(1) { 
    //     &mut [Tok::Return] => {
    //         let name = std::mem::take(id);
    //         self.consume(1);
    //         name

    //     },
    // };

    // let name = match self.tokens(2) { 
    //     &mut [Tok::Return, Tok::Semicolon] => {
    //         let name = std::mem::take(id);
    //         self.consume(2);
    //         name

    //     },
    // };

    // let name = match self.tokens(2) { 
    //     &mut [Tok::Break, Tok::Semicolon] => {
    //         let name = std::mem::take(id);
    //         self.consume(2);
    //         name

    //     },
    // };

    // let name = match self.tokens(2) { 
    //     &mut [Tok::Continue, Tok::Semicolon] => {
    //         let name = std::mem::take(id);
    //         self.consume(2);
    //         name

    //     },
    // };



    // match self.tokens(1)[0] {
    //     // &mut [Tok::Function] => {self.func()},
    //     &mut [Tok::Return] => {
    //         self.consume(1);
    //         self.expect(Tok::Semicolon)?;

    //     },
    //     &mut [Tok::Break] => {
    //         self.consume(1);
    //         self.expect(Tok::Semicolon)?;

    //     },
    //     &mut [Tok::Continue] => {
    //         self.consume(1);
    //         self.expect(Tok::Semicolon)?;

    //     },
    //     _ => {self.problem = Some(format!("Parsing Error: stmt").into()); None},
    // }

// args:
// | exp
// | exp Comma args

fn args(&mut self) -> Option<Vec<u8>> {
    let mut args_list = Vec::New();
    Some(arg) = self.exp()?;
    // self.exp()?;
        loop {

            if let Tok::Comma = self.tokens(1)[0] { 
                self.consume(1);
                self.args()?;
                args_list.push_back(arg) //push args into args list
            },
            _ => {
                self.problem = Some(format!("Parsing Error: args").into());
                return None;
            },
            
            
        };

        _ => {
            self.problem = Some(format!("Parsing Error: args").into());
            return None;
        },


}

// exp: exp Equality boolexp
// | exp NotEqual boolexp
// | boolexp

fn exp(&mut self) -> Option<()> {
    self.boolexp()?;
    match self.tokens(1)[0] { 
        &mut [Tok::Equality] => {
            self.consume(1);
            self.boolexp()?;

        },
        &mut [Tok::NotEqual] => {
            self.consume(1);
            self.boolexp()?;

        },
        _ => {
            self.problem = Some(format!("Parsing Error: exp").into());
            return None;
        },
    };
}
// boolexp: boolexp Less addexp
//    | boolexp LessEqual addexp
//    | boolexp Greater addexp
//    | boolexp GreaterEqual addexp
//    | addexp
fn boolexp(&mut self) -> Option<()> {
    self.addexp()?;
    match self.tokens(1)[0] { 
        &mut [Tok::Less] => {
            self.consume(1);
            self.addexp()?;

        },
        &mut [Tok::LessEqual] => {
            self.consume(1);
            self.addexp()?;

        },
        &mut [Tok::Greater] => {
            self.consume(1);
            self.addexp()?;

        },
        &mut [Tok::GreaterEqual] => {
            self.consume(1);
            self.addexp()?;

        },
        _ => {
            self.problem = Some(format!("Parsing Error: boolexp").into());
            return None;
        },
    };
}

// addexp: addexp Plus multexp
//   | addexp Substract multexp
//   | multexp
fn addexpr(&mut self) -> Option<()> {
    let mut lhs = self.mulexpr()?;
    
    loop {
        match *lex{
            b'+' | b'-' => {
                let op = *lex;
                lex = lex.add(1); 
                let rhs = self.mulexpr()?;
                let temp = temp_name();
                
                // emit::bin_op(op, &temp, &lhs, &rhs); 
                // do print instead
                lhs = temp;
            },
            _ => return lhs,
        }
    }
}


// multexp: multexp Multiply numexp
//    | multexp Divide numexp
//    | multexp Modulus numexp
//    | baseexp
fn mulexpr(&mut self) -> Option<()> {
    let mut lhs = self.mulexpr()?;
    
    loop {
        match *lex{
            b'*' | b'/' | b'%' => {
                let op = *lex;
                // lex = lex.add(1); 
                let rhs = self.baseexp()?;
                let temp = temp_name();
                
                // emit::bin_op(op, &temp, &lhs, &rhs); 
                // do print instead
                lhs = temp;
            },
            _ => return lhs,
        }
    }
}

// baseexp: Num
//    | Ident
//    | Ident LeftBracket exp RightBracket
//    | Ident LeftParen args RightBracket
//    | LeftParen exp RightParen
fn baseexp(&mut self) -> Option<()> {
    match self.peek_many(4){
        &mut [Tok::Num(ref mut num) _, _,_] => {
            let size = std::mem::take(num)
            self.consume(1);
            size
        },

        &mut [Tok::Ident(ref mut id), _, _,_] => {
            let name = std::mem::take(id);
            // let size = std::mem::take(num)
            self.consume(1);
            name
        },

        &mut [Tok::Ident(ref mut id), Tok::LeftBracket, _, _] => {
            let name = std::mem::take(id);
            self.consume(2);
            name
            self.exp()?;
            if let Tok::RightBracket = self.tokens(1)[0]
                self.consume(1);
            },
        },

        &mut [Tok::Ident(ref mut id), Tok::LeftBracket, _, _] => {
            let name = std::mem::take(id);
            self.consume(2);
            name
            self.args()?;
            if let Tok::RightBracket = self.tokens(1)[0]
                self.consume(1);
            },
        },

        &mut [Tok::LeftParen,_ ,_, _] => {
            // let name = std::mem::take(id);
            self.consume(1);
            // name
            self.exp()?;
            if let Tok::RightParen = self.tokens(1)[0]
                self.consume(1);
            },
        },

    }




