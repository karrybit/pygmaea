use crate::lexer::Lexer;
use std::io::Write;

pub(crate) fn start() {
    loop {
        let mut command = String::new();
        print!(">> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut command)
            .unwrap_or_else(|e| panic!("{}", e));

        if command.is_empty() {
            continue;
        }
        if [":exit", ":quit", ":q"].contains(&command.trim()) {
            break;
        }

        let mut lexer = Lexer::new(command);
        let mut token = lexer.next_token();
        while !token.token_type.is_eof() {
            println!("{}", token);
            token = lexer.next_token();
        }
    }
}
