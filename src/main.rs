mod io;
mod markov;
mod tokenize;

use std::path::PathBuf;

use clap::Parser;
use clap::ValueHint;

#[derive(Parser)]
#[clap(
    name="markov",
    version,
    about,
    long_about = None
)]
struct Arg {
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    input: Option<PathBuf>,

    #[clap(short, long, default_value_t = 2)]
    state_size: usize,

    #[clap(short, long, default_value_t = 100)]
    repeat: usize,
}

fn main() {
    let args: Arg = Arg::parse();

    // Load resource
    let source = io::load_source(args.input.as_deref());
    let tokens = tokenize::tokenize(&source);
    // Create chain
    let chain = markov::build_chain(&tokens, &args.state_size);
    // Write results
    io::write_stdout(&chain, &args.repeat);
}
