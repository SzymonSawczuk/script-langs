#[derive(Default, Debug)]
pub struct Aggregator<
    T: num::Num + std::ops::AddAssign + std::marker::Copy + PartialOrd<T> + std::ops::Sub<Output = T>,
> {
    values: Vec<T>,
}

impl<T> Aggregator<T>
where
    T: num::Num
        + std::ops::AddAssign
        + std::marker::Copy
        + PartialOrd<T>
        + std::ops::Sub<Output = T>,
{
    pub fn change_values(&mut self, new_values: Vec<T>) {
        self.values = new_values.clone();
    }

    fn clone_values_to_sorted(&self) -> Vec<T> {
        let mut sorted_values = self.values.clone();
        sorted_values
            .sort_by(|first_elem, second_elem| first_elem.partial_cmp(second_elem).unwrap());

        return sorted_values;
    }

    fn find_minimum_maximum(&self, index: usize) -> T {
        let sorted_values = self.clone_values_to_sorted();

        if sorted_values.len() > 0 {
            return sorted_values[index];
        }
        return T::zero();
    }

    pub fn find_minimum(&self) -> T {
        return self.find_minimum_maximum(0);
    }

    pub fn find_maximum(&self) -> T {
        if self.values.len() <= 0 {
            return T::zero();
        }
        return self.find_minimum_maximum(self.values.len() - 1);
    }

    pub fn find_sum(&self) -> T {
        return self.values.iter().fold(T::zero(), |mut sum, &val| {
            sum += val;
            sum
        });
    }

    pub fn find_avarage(&self) -> T {
        return self.find_sum() / self.count_elements();
    }

    pub fn count_elements(&self) -> T {
        return self.values.iter().fold(T::zero(), |mut sum, _| {
            sum += T::one();
            sum
        });
    }

    pub fn find_median(&self) -> T {
        if self.values.len() <= 0 {
            return T::zero();
        }

        let sorted_values = self.clone_values_to_sorted();

        let index_middle = sorted_values.len() / 2;

        if sorted_values.len() % 2 == 0 {
            return (sorted_values[index_middle - 1] + sorted_values[index_middle])
                / (T::one() + T::one());
        }

        return sorted_values[index_middle];
    }

    pub fn find_variance(&self) -> T {
        let avarage_value = self.find_avarage();
        let mut sum_of_sub = T::zero();
        let mut length = T::zero();

        for value in &self.values {
            sum_of_sub += num::pow(*value - avarage_value, 2);
            length += T::one();
        }

        return sum_of_sub / (length - T::one());
    }
}
