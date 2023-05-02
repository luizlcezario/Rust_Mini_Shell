use std::{fs::File, io::{Read, self, Write}, env};
use super::parser::commands::ParsedHead;
use super::parser::parser;
use super::executor::execute;
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
    let mut a = String::new();
    let mut tokens: ParsedHead;
    let mut error: bool = false;
    loop {
        display_prompt();
        match io::stdin().read_line(&mut a) {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e),
        };
        (tokens, error) = parser::parser(&a);
        if error == true {
            a.clear();
            continue;
        }
        execute::execute(tokens);
        a.clear();
    }
}
