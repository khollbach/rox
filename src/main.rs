mod token_type;
mod token;

use std::{env, process, fs::{File, self}, io::{self, Write}};

fn main() {
    Lox::new().main();
}

struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Self {
        Self { had_error: false }
    }

    fn main(mut self) {
        let mut args = env::args().skip(1);
        match args.len() {
            0 => self.run_prompt(),
            1 => self.run_file(args.next().unwrap()),
            _ => {
                println!("Usage: rox [script]");
                process::exit(64);
            }
        }
    }

    fn run_prompt(&mut self) -> io::Result<()> {
        let mut lines = io::stdin().lines();
        loop {
            print!("> ");
            io::stdout().flush()?;

            let Some(line) = lines.next() else {
                return Ok(());
            } 

            self.run(line?);
            self.had_error = false;
        }
    }

    fn run_file(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        let contents = fs::read_to_string(path)?
        self.run(&contents);

        if self.had_error {
            process::exit(65);
        }
    }

    fn run(&mut self, source: &str) {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for t in tokens {
            println!("{t}");
        }
    }

    fn error(&mut self, line: usize, message: &str) {
        report(line, "", message);
    }

    fn report(&mut self, line: usize, where_: &str, message: &str) {
        eprintln!("[line {line}] Error{where_}: {message}");
        self.had_error = true;
    }
}
