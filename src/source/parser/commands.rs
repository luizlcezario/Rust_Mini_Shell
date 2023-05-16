use std::{
    collections::LinkedList,
    process::{Command, Stdio},
};

use crate::source::{builtins::{cd::built_cd, env::built_env}, minishell::Shell};


#[derive(PartialEq, Clone)]
pub enum ParseTypes {
    Word,
    Pipe,
    Redirection,
    End,
}
#[derive(Clone)]
pub struct ElementLine {
    pub parse_type: ParseTypes,
    pub value: String,
}

impl PartialEq for ElementLine {
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _
    }
}

impl ElementLine {
    pub fn new() -> ElementLine {
        ElementLine {
            parse_type: ParseTypes::End,
            value: String::new(),
        }
    }
    pub fn select_type(&mut self, value: &String) {
        if value == "|" {
            self.parse_type = ParseTypes::Pipe;
        } else if value == ">" || value == "<" || value == ">>" || value == "<<" {
            self.parse_type = ParseTypes::Redirection;
        } else {
            self.parse_type = ParseTypes::Word;
        }
    }
    pub fn add_value(&mut self, value: String) {
        self.value.push_str(&value);
    }
    pub fn get_value(&self) -> &String {
        &self.value
    }
    pub fn get_type(&self) -> &ParseTypes {
        &self.parse_type
    }
    pub fn is_builtin(&self, shell: & mut Shell, cmd: &str, splitted: & mut Vec<&str>) -> u32 {
        if cmd == "cd" {
            built_cd(shell, splitted);
            return 1;
        } else if cmd == "env" {
            built_env();
            return 1;
        } else if cmd == "export" {
            return 1;
        } else if cmd == "pwd" {
            return 1;
        } else if cmd == "unset" {
            return 1;
        } else if cmd == "exit" {
            return 1;
        } else {
            return 0;
        }
    }
    pub fn execute(&self, shell: & mut Shell,  pipe_in: Stdio, pipe_out: Stdio, last:bool, ) -> Stdio {
        let mut splitted = self.value.split(" ").collect::<Vec<&str>>();
        let sed_child;
        if self.is_builtin(shell, &splitted[0], & mut splitted) == 1 {
            if last {
                return Stdio::null();
            }
            return pipe_out;
        }
        if splitted[1..].concat() != "" {
            sed_child = Command::new(splitted[0])
                .arg(splitted[1..].concat())
                .env_clear()
                .envs(shell.env.get_all())
                .stdin(pipe_in)
                .stdout(pipe_out)
                .spawn()
                .expect("error on spawn");
        } else {
            sed_child = Command::new(splitted[0])
                .stdin(pipe_in)
                .env_clear()
                .envs(shell.env.get_all())
                .stdout(pipe_out)
                .spawn()
                .expect("error on spawn");
        } 
        if last {
            (sed_child.wait_with_output().expect("error on wait"));
            return Stdio::null();
        }
        return sed_child.stdout.expect("error on stdout").into();
    }
}

#[derive(Clone)]
pub struct ParsedHead {
    pub n_cmds: i32,
    pub tokens: LinkedList<ElementLine>,
}

impl ParsedHead {
    pub fn add_token(&mut self, cmd: ElementLine) {
        self.tokens.push_back(cmd);
        self.n_cmds += 1;
    }

    pub fn new() -> ParsedHead {
        ParsedHead {
            n_cmds: 0,
            tokens: LinkedList::new(),
        }
    }
    pub fn get_all_until_next(&mut self, now: &ElementLine) -> LinkedList<&ElementLine> {
        let mut list = LinkedList::new();
        let mut now_pos = false;
        for token in self.tokens.iter() {
            if token == now {
                now_pos = true;
            } else if now_pos && token.get_type() == &ParseTypes::Pipe {
                break;
            } else if now_pos && token.get_type() == &ParseTypes::End {
                break;
            } else if now_pos {
                list.push_back(token);
            }
        }
        return list;
    }
}
