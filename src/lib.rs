
use std::fs;

fn run(query: &String, filepath: &String) {
    let contents = fs::read(filepath).expect("Should have been able to read file");

}
