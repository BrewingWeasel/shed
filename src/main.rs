use clap::Parser;
use shed::Config;
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

    /// Print lines by default
    #[arg(short = 'n', long, default_value_t = false)]
    quiet: bool,

    #[arg(short = 'p', long, default_value_t = false)]
    pretty_print: bool,
}

fn main() {
    let cli = Args::parse();

    run(cli);
}

fn run(cli: Args) {
    let text = cli.file.as_ref().map_or_else(
        || {
            let mut stdin = String::new();
            std::io::stdin().read_to_string(&mut stdin).unwrap(); // Rewrite all of this
            stdin
        },
        |file| read_to_string(file).unwrap(),
    );
    let modified = shed::parse(&cli.pattern, Config { quiet: cli.quiet }, text);
    if cli.edit_in_place {
        std::fs::write(cli.file.unwrap(), modified).expect("File was unable to be written to");
    } else {
        print!("{modified}");
    }
}
