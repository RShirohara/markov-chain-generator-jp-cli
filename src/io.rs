use std::fs;
use std::io::stdin;
use std::io::Read;
use std::path::Path;

// Load resource

pub fn load_source(path: Option<&Path>) -> &str {
    match path {
        Some(path) => {
            return load_file(path);
        }
        None => {
            return load_stdin();
        }
    }
}

fn load_file(path: &Path) -> &str {
    let content = fs::read_to_string(path).unwrap();
    return content.as_str();
}

fn load_stdin() -> &'static str {
    let stdin = stdin().lock();
    let buffer = String::new();
    stdin.read_to_string(&mut buffer).unwrap();
    return buffer.as_str();
}

// Write resource
