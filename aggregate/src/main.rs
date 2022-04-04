use clap::Parser;
use std::io::prelude::*;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // / Name of the person to greet
// #[clap(short, long)]
// name: String,

// /// Number of times to greet
// #[clap(short, long, default_value_t = 1)]
// count: u8,
}

fn find_minimum<T>(values: &Vec<T>) -> &T {
    return &values[0];
}

fn find_maximum<T>(values: &Vec<T>) -> &T {
    return &values[values.len() - 1];
}

fn find_sum(values: &Vec<i32>) -> i32 {
    return values.iter().fold(0, |mut sum, &val| {
        sum += val;
        sum
    });
}

fn find_avarage(values: &Vec<i32>) -> f32 {
    return find_sum(values) as f32 / (count_elements(values) as f32);
}

fn count_elements(values: &Vec<i32>) -> i32 {
    return values.iter().fold(0, |mut sum, _| {
        sum += 1;
        sum
    });
}

fn main() {
    let args = Args::parse();

    let mut values: Vec<i32> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| match x {
            Result::Ok(elem) => match elem.parse::<i32>() {
                Result::Ok(result) => result,
                Result::Err(error) => {
                    eprintln!("{}", error);
                    std::process::exit(2);
                }
            },
            Result::Err(error) => {
                eprintln!("{}", error);
                std::process::exit(2);
            }
        })
        .collect();

    println!("{:?}", values);
    values.sort();

    println!("{:?}", values);
    println!("{}", find_minimum(&values));
    println!("{}", find_maximum(&values));
    println!("{}", find_sum(&values));
    println!("{}", find_avarage(&values));
    println!("{}", count_elements(&values));
}
