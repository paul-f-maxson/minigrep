use std::env;
use std::process;

fn main() {
    use minigrep::config::Config;
    use minigrep::run::run;

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("searching for \"{}\" in {}", config.query, config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
