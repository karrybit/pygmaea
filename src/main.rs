use whoami;

mod ast;
mod error;
mod lexer;
mod parser;
mod repl;
mod token;
mod token_type;

fn main() {
    let username = whoami::username();
    println!(
        "Hello {}! This is the Monkey programming language!",
        username
    );
    println!("Feel free to type in commands");
    repl::start();
}
