use std::fmt::Display;

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
    pub fn build(args: &[String]) -> Result<Config, ConfigError> {
        let query = {
            if let Some(val) = args.get(1) {
                val.clone()
            } else {
                return Err(ConfigError::InputError);
            }
        };

        let file_path = {
            if let Some(val) = args.get(2) {
                val.clone()
            } else {
                return Err(ConfigError::InputError);
            }
        };

        let ignore_case = {
            if let Some(val) = args.get(3) {
                vec!["true".to_owned(), "yes".to_owned(), "ignorecase".to_owned()].contains(val)
            } else {
                return Err(ConfigError::InputError);
            }
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
