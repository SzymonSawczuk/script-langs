use clap::Parser;
use std::io;
use std::io::prelude::*;

/// Tail program
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// How many lines to write out
    #[clap(long, default_value_t = 5)]
    lines: usize,

    /// Don't show error
    #[clap(short)]
    e: bool,
}

fn main() {
    let args = Args::parse();
    let lines_of_text: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|x| match x {
            Result::Ok(x) => x,
            Result::Err(err) => {
                eprintln!("Can't read line: {}", err);
                std::process::exit(2);
            }
        })
        .collect();

    let index_begin = if args.lines < lines_of_text.len() {
        lines_of_text.len() - args.lines
    } else {
        0
    };

    lines_of_text[index_begin..]
        .iter()
        .for_each(|elem| println!("{}", elem));

    if args.lines > lines_of_text.len() {
        if !args.e {
            eprintln!(
                "Zabraklo {} linii do wypisania",
                args.lines - lines_of_text.len()
            );
        }
        std::process::exit(2);
    }
}
