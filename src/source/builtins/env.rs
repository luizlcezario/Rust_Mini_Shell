use crate::source::minishell::Shell;


pub fn built_env(shell: &Shell, tokens: &mut Vec<String>) -> i32 {
    if tokens.len() > 1 {
        eprintln!("minishell: env: too many arguments");
        return 1;
    }
    let mut env = String::new();
    for (u, (key, value)) in shell.env.get_all().iter().enumerate() {
            env += &key;
            env += "=";
            env += &value;
            if u != shell.env.get_all().len() - 1 {
                env += "\n";
            }
    }
    tokens.clear();
    tokens.push("echo".to_string());
    tokens.push(env);
    return 0;
}