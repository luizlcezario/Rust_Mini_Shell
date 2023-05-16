use std::{fs::File, io::{Read, self, Write},  path::Path};
use super::parser::commands::ParsedHead;
use super::parser::parser;
use super::executor::execute;
use colored::Colorize;
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
    pub fn reset(&mut self){
        self.line.clear();
        self.last_error = self.error;
        self.tokens = ParsedHead::new();
        self.error = 0;
    }
}


fn display_prompt(shell: &Shell) {
    let user = shell.env.get_env("USER");
    match user {
        Some(u) => print!("{}:", u.green()),
        None =>{
            let mut contents = String::new();
            match File::open("/etc/hostname") {
                Ok(mut file) => file.read_to_string(&mut contents).unwrap(),
                Err(e) => panic!("Error: hostname file not found : {}", e),
            };
            print!("{}:", contents.trim_matches('\n').green());
        }
    };
    let user = shell.env.get_env("PWD");
    match user {
        Some(u) => print!("{}$ ", Path::new(u)
        .file_name()
        .expect("Failed to get last directory")
        .to_str()
        .expect("Failed to convert last directory to string").blue()),
        None => (),
    };
    
    io::stdout().flush().unwrap();
}

pub fn minishell() {
    let mut shell: Shell = Shell::new();

    loop {
        display_prompt(&shell);
        match io::stdin().read_line(&mut shell.line) {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e),
        };
        (shell.tokens, shell.error) = parser::parser(&shell.line);
        if shell.error != 0 {
            shell.reset();
            continue;
        }
        shell = execute::execute(shell);
        shell.reset();
    }
}
