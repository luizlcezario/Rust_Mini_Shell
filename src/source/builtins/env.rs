use crate::source::minishell::Shell;



pub fn built_env(shell: &mut Shell, tokens:  & mut Vec<&str>) -> i32  {
	if tokens.len() > 1 {
		eprintln!("minishell: env: too many arguments");
		return 1;
	}
	for (key, value) in shell.env.get_all().iter() {
		println!("{}={}", key, value);
	}
	return 0;
}