use std::{path::Path, env};

use crate::source::minishell::Shell;

pub fn built_cd(shell: & mut Shell, splitted: & mut Vec<&str>) -> u32 {
	if splitted.len() > 2 {
		println!("minishell: cd: too many arguments");
		return 1;
	}
	if splitted.len() == 1 {
		splitted.push("~");
	}
	let root = Path::new(splitted[1]);
	shell.env.set_env("OLDPWD", shell.env.get_env("PWD").unwrap().to_string());
	if env::set_current_dir(&root).is_ok() {
		shell.env.set_env("PWD", env::current_dir().unwrap().to_str().unwrap().to_string());
	} else {
		println!("minishell: cd: {}: No such file or directory", splitted[1]);
		return 1;
	}
	return 0;
}