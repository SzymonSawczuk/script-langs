use crate::aggregator_lib::Aggregator;
use std::io::prelude::*;

pub trait ValuesReader {
    fn read_values_from_input(&mut self);
}

impl<T> ValuesReader for Aggregator<T>
where
    T: num::Num + std::str::FromStr + PartialOrd<T> + std::ops::AddAssign + std::marker::Copy,
{
    fn read_values_from_input(&mut self) {
        self.change_values(
            std::io::stdin()
                .lock()
                .lines()
                .filter_map(|x| match x {
                    Result::Ok(elem) => elem.parse::<T>().ok(),
                    Result::Err(error) => {
                        eprintln!("{}", error);
                        std::process::exit(2);
                    }
                })
                .collect(),
        )
    }
}
