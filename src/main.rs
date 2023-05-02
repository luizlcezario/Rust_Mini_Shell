mod source;
use std::env;
use source::minishell;

fn main() {
    if env::args().len() != 1 {
        println!("Usage: ./minishell");
        return;
    }
    minishell::minishell();
}
