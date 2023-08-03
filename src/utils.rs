use std::env;

pub fn get_env(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| {
        std::process::exit(1);
    })
}
