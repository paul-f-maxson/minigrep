use std::env;
use std::fs;
use std::process;

fn main() {
    use crate::Config::Config;
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    run(config);
}

mod Config {
    use std::fmt::Display;
    use std::io;

    pub struct Config {
        pub query: String,
        pub file_path: String,
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

            Ok(Config { query, file_path })
        }
    }
}

fn run(Config::Config { query, file_path }: Config::Config) {
    let contents = fs::read(file_path).expect("Should have been able to read file");
}
