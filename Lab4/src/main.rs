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
use std::collections::HashMap;
// use std::env;

#[derive(PartialEq)]
enum Type {
    Var,
    Arr,
    Fn,

}

struct Par {
    lex: Lex, // generate token
    toks: SliceDeque<Tok>, // buffers tokens
    problem: Option<Box<dyn Error>>,
    // temp names --V
    t_count: usize,
    // l_count: usize,
    top_stack: Vec<String>,
    bot_stack: Vec<String>,

    types: Vec<HashMap<String, Type>>,
}

impl Par {
    fn make(file_path: &str) -> Result<Par, Box<dyn Error>> {
        let types = Vec::new();

        types.push(HashMap::new());

        Ok(Par{
            lex: Lex::make(file_path)?, toks: SliceDeque::new(), problem:None,
            t_count: 0, // l_count: 0,
            top_stack: Vec::new(), bot_stack: Vec::new(),
            types,
        })
    }
    fn tokens(&mut self, amt: usize) -> &mut [Tok] { // buffers token

        while self.toks.len() < amt {self.toks.push_back(self.lex.next());}

        &mut self.toks[0..amt]
    }

    fn consume (&mut self, amt: usize) {for _ in 0..amt {self.toks.pop_front();}} // reduce/consume tokens

    fn temp_name(&mut self) -> Vec<u8> { // deals with naming temporary values
        let mut res = Vec::from(b"temp");
        res.extend_from_slice(&self.t_count.to_string().into_bytes());
        self.t_count += 1;
        res
    }

    fn type_check(&mut self, i: usize, name: &Vec<u8>, check_type: Type) -> Option<()> {
        // fn type_check(&mut self, i: usize, name: Vec<u8>, check_type: Type) -> Option<()> {
        if 0 == i{
            self.problem = Some(format!("semantic errors!").into());
            return None;
        }

        let i = i - 1;

        if let Some(symbol) = self.types[i].get(name){
            if *symbol == check_type { 
                return Some(());
            }
            else { 
                self.problem = Some(format!("semantic errors!").into());
                return None; 
            }
        }
        else { self.type_check(i, name, check_type)}
    }

    // what do we need this for?????
    // fn temp_label(&mut self) -> Vec<u8> { // helps label temporary names
    //     let mut res = Vec::from(b"label");
    //     res.extend_from_slice(&self.l_count.to_string().into_bytes());
    //     self.l_count += 1;
    //     res
    // }
    // TODO: Grammar stuff starts here ------------------------------------------------------------------------------
    // prog:
    // | func prog
    fn parse(&mut self) -> Option<()> {
        self.types.push(HashMap::new());

        
        loop {
            match self.tokens(1) {
                &mut [Tok::Function] => {
                    self.func()?; 
                    // return None;
                },
                &mut [Tok::Empty] => {
                    // return None;
                },
                _ => {
                    self.problem = Some(format!("Parsing Error: program").into()); 
                    // return None;
                },
            }

        }
        if let Some(Type::Fn) = self.types[0].get(&Vec::from("main").as_bytes()){
            // break Ok(());
            return Some(());
        }else{
            self.problem = Some(format!("err").into());
            return None;
        }
        // if let Some(Type::Fn) = self.types[0].get(&Vec::from("main").as_bytes()){
        //     // break Ok(());
        //     return Some(());
        // }else{
        //     self.problem = Some(format!("err").into());
        //     return None;
        // }

        self.types.pop()
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
    // Bin_op does binary operators
    fn bin_op (&mut self, op:Tok, lhs:Vec<u8>, rhs:Vec<u8>) -> Option<Vec<u8>> {
        let dst = self.temp_name();
        let ope;
        match op {
            Tok::Multiply => {ope = Vec::from(b"*");},
            Tok::Divide => {ope = Vec::from(b"/");},
            Tok::Modulus => {ope = Vec::from(b"%");},
            Tok::Plus => {ope = Vec::from(b"+");},
            Tok::Substract => {ope = Vec::from(b"-");},
            Tok::Less => {ope = Vec::from(b"<");},
            Tok::Greater => {ope = Vec::from(b">");},
            Tok::LessEqual => {ope = Vec::from(b"<=");},
            Tok::GreaterEqual => {ope = Vec::from(b">=");},
            Tok::Equality => {ope = Vec::from(b"==");},
            Tok::NotEqual => {ope = Vec::from(b"!=");},
            _ => {self.problem = Some(format!("Binary Operation Error").into()); return None},
        };

        ////println!("{} = {} {} {}", String::from_utf8_lossy(&dst), String::from_utf8_lossy(&lhs),String::from_utf8_lossy(&ope), String::from_utf8_lossy(&rhs));
        println!("{} {}, {}, {}", String::from_utf8_lossy(&dst), String::from_utf8_lossy(&lhs),String::from_utf8_lossy(&ope), String::from_utf8_lossy(&rhs));
        Some(dst)
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
        
        //print!("function header: {} ", String::from_utf8_lossy(&name)); // output function header
        print!("func {} (", String::from_utf8_lossy(&name));
        if let Some(_already_present) = self.types[0].insert(name.clone(), Type::Fn){
            self.problem = Some(format!("func nname dup").into());
            return None;
        }

        

        let mut first = true;
        loop {

            if let Tok::RightParen = self.tokens(1)[0] { // what if ')'?
                self.consume(1);
                //print!("\n"); // finished reading params, newline
                print!(")\n");
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

                //print!(",{} ", String::from_utf8_lossy(&arg));
                print!("%int {} ", String::from_utf8_lossy(&arg));
            } else {
                self.problem = Some(format!("Parsing Error: Expected int...").into());
                return None;
            }
            
            first = false;
            
        }

        // continue with statements block
        self.stmts()
    }


    // fn block(&mut self, params: Vec<String>) -> bool {
    //     self.types.push(HashMap::new());
    //         for param_list in params {
    //             if let Some(_already_present) = self.types.last_mut().unwrap().insert((*param_list).to_string(), Type::Var){
    //                 self.problem = Some(format!("duplicate parameter name").into());
    //                 return None;
    //             }
    //         }
    // }

    // block: LeftCurly stmts RightCurly
    // stmts: 
    //      | stmt stmts
    fn stmts(&mut self) -> Option<()> {
        self.types.push(HashMap::new());
        match self.tokens(1) {
            &mut [Tok::LeftCurly] => {
                self.consume(1);
                //println!("{{");
            },
            _ => {self.problem = Some(format!("Parsing Error: Expected statements block").into()); return None;},
        }
        
        loop {
            if let Tok::RightCurly = self.tokens(1)[0] {
                self.consume(1);
                //println!("}}\n");
                break Some(());
            }
            //if it wasn't a '}'... well then it's something else
            self.stmt()?;
        }
        self.types.pop()
    }

    // stmt: Int LeftBracket Num RightBracket Ident Semicolon //DONE
    //     | Int Ident Semicolon //DONE
    //     | Int Ident Assign exp Semicolon //DONE-NEED fn exp()
    //     | Ident Assign exp Semicolon //DONE-NEED fn exp()
    //     | Ident LeftBracket exp RightBracket Assign exp Semicolon //DONE-NEED fn exp()
    //     | While bool_exp block //DONE-NEED fn exp()
    //     | If bool_exp block
    //     | If bool_exp block Else bool_exp block
    //     | Print LeftParen exp RightParen Semicolon //DONE-NEED fn exp()
    //     | Read LeftParen Ident RightParen Semicolon //DONE
    //     | Read LeftParen Ident LeftBracket exp RightBracket RightParen Semicolon //DONE-NEED fn exp()
    //     | Return Semicolon //DONE
    //     | Return exp Semicolon //DONE-NEED fn exp()
    //     | Break Semicolon //DONE
    //     | Continue Semicolon //DONE
    
    // the possible derivations are not ordered in fn stmt...
    fn stmt(&mut self) -> Option<()> {
        match self.tokens(6) { // we're looking at 6 tokens in advance
            
            // While bool_exp block
            &mut[Tok::While, _, _, _, _, _, ] => {
                self.consume(1);
                //print!("while (");
                if let Some(cond) = self.bool_exp() {
                    //println!("cond({}))", String::from_utf8_lossy(&cond));
                    self.stmts()
                } else { None }
            }

            &mut [Tok::Ident(ref mut id), Tok::Assign, _, _, _, _, ] => {
                let name = std::mem::take(id);
                self.consume(2);

                if let Some(rhs)= self.exp() {
                    //println!("assign var: {} = {}", String::from_utf8_lossy(&name), String::from_utf8_lossy(&rhs));
                    println!("%mov {} , {}", String::from_utf8_lossy(&name), String::from_utf8_lossy(&rhs));
                    self.expect(Tok::Semicolon)?; // MIGHT CAUSE PROBLEM, KEEP AN EYE HERE
                    Some(())
                } else { None }

                self.type_check(self.types.len(), &name, Type::Var)?;
                // if !self.type_check(self.types.len(), &name, Type::Var) {
                //     panic!("Assign to undeclared var");
                // }
            }

            // Ident LeftBracket exp RightBracket Assign exp Semicolon
            &mut [Tok::Ident(ref mut id), Tok::LeftBracket , _, _, _, _,] => {
                let name = std::mem::take(id);
                self.consume(2);

                // if !self.type_check(self.types.len(), &name, Type::Arr) {
                //     panic!("Assign to undeclared arr");
                // }
                self.type_check(self.types.len(), &name, Type::Arr)?;

                if let Some(index)= self.exp() { // NESTED IF KEEP AN EYE
                    self.expect(Tok::RightBracket)?; // MIGHT CAUSE PROBLEM, KEEP AN EYE HERE
                    self.expect(Tok::Assign)?;

                    if let Some(rhs)= self.exp() {
                        //println!("assign arr: {}[{}] = {}", String::from_utf8_lossy(&name), String::from_utf8_lossy(&index), String::from_utf8_lossy(&rhs));
                        println!("%mov [{} + {}], {}",String::from_utf8_lossy(&name), String::from_utf8_lossy(&index), String::from_utf8_lossy(&rhs));
                        self.expect(Tok::Semicolon);
                        Some(())
                    } else {None}
                } else { None }

                // if !self.type_check(self.types.len(), &name, Type::Fn) {
                //     panic!("Attempted use of non existant function {}", &name);
                // }
                self.type_check(self.types.len(), &name, Type::Fn)?;

            }

            &mut [Tok::Int, Tok::Ident(ref mut id), Tok::Assign, _, _, _, ] => {
                let name = std::mem::take(id);
                self.consume(3);

                println!("%int {}", String::from_utf8_lossy(&name)); 
                if let Some(_already_present) = self.types.last_mut().unwrap().insert(name.clone(), Type::Var) {
                    panic!("variable name clash")
                }

                if let Some(rhs)= self.exp() {
                    //println!("declare - assign var: {} = {}", String::from_utf8_lossy(&name), String::from_utf8_lossy(&rhs));
                        
                    
                    

                    println!("%mov {}, {}", String::from_utf8_lossy(&name), String::from_utf8_lossy(&rhs));
                    self.expect(Tok::Semicolon)?; // MIGHT CAUSE PROBLEM, KEEP AN EYE HERE
                    Some(())
                } else { None }
            }

            &mut [Tok::Int, Tok::LeftBracket, Tok::Num(ref mut num), Tok::RightBracket, Tok::Ident(ref mut id), Tok::Semicolon] => {
                let length = std::mem::take(num);
                let name = std::mem::take(id);
                self.consume(6);
                //println!("declare array: {}, {}", String::from_utf8_lossy(&name), String::from_utf8_lossy(&length));
                println!("%int[] {}, {}", String::from_utf8_lossy(&name));
                if let Some(_already_present) = self.types.last_mut().unwrap().insert(name.clone(), Type::Arr) {
                    panic!("array name clash")
                }
                Some(())
            }

            &mut [Tok::Int, Tok::Ident(ref mut id), Tok::Semicolon, _, _, _, ] => {
                let name = std::mem::take(id);
                self.consume(3);
                //println!("declare var: {}", String::from_utf8_lossy(&name));
                println!("%int {}", String::from_utf8_lossy(&name));
                if let Some(_already_present) = self.types.last_mut().unwrap().insert(name.clone(), Type::Var) {
                    panic!("variable name clash")
                }
                Some(())
            }

            // Print LeftParen exp RightParen Semicolon
            &mut [Tok::Print, Tok::LeftParen, _, _, _, _, ] => {
                self.consume(2);

                if let Some(value)= self.exp() {
                    //println!("print: {}", String::from_utf8_lossy(&value));
                    println!("%out {}", String::from_utf8_lossy(&value));
                    self.expect(Tok::RightParen)?;
                    self.expect(Tok::Semicolon)?; // MIGHT CAUSE PROBLEM, KEEP AN EYE HERE
                    Some(())
                } else { None }

            }

            // Read LeftParen Ident RightParen Semicolon
            &mut [Tok::Read, Tok::LeftParen, Tok::Ident(ref mut id), Tok::RightParen, Tok::Semicolon, _,] => {
                let name = std::mem::take(id);
                self.consume(5);
                //println!("read: {}", String::from_utf8_lossy(&name));
                println!("%input {}", String::from_utf8_lossy(&name));
                Some(())
            }

            // Read LeftParen Ident LeftBracket exp RightBracket RightParen Semicolon
            &mut [Tok::Read, Tok::LeftParen, Tok::Ident(ref mut id), Tok::LeftBracket, _, _,] => {
                let name = std::mem::take(id);
                self.consume(4);

                if let Some(index)= self.exp() {
                    //println!("read: {}[{}]", String::from_utf8_lossy(&name), String::from_utf8_lossy(&index));
                    println!("%input [{} + {}]",String::from_utf8_lossy(&name), String::from_utf8_lossy(&index));
                    self.expect(Tok::RightBracket)?;
                    self.expect(Tok::RightParen)?;
                    self.expect(Tok::Semicolon)?; // MIGHT CAUSE PROBLEM, KEEP AN EYE HERE
                    Some(())
                } else { None }
            }

            
            &mut [Tok::Return, Tok::Semicolon, _, _, _, _, ] => {
                self.consume(2);
                //println!("return");
                println!("return");
                Some(())
            }
            // Return exp Semicolon
            // slides say ordering matter, so we can put the more ambiguous one at the bottom of the specific derivation then
            &mut [Tok::Return, _, _, _, _, _, ] => {
                self.consume(1);
                if let Some(value) = self.exp() {
                    //println!("return: {}", String::from_utf8_lossy(&value));
                    println!("%ret {}", String::from_utf8_lossy(&value));
                    self.expect(Tok::Semicolon)?;
                    Some(())
                } else { None }
            }

            
            &mut [Tok::Break, Tok::Semicolon, _, _, _, _, ] => {
                self.consume(2);
                //println!("break");
                println!("break");
                Some(())
            }

            &mut [Tok::Continue, Tok::Semicolon, _, _, _, _, ] => {
                self.consume(2);
                //println!("continue");
                println!("continue");
                Some(())
            }

            // if is special, since one derivation is ambiguous so we just make a function for it
            &mut[Tok::If, _, _, _, _, _, ] => {
                self.if_block()
            }

            _ => {self.problem = Some(format!("Parsing error: statement formatting").into()); None}
        }
    }

    // If bool_exp block
    // If bool_exp block Else bool_exp block
    fn if_block(&mut self) -> Option<()> {
        self.consume(1);
        //print!("if(");

        if let Some(cond) = self.bool_exp() {
            //println!("cond({})", String::from_utf8_lossy(&cond));

            self.stmts()?;

            match self.tokens(1) {
                &mut [Tok::Else] => {
                    self.consume(1);
                    //println!("else");
                    self.stmts()?;
                },
                _ => {},
            }

            Some(())
        }
        else { None }
    }

    // baseexp: Num //DONE
    //        | Ident //DONE
    //        | Ident LeftBracket exp RightBracket //DONE-NEED fn exp()
    //        | Ident LeftParen args RightBracket //DONE-NEED fn func_call
    //        | LeftParen exp RightParen //DONE-NEED fn exp()
    fn base_exp(&mut self) -> Option<Vec<u8>> {
        match self.tokens(2) {

            &mut[Tok::LeftParen, _,] => {
                self.consume(1);
                let exp = self.exp()?;
                if let Tok::RightParen = self.tokens(1)[0] {
                    self.consume(1);
                    Some(exp)
                } else {
                    self.problem = Some(format!("(exp) format failed").into());
                    None
                }
            }

            &mut[Tok::Num(ref mut num), _, ] => {
                let number = std::mem::take(num);
                self.consume(1);
                Some(number)
            },

            &mut[Tok::Ident(ref mut id), Tok::LeftBracket] => {
                let name = std::mem::take(id);
                self.consume(2);
                let index = self.exp()?;
                if let Tok::RightBracket = self.tokens(1)[0] {
                    let dst = self.temp_name();
                    self.consume(1);
                    //println!("{} = {}[{}]", String::from_utf8_lossy(&dst), String::from_utf8_lossy(&name), String::from_utf8_lossy(&index));
                    println!("%mov {}, [{} + {}]",String::from_utf8_lossy(&dst), String::from_utf8_lossy(&name), String::from_utf8_lossy(&index));
                    Some(dst)
                } else {
                    self.problem = Some(format!("array format failed").into());
                    None
                }
                
            },

            &mut[Tok::Ident(ref mut id), Tok::LeftParen] => {
                let fn_name = std::mem::take(id);
                self.consume(2);
                self.fn_call(fn_name)
            }

            &mut[Tok::Ident(ref mut id), _,] => {
                let name = std::mem::take(id);

                // if !self.type_check(self.types.len(), &name, Type::Fn) {
                //     panic!("Attempted use of non existant function {}", &name);
                // }
                self.type_check(self.types.len(), &name, Type::Fn)?;

                self.consume(1);
                Some(name)
            },

            _ => {self.problem = Some(format!("expression error").into()); None}
        }
    }

    // multexp: multexp Multiply baseexp
    //        | multexp Divide baseexp
    //        | multexp Modulus baseexp
    //        | baseexp
    fn mult_exp(&mut self) -> Option<Vec<u8>> {
        let mut lhs_val = self.base_exp()?;
        
        loop {
            if let Tok::Multiply = self.tokens(1)[0] {
                self.consume(1);
                // let op = Tok::Multiply;
                let rhs_val = self.base_exp()?;
                // call lhs = binOP
                lhs_val = self.bin_op(Tok::Multiply, lhs_val, rhs_val)?;
            }
            else if let Tok::Divide = self.tokens(1)[0] {
                self.consume(1);
                // let op = Tok::Divide;
                let rhs_val = self.base_exp()?;
                // call lhs = binOP
                lhs_val = self.bin_op(Tok::Divide, lhs_val, rhs_val)?;
            }
            else if let Tok::Modulus = self.tokens(1)[0] {
                self.consume(1);
                // let op = Tok::Modulus;
                let rhs_val = self.base_exp()?;
                // call lhs = binOP
                lhs_val = self.bin_op(Tok::Modulus, lhs_val, rhs_val)?;
                
            } else {
                break Some(lhs_val);
            }
        }
    }

    // addexp: addexp Plus multexp
    //       | addexp Substract multexp
    //       | multexp
    fn add_exp(&mut self) -> Option<Vec<u8>> {
        let mut lhs_val = self.mult_exp()?;
        
        loop {
            if let Tok::Plus = self.tokens(1)[0] {
                self.consume(1);
                // let op = Tok::Multiply;
                let rhs_val = self.mult_exp()?;
                // call lhs = binOP
                lhs_val = self.bin_op(Tok::Plus, lhs_val, rhs_val)?;
            } else if let Tok::Substract = self.tokens(1)[0] {
                self.consume(1);
                // let op = Tok::Divide;
                let rhs_val = self.mult_exp()?;
                // call lhs = binOP
                lhs_val = self.bin_op(Tok::Substract, lhs_val, rhs_val)?;
            } else {
                break Some(lhs_val);
            }
        }
    }

    // boolexp: boolexp Less addexp
    //        | boolexp LessEqual addexp
    //        | boolexp Greater addexp
    //        | boolexp GreaterEqual addexp
    //        | addexp
    fn bool_exp(&mut self) -> Option<Vec<u8>> {
        let mut lhs_val = self.add_exp()?;
        
        loop {
            if let Tok::Less = self.tokens(1)[0] {
                self.consume(1);
                // let op = Tok::Multiply;
                let rhs_val = self.add_exp()?;
                // call lhs = binOP
                lhs_val = self.bin_op(Tok::Less, lhs_val, rhs_val)?;
            } else if let Tok::Greater = self.tokens(1)[0] {
                self.consume(1);
                // let op = Tok::Divide;
                let rhs_val = self.add_exp()?;
                // call lhs = binOP
                lhs_val = self.bin_op(Tok::Greater, lhs_val, rhs_val)?;
            } else if let Tok::LessEqual = self.tokens(1)[0] {
                self.consume(1);
                // let op = Tok::Divide;
                let rhs_val = self.add_exp()?;
                // call lhs = binOP
                lhs_val = self.bin_op(Tok::LessEqual, lhs_val, rhs_val)?;
            } else if let Tok::GreaterEqual = self.tokens(1)[0] {
                self.consume(1);
                // let op = Tok::Divide;
                let rhs_val = self.add_exp()?;
                // call lhs = binOP
                lhs_val = self.bin_op(Tok::GreaterEqual, lhs_val, rhs_val)?;
            }  else {
                break Some(lhs_val);
            }
        }
    }

    // exp: exp Equality boolexp
    //    | exp NotEqual boolexp
    //    | boolexp
    fn exp(&mut self) -> Option<Vec<u8>> {
        let mut lhs_val = self.bool_exp()?;
        
        loop {
            if let Tok::Equality = self.tokens(1)[0] {
                self.consume(1);
                // let op = Tok::Multiply;
                let rhs_val = self.bool_exp()?;
                // call lhs = binOP
                lhs_val = self.bin_op(Tok::Equality, lhs_val, rhs_val)?;
            } else if let Tok::NotEqual = self.tokens(1)[0] {
                self.consume(1);
                // let op = Tok::Divide;
                let rhs_val = self.bool_exp()?;
                // call lhs = binOP
                lhs_val = self.bin_op(Tok::NotEqual, lhs_val, rhs_val)?;
            } else {
                break Some(lhs_val);
            }
        }
    }


    // base_exp: Ident LeftParen args RightBracket
    // args:
    //     | exp
    //     | exp Comma args
    fn fn_call(&mut self, fn_name: Vec<u8>) -> Option<Vec<u8>> {
        let mut first = true;
        // take in all function call arguments
        let mut arguments = Vec::new();
        loop {
            if let Tok::RightParen = self.tokens(1)[0] { // what if ')'?
                self.consume(1);
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
            let argu = self.exp()?;
            arguments.push(argu);
            
            first = false;
        }
        let dst = self.temp_name();
        
        //print! ("call: {} = {}(", String::from_utf8_lossy(&dst), String::from_utf8_lossy(&fn_name));
        print! ("%call {}, {}(", String::from_utf8_lossy(&dst), String::from_utf8_lossy(&fn_name));
        let mut argufirst = true;
        while !arguments.is_empty() {
            if !argufirst {print!(",");}
            //print!("{}", String::from_utf8_lossy(&arguments.remove(0)));
            print!("{}", String::from_utf8_lossy(&arguments.remove(0)));
            argufirst = false
        }
        //println!(")");
        println!(")");
        Some(dst)

        // let dst = self.temp_name();
        // //print! ("call: {} = {}(", String::from_utf8_lossy(&dst), String::from_utf8_lossy(&fn_name));

        // let mut first = true;
        // loop {
        //     if let Tok::RightParen = self.tokens(1)[0] { // what if ')'?
        //         self.consume(1);
        //         break;
        //     }
        //     if !first{

        //         self.expect(Tok::Comma)?; // remember '?'
        //         //print!(", ");
        //         // if let Tok::Comma = self.tokens(1)[0] { // what if 'int'?
        //         //     self.consume(1);
        //         // } else {
        //         //     self.problem = Some(format!("Parsing Error: Expected Comma...").into());
        //         //     return None;
        //         // }
        //     }
        //     let argu = self.exp()?;
        //     //print!("{}", String::from_utf8_lossy(&argu));
            
        //     first = false;

            
        // }
        // //println!("))");
    }


    

    

}

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
    Empty, // signals parser that no token would've been here
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

    fn next(&mut self) -> Tok { // NEW: We need this so that parser can call a function that returns a token
        if let Some(tok) = self.lex() {tok}
        else {Tok::Empty}
    }

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

            b'A' ..=b'Z' | b'a' ..=b'z' | b'_' => {self.lex_id()},
            b'0' ..=b'9' => {return self.lex_num();},
            b'#' => {self.lex_com(); self.lex()},


            ch => {self.problem = Some(format!("Lexer: found an invalid char {}", ch).into()); None}
        }
    }

    fn lex_id (&mut self) -> Option<Tok> {
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

    fn lex_num (&mut self) -> Option<Tok> {
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

    fn lex_com (&mut self) {
        // just consume all comment line, "lexer must NOT return comment tokens"
        while let Some(byte) = self.it.next() {
            match byte {b'\n' => {break}, _=> {continue},}
        }
    }

}
fn main() -> Result<(), Box<dyn Error>> {
    // env::set_var("RUST_BACKTRACE", "full");

    let args : Vec<String> = std::env::args().collect();
    let mut par = Par::make(&args[1])?;

    par.parse();


    Ok(())

    // let args : Vec<String> = std::env::args().collect();
    // let mut lex = Lex::make(&args[1])?;

    // while let Some(tok) = lex.lex() {
    //     match tok {
    //         Tok::Ident(vec) | Tok::Num(vec) => {//print!("{}, ", String::from_utf8_lossy(&vec[..]));},
    //         _ => {//print!("{:?}, ", tok);},
    //     }
    // }

    // //println!("");

    // if let Some(err) = lex.problem {
    //     //println!("Problem, line {}: {}", lex.line, err);
    //     return Err(err);
    // }

    // Ok(())
}
