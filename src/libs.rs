use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;

// Load resource

pub fn load_source(path: Option<&Path>) -> Vec<String> {
    match path {
        Some(path) => {
            return load_file(path);
        }
        None => {
            return load_stdin();
        }
    }
}

fn load_file(path: &Path) -> Vec<String> {
    let file = File::open(path).unwrap();
    let buffer = BufReader::new(file);
    return buffer.lines().collect::<Result<_, _>>().unwrap();
}

fn load_stdin() -> Vec<String> {
    let stdin = stdin().lock();
    let buffer = BufReader::new(stdin);
    return buffer.lines().collect::<Result<_, _>>().unwrap();
}

// Write resource
