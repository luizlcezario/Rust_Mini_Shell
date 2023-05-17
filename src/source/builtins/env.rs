use crate::source::minishell::Shell;


pub fn built_env(shell: &Shell, tokens: &mut Vec<String>) -> i32 {
    if tokens.len() > 1 {
        eprintln!("minishell: env: too many arguments");
        return 1;
    }
    let mut env = String::new();
    for (key, value) in shell.env.get_all() {
            env += &key;
            env += "=";
            env += &value;
            env += "\n";
    }
    tokens[0] = "echo".to_string();
    tokens[1] = "-n".to_string();
    tokens[2] = env;
    return 0;
}