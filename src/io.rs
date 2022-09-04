use markov::Chain;
use std::fs;
use std::io::stdin;
use std::io::stdout;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::path::Path;

// Load resource

pub fn load_source(path: Option<&Path>) -> String {
    match path {
        Some(path) => {
            return load_file(path);
        }
        None => {
            return load_stdin();
        }
    }
}

fn load_file(path: &Path) -> String {
    let content = fs::read_to_string(path).unwrap();
    return content;
}

fn load_stdin() -> String {
    let mut stdin = stdin().lock();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer).unwrap();
    return buffer;
}

// Write resource

pub fn write_stdout(chain: &Chain<String>, repeat: &usize) {
    let stdout = stdout().lock();
    let mut buffer = BufWriter::new(stdout);

    // Write generated string
    for s in chain.str_iter_for(*repeat) {
        writeln!(buffer, "{}", s.replace(" ", "")).unwrap();
    }
}
