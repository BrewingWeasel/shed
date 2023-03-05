use clap::Parser;
use std::{fs::read_to_string, io::Read};

/// Clone of sed
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Sed pattern to use (ex: `s/original/final/`)
    pattern: String,

    /// Optional file, reads from stdin if not inputed
    file: Option<String>,

    /// Edit in place
    #[arg(short = 'i', long, default_value_t = false)]
    edit_in_place: bool,
}

fn main() {
    let cli = Args::parse();

    run(cli);
}

fn run(cli: Args) {
    let text = match cli.file {
        Some(file) => read_to_string(file).unwrap(),
        None => {
            let mut stdin = String::new();
            std::io::stdin().read_to_string(&mut stdin).unwrap(); // Rewrite all of this
            stdin
        }
    };
    shed::parse(cli.pattern, text, cli.edit_in_place);
}
