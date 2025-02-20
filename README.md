# Compiler System

# Project Overview

This project is developing a compiler using Rust for an experimental programming language. The developed compiler is be able to compile programs written in this experimental language into a low-level assembly-like format, which can then be executed by an interpreter.  

# Four phases/milestones:  
(i) development of the tokenizer: (Lexical Analysis) – Converts the input source code into a sequence of tokens  

(ii) development of the parser: (Syntax Analysis) – Constructs an Abstract Syntax Tree (AST) to validate and represent program structure   

(iii) development of the code generator (part I): - Generates an initial low-level intermediate representation (IR)   

(iv) development of the code generator (part II) - Optimizes and finalizes the generated IR for efficient execution   

# Installation  

**Install Rust:**   
Run the following command to install Rust :  
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Verify:**
```sh
rustc --version
```
```sh
cargo --version
```



**Instructions to Compile:**  
**Clone the Repository:**   
```sh
https://github.com/ylian121/compiler.git
```
```sh
cd compiler
```

**Compile and Run the Project:**   
```sh
cargo run
```
