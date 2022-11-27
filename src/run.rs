use crate::{config, search};
use std::error::Error;
use std::fs;

pub fn run(
    config::Config {
        query,
        file_path,
        ignore_case,
    }: config::Config,
) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    for line in {
        // Choose search function based on config
        if ignore_case {
            search::search_case_insensitive
        } else {
            search::search_case_sensitive
        }
    }(&query, &contents)
    {
        println!("{line}");
    }

    Ok(())
}
