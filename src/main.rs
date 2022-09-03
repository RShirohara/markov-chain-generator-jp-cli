mod libs;

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

    #[clap(short, long, value_hint = ValueHint::FilePath)]
    output: Option<PathBuf>,
}

fn main() {
    let args: Arg = Arg::parse();

    // Load resource
    let source = libs::load_source(args.input.as_deref());
}
