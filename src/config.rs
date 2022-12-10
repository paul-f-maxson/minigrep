use std::{env, fmt::Display};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

pub enum ConfigError {
    InputError,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ConfigError::InputError => write!(f, "two arguments required"),
        }
    }
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, ConfigError> {
        // Discard first arg (name of program)
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err(ConfigError::InputError),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err(ConfigError::InputError),
        };

        let ignore_case = match args.next() {
            Some(arg) => {
                vec!["true".to_owned(), "yes".to_owned(), "ignorecase".to_owned()].contains(&arg)
            }
            None => env::var("IGNORE_CASE").is_ok(),
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
