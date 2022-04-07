mod aggregator_lib;
mod arguments_lib;
mod read_values_lib;

use aggregator_lib::Aggregator;
use arguments_lib::Args;
use clap::Parser;
use read_values_lib::ValuesReader;

fn main() {
    let args = Args::parse();
    let mut aggregator = Aggregator::<f32>::default();
    ValuesReader::read_values_from_input(&mut aggregator);
    let mut description = "";

    if args.minimum {
        if !args.nodesc {
            description = "Minimum value is: ";
        }
        println!("{}{}", description, aggregator.find_minimum());
    }

    if args.maximum {
        if !args.nodesc {
            description = "Maximum value is: ";
        }
        println!("{}{}", description, aggregator.find_maximum());
    }

    if args.sum {
        if !args.nodesc {
            description = "Sum of values is: ";
        }
        println!("{}{}", description, aggregator.find_sum());
    }

    if args.avarage {
        if !args.nodesc {
            description = "Avarage value is: ";
        }
        println!("{}{}", description, aggregator.find_avarage());
    }

    if args.count {
        if !args.nodesc {
            description = "Amount of values is: ";
        }
        println!("{}{}", description, aggregator.count_elements());
    }

    if args.median {
        if !args.nodesc {
            description = "Median of values is: ";
        }
        println!("{}{}", description, aggregator.find_median());
    }

    if args.variance {
        if !args.nodesc {
            description = "Variance of values is: ";
        }
        println!("{}{}", description, aggregator.find_variance());
    }
}
