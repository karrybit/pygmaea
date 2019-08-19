use crate::lexer::Lexer;
use std::io::Write;

const PROMPT: &'static str = ">>";
const EXIT_COMMAND: [&'static str; 3] = [":exit", ":quit", ":q"];

pub(crate) fn start() {
    loop {
        let mut command = String::new();
        print!("{} ", PROMPT);
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut command)
            .unwrap_or_else(|e| panic!("{}", e));

        if command.is_empty() {
            continue;
        }
        if EXIT_COMMAND.contains(&command.trim()) {
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
