use std::{
    process::{Command, Stdio},
};

use regex::{Regex, Error};

use crate::source::{builtins::{cd::built_cd, env::built_env, pwd::built_pwd, exit::built_exit, export::built_export, unset::built_unset}, minishell::Shell, executor::execute::Pipe};

use super::parser::validade_quote;


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
pub fn remove_dolar_by_env(mut word: String, shell: &Shell ) -> String {
    while let Some(a) = word.find('$') {
        let mut has_special = false;
        let mut var = String::new();
        let f = word[a..].find(' ');
        if f.is_some() {
            var = word[a..(a + f.unwrap())].to_string();
        } else {
            let special_chars_regex = Regex::new(r"[!#%^&*()+={}\[\]|:;\\<>,?/]").unwrap();
            if special_chars_regex.is_match(&word[(a + 1)..]){
                has_special = true
            } else {
                var = word[(a + 1)..].to_string();
            }
        }
        if has_special && word[a..a + 2].contains("$?") {
            word.replace_range(a..a + 2, &shell.last_error.to_string());
        } else {
            let value = shell.env.get_env(&var);
            match value {
                Some(value) => word.replace_range(a..(a + var.len()), &value),
                None => word.replace_range(a..=(a + var.len()), ""),
            }
        }
    }
    return word;
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
    pub fn is_builtin(&self, shell: & mut Shell, cmd: String, splitted: & mut Vec<String>) -> i32 {
        if cmd.eq("cd") {
            shell.error = built_cd(shell, splitted);
            return 1;
        } else if cmd.eq("env") {
            shell.error = built_env(shell, splitted);
            return shell.error;
        } else if cmd.eq("export") {
            shell.error = built_export(shell, splitted);
            return 1;
        } else if cmd.eq("pwd") {
            shell.error = built_pwd(shell, splitted);
            return 1;
        } else if cmd.eq("unset") {
            shell.error = built_unset(shell, splitted);
            return 1;
        } else if cmd.eq("exit") {
            shell.error = built_exit(shell, splitted);
            return 1;
        } else {
            return 0;
        }
    }
            
    pub fn split_string(&self, shell: &Shell) -> Vec<String> {
        let mut splitted: Vec<String> = Vec::new();
        let mut i = 0;
        let mut word = String::new();
        while i < self.value.len() {
            if self.value.chars().nth(i).unwrap() == '\"' || self.value.chars().nth(i).unwrap() == '\'' {
                let (pos, _) = validade_quote(&self.value, &i);
                word.push_str(
                    self.value.get((i + 1)..=(i + pos ))
                        .expect("minishell: syntax error near unexpected token `newline'"),
                );
                i += pos + 2;
                if self.value.chars().nth(i - 1).unwrap() == '"' {
                    word = remove_dolar_by_env(word, shell);
                }
            } else if self.value.chars().nth(i).unwrap() == ' ' &&  word != "" {
                word = remove_dolar_by_env(word, shell);
                splitted.push(word.clone());
                word.clear();
                i += 1;
            }
            else {
                word.push(self.value.chars().nth(i).unwrap());
                i += 1;
            }
        }
        if word != "" {
            if self.value.chars().nth(i - 1).unwrap() == '"' {
                word = remove_dolar_by_env(word, shell);
            }
            splitted.push(word);
        }
        return splitted;
    }

    pub fn execute(&self, shell: & mut Shell, mut  pipe: Pipe, last:bool) -> Pipe {
        let mut splitted = self.split_string(&shell);
        let sed_child;
        if self.is_builtin(shell, splitted[0].clone(), &mut splitted) == 1 {
            if last {
                pipe.pipe_in = Stdio::null();
                return pipe;
            }
            pipe.pipe_in = pipe.pipe_out;
            pipe.pipe_out = Stdio::null();
            return pipe;
        }
        if shell.env.path_validation(&splitted[0]) {
                if splitted[1..].concat() != "" {
                    sed_child = Command::new(&splitted[0])
                    .arg(splitted[1..].concat())
                    .env_clear()
                    .envs(shell.env.get_all())
                    .stdin(pipe.pipe_in)
                    .stdout(pipe.pipe_out)
                    .spawn().expect("error on exec");
            } else {
                sed_child = Command::new(&splitted[0])
                .stdin(pipe.pipe_in)
                .env_clear()
                .envs(shell.env.get_all())
                .stdout(pipe.pipe_out)
                .spawn().expect("error on exec");
            } 
            if last {
                (sed_child.wait_with_output().expect("error on wait"));
                pipe.pipe_in = Stdio::null();
                pipe.pipe_out = Stdio::null();
                return pipe;
            }
            pipe.pipe_in = sed_child.stdout.expect("error on stdout").into();
            pipe.pipe_out = Stdio::null();
            return pipe;
        } else {
            eprintln!("minishell: {}: command not found", splitted[0]);
            shell.error = 127;
            pipe.pipe_in = pipe.pipe_out;
            pipe.pipe_out = Stdio::null();
            return pipe;
        }
    }
}

#[derive(Clone)]
pub struct ParsedHead {
    pub n_cmds: i32,
    pub tokens: Vec<ElementLine>,
}

impl ParsedHead {
    pub fn add_token(&mut self, cmd: ElementLine) {
        if self.tokens.last() != None && cmd.parse_type == ParseTypes::Word && self.tokens.last().unwrap().parse_type != ParseTypes::Redirection {
            self.n_cmds += 1;
        }
        if self.tokens.last() == None && cmd.parse_type == ParseTypes::Word {
            self.n_cmds += 1;
        }
        self.tokens.push(cmd);
    }

    pub fn new() -> ParsedHead {
        ParsedHead {
            n_cmds: 0,
            tokens: Vec::new(),
        }
    }
    // pub fn get_all_until_next(&mut self, now: &ElementLine) -> Vec<&ElementLine> {
    //     let mut list = Vec::new();
    //     let mut now_pos = false;
    //     for token in self.tokens.iter() {
    //         if token == now {
    //             now_pos = true;
    //         } else if now_pos && token.get_type() == &ParseTypes::Pipe {
    //             break;
    //         } else if now_pos && token.get_type() == &ParseTypes::End {
    //             break;
    //         } else if now_pos {
    //             list.push(token);
    //         }
    //     }
    //     return list;
    // }
}
