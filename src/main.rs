mod error;
mod scanner;
mod token;
mod token_type;

use std::{
    env, fs,
    io::{self, Write},
    path::Path,
    process,
};

use error::ErrorReporter;
use scanner::Scanner;

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);
    match args.len() {
        0 => run_prompt(),
        1 => run_file(args.next().unwrap()),
        _ => {
            println!("Usage: rox [script]");
            process::exit(64);
        }
    }
}

fn run_prompt() -> io::Result<()> {
    let mut lines = io::stdin().lines();
    loop {
        print!("> ");
        io::stdout().flush()?;
        let Some(line) = lines.next() else {
            return Ok(());
        };
        run(line?);
    }
}

fn run_file(path: impl AsRef<Path>) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;
    let er = run(contents);
    if er.had_error() {
        process::exit(65);
    }
    Ok(())
}

fn run(source: String) -> ErrorReporter {
    let mut er = ErrorReporter::new();
    let scanner = Scanner::new(source.into(), &mut er);
    let tokens = scanner.scan_tokens();
    for t in tokens {
        println!("{t}");
    }
    er
}
