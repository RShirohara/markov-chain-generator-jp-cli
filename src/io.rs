use markov::Chain;
use std::fs;
use std::io::{stdin, stdout, BufWriter, Error, Read, Write};
use std::path::Path;

// Load resource

pub fn load_resource(path: Option<&Path>) -> Result<String, Error> {
    match path {
        Some(path) => read_file(path),
        None => read_stdin(),
    }
}

fn read_file(path: &Path) -> Result<String, Error> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

fn read_stdin() -> Result<String, Error> {
    let stdin = stdin();
    let mut buffer = String::new();
    stdin.lock().read_to_string(&mut buffer)?;
    Ok(buffer)
}

// Write resource

pub fn write_stdout(chain: &Chain<String>, repeat: &usize) -> Result<(), Error> {
    let stdout = stdout();
    let mut buffer = BufWriter::new(stdout);

    // Write generated string
    for s in chain.str_iter_for(*repeat) {
        writeln!(buffer, "{}", s.replace(' ', ""))?;
    }
    Ok(())
}
