#[derive(Debug)]

enum Tok {
    func, return, int, print, read, while, if,
    else, break, continue,
    "(", ")", "}", "{", "[", "]", ",", ";",
    "+", "-", "*", "/", "%", "=", "<", "<=",
    ">", ">=", "==", "!=",

}

//lab dfa
fn main() {
    let x = 3
    println!("{x}")
}