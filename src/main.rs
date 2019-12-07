use whoami;

mod repl;

fn main() {
    let username = whoami::username();
    println!(
        "Hello {}! This is the Monkey programming language!",
        username
    );
    println!("Feel free to type in commands");
    repl::start();
}
