use whoami;

mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

fn main() {
    let username = whoami::username();
    println!(
        "Hello {}! This is the Monkey programming language!",
        username
    );
    println!("Feel free to type in commands");
    repl::start();
}
