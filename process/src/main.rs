use clap::Parser;
use std::io;
use std::io::prelude::*;

/// Process program
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Ignore first n characters
    #[clap(long, default_value_t = 0)]
    ignorefirst: usize,

    /// Ignore last n characters
    #[clap(long, default_value_t = 0)]
    ignorelast: usize,

    /// Replace all apperance of character with \t
    #[clap(long, default_value = ",")]
    delimiter: String,

    /// Replace separator in delimiter option
    #[clap(long, default_value = "\t")]
    separator: String,

    /// Write only columns determined in option (format of k l ... m)
    #[clap(long, multiple_values = true, min_values = 0)]
    project: Vec<usize>,

    /// Write only lines which contains specified strings
    #[clap(long, default_value = "")]
    select: String,
}

fn find_delimiter_index(line: &String, delimiter: &String) -> Vec<usize> {
    let mut apperances: Vec<usize> = Vec::new();
    let mut current_possible_index = 0;
    let mut index_delimiter = 0;

    for (index_of_character, character) in line.chars().enumerate() {
        if delimiter.chars().nth(index_delimiter).unwrap() == character {
            index_delimiter += 1;

            if index_delimiter == delimiter.len() {
                apperances.push(current_possible_index);
                index_delimiter = 0;
                current_possible_index = index_of_character + 1;
            }
            continue;
        }

        current_possible_index = index_of_character + 1;
    }

    return apperances;
}

fn make_result_line(
    line: &String,
    result_line: &mut String,
    ignore_first: usize,
    ignore_last: usize,
    delimiter: &String,
    separator: &str,
    project: &Vec<usize>,
    select: &String,
) {
    let mut how_mush_delimiter_to_skip: usize = 0;
    let index_of_delimiter_to_skip: Vec<usize> = find_delimiter_index(line, delimiter);
    let mut current_column: usize = 1;

    if !line.contains(select) {
        return;
    }

    for (index_of_character, character) in line.chars().enumerate() {
        if character == '\t' {
            current_column += 1;
        }
        if index_of_character < ignore_first || index_of_character >= line.len() - ignore_last {
            continue;
        }

        if index_of_delimiter_to_skip.contains(&index_of_character) {
            how_mush_delimiter_to_skip = delimiter.len();
            result_line.push_str(&separator);
            if separator == "\t" {
                current_column += 1;
            }
        }

        if how_mush_delimiter_to_skip != 0 {
            how_mush_delimiter_to_skip -= 1;
            continue;
        }

        if project.len() == 0 || project.contains(&(current_column)) {
            result_line.push(character);
        }
    }
}

fn change_first_gap(result_line: &String) -> String {
    let mut result_line_new: String = String::new();
    let mut first_element_occured: bool = false;
    for character in result_line.chars() {
        if character != ' ' && character != '\t' {
            first_element_occured = true;
        }
        if first_element_occured {
            result_line_new.push(character);
        }
    }

    return result_line_new;
}

fn get_result(lines: Vec<String>, result_text: &mut Vec<String>) {
    let args = Args::parse();

    let separator: &str = match args.separator.as_str() {
        "\\n" => "\n",
        "\\t" => "\t",
        _ => args.separator.as_str(),
    };

    for line in &lines {
        let mut result_line: String = String::new();
        make_result_line(
            line,
            &mut result_line,
            args.ignorefirst,
            args.ignorelast,
            &args.delimiter,
            &separator,
            &args.project,
            &args.select,
        );

        if result_line != "" {
            result_line = change_first_gap(&result_line);
            result_text.push(result_line);
        }
    }
}

fn main() {
    let _args = Args::parse();

    let lines: Vec<String> = io::stdin()
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

    if lines.len() == 0 {
        std::process::exit(2);
    }

    let mut result_text: Vec<String> = Vec::new();
    get_result(lines, &mut result_text);

    if result_text.len() == 0 {
        std::process::exit(1);
    }

    result_text.iter().for_each(|line| println!("{}", line));
}
