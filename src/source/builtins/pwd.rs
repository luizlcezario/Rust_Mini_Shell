use crate::source::minishell::Shell;


pub fn built_pwd(shell: &mut Shell, tokens:  & mut Vec<&str>) -> i32 {
	if tokens.len() > 1 {
		eprintln!("minishell: pwd: too many arguments");
		return 1;
	}
	let pwd = shell.env.get_env("PWD").unwrap();
	println!("{}", pwd);
	return 0;
}