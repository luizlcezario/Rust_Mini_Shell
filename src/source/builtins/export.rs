use crate::source::minishell::Shell;

pub fn built_export(shell: &mut Shell, tokens: &mut [String]) -> i32 {
    for (u, token) in tokens.iter().enumerate() {
        if u == 0 {
            continue;
        }
        let splitted = token.split_once('=');
        match splitted {
            Some((key, value)) => {
                shell.env.set_env(key.to_string(), value.to_string());
            }
            None => {
                eprintln!("minishell: export: {}: not a valid identifier", token);
                return 1;
            }
        }
    }
    0
}
