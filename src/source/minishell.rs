use std::{fs::File, io::{Read, self, Write}, env};
use super::parser::commands::ParsedHead;
use super::parser::parser;
use super::executor::execute;

pub struct Shell {
    pub line: String,
    pub env: super::env::new_env::Env,
    pub tokens: ParsedHead,
    pub error: i32,
    pub last_error: i32,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            line: String::new(),
            env: super::env::new_env::Env::new(),
            tokens: ParsedHead::new(),
            error: 0,
            last_error: 0,
        }
    }
    pub fn reset(&mut self) {
        self.line.clear();
        self.last_error = self.error;
        self.tokens = ParsedHead::new();
        self.error = 0;
    }
}


fn display_prompt() {
    let mut contents = String::new();
    match File::open("/etc/hostname") {
        Ok(mut file) => file.read_to_string(&mut contents).unwrap(),
        Err(e) => panic!("Error: hostname file not found : {}", e),
    };
    print!(
        "{}:{}$ ",
        contents.trim_end(),
        env::current_dir().unwrap().display()
    );
    io::stdout().flush().unwrap();
}

pub fn minishell() {
    let mut shell: Shell = Shell::new();

    loop {
        display_prompt();
        match io::stdin().read_line(&mut shell.line) {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e),
        };
        (shell.tokens, shell.error) = parser::parser(&shell.line);
        if shell.error != 0 {
            shell.reset();
            continue;
        }
        execute::execute(&shell.tokens);
        shell.reset();
    }
}
