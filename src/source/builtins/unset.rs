use crate::source::minishell::Shell;

pub fn built_unset(shell: &mut Shell, tokens:  & mut Vec<&str>) -> i32  {
	for (u, token) in tokens.iter().enumerate() {
		if u == 0 {
			continue;
		}
		shell.env.remove_env(token.to_string());
	}
	return 0;
}