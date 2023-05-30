use std::process::exit;

use crate::source::minishell::Shell;

pub fn built_exit(shell: &mut Shell, tokens: &mut Vec<String>) -> i32 {
    if tokens.len() > 2 {
        eprintln!("minishell: exit: too many arguments");
        return 1;
    }
    println!("exit");
    if tokens.len() == 2 {
        exit(tokens[1].parse::<i32>().unwrap());
    }
    exit(shell.error);
}
