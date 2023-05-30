use std::io::Error;
use std::{fs::File, process::Stdio};

use crate::source::minishell::Shell;
use crate::source::parser::commands::{remove_dolar_by_env, ElementLine, ParseTypes};
use crate::source::redirections::heredoc::heredoc;

pub struct Pipe {
    pub pipe_in: Stdio,
    pub pipe_out: Stdio,
    pub value: String,
}

impl Pipe {
    pub fn new() -> Pipe {
        Pipe {
            pipe_in: Stdio::inherit(),
            pipe_out: Stdio::inherit(),
            value: String::new(),
        }
    }
    pub fn open_read(&mut self, path: &String) -> bool {
        let file = File::open(path);
        Pipe::verify_open(file, path, &mut self.pipe_in)
    }
    pub fn open_write(&mut self, path: &String) -> bool {
        let file = File::create(path);
        Pipe::verify_open(file, path, &mut self.pipe_out)
    }
    fn verify_open(file: Result<File, Error>, value: &String, pipe: &mut Stdio) -> bool {
        match file {
            Ok(f) => {
                *pipe = Stdio::from(f);
                false
            }
            Err(_) => {
                eprintln!("minishell: no such file or directory: {}", value);
                true
            }
        }
    }
}

fn execute_pipes(
    shell: &mut Shell,
    tokens: &Vec<ElementLine>,
    now: usize,
    mut error: bool,
    mut pipe: Pipe,
) -> (Pipe, bool, usize) {
    let last;
    if now < tokens.len() {
        let token_now = &tokens[now];
        if token_now.get_type() == &ParseTypes::Word {
            (pipe, error, last) = execute_pipes(shell, tokens, now + 1, error, pipe);
            if !error {
                pipe = token_now.execute(shell, pipe, last == tokens.len());
            }
            return (pipe, error, last);
        } else if token_now.get_type() == &ParseTypes::Redirection {
            let mut fi = tokens[now + 1]
                .value
                .trim()
                .trim_matches(' ')
                .trim_matches('\"')
                .to_string();
            if fi.find('\'') != Some(0) {
                fi = remove_dolar_by_env(fi, shell);
            }
            if token_now.value == ">" {
                error = pipe.open_write(&fi);
            } else if token_now.value == "<" {
                error = pipe.open_read(&fi);
            } else if token_now.value == ">>" {
                error = pipe.open_write(&fi);
            } else if token_now.value == "<<" {
                heredoc(&fi);
            }
            (pipe, error, last) = execute_pipes(shell, tokens, now + 2, error, pipe);
            if !error {
                return (pipe, error, last);
            }
        } else {
            return (pipe, error, now);
        }
    }
    (pipe, error, now)
}

pub fn execute(mut shell: Shell) -> Shell {
    let mut cmds = 0;
    let mut last = 0;
    let mut now;
    let mut pipe = Pipe::new();
    while cmds < shell.tokens.n_cmds {
        if cmds + 1 == shell.tokens.n_cmds {
            pipe.pipe_out = Stdio::inherit();
        } else {
            pipe.pipe_out = Stdio::piped();
        }
        let tokens = shell.tokens.tokens[(last)..].to_vec().clone();
        (pipe, _, now) = execute_pipes(&mut shell, &tokens, 0, false, pipe);
        last = last + now + 1;
        cmds += 1;
    }
    shell
}
