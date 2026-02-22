use std::{
    env::{self},
    fs, io,
};

pub struct LoxError {
    line: usize,
    message: String,
}

impl LoxError {
    pub fn report(&self) {
        eprint!("[line {}] Error {}", self.line, self.message)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let length = args.len();
    if length > 1 {
        print!("USAGE: ./binary /path/to/file \n OR \n cargo run -- path/to/file")
    } else if length == 1 {
        run_from_file(args.get(1).unwrap()).unwrap_or_else(|err| {
            panic!("failed to run from file: {:?}", err);
        });
    } else {
        run_by_prompt().unwrap_or_else(|err| {
            panic!("failed to run interactive prompt: {:?}", err);
        });
    }
    println!("Hello, world!");
}

fn run_from_file(path: &str) -> io::Result<()> {
    let content = fs::read_to_string(path)?;
    run(content);
    Ok(())
}

fn run_by_prompt() -> io::Result<()> {
    loop {
        print!("> ");

        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line)?;

        if bytes_read == 0 {
            print!("");
            break;
        }

        run(line);
    }

    Ok(())
}

fn run(source: String) {
    todo!("scanner + tokens must be implemented in run");
}
