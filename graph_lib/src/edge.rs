use pyo3::prelude::*;

#[derive(Default, Debug, FromPyObject)]
pub struct Edge<T> {
    weight: usize,
    first_node: T,
    second_node: T,
}

impl<T> Edge<T>
where
    T: std::fmt::Display + std::cmp::Eq,
{
    pub fn new(new_weight: usize, new_first_node: T, new_second_node: T) -> Self {
        Edge::<T> {
            weight: new_weight,
            first_node: new_first_node,
            second_node: new_second_node,
        }
    }

    pub fn get_weight(&self) -> usize {
        return self.weight;
    }

    pub fn get_first_node(&self) -> &T {
        return &self.first_node;
    }

    pub fn get_second_node(&self) -> &T {
        return &self.second_node;
    }

    pub fn set_weight(&mut self, new_weight: usize) {
        self.weight = new_weight;
    }

    pub fn set_first_node(&mut self, new_first_node: T) {
        self.first_node = new_first_node;
    }

    pub fn set_second_node(&mut self, new_second_node: T) {
        self.second_node = new_second_node;
    }
}

impl<T> PartialEq for Edge<T>
where
    T: std::fmt::Display + std::cmp::Eq,
{
    fn eq(&self, other: &Self) -> bool {
        return self.first_node == other.first_node && self.second_node == other.second_node;
    }
}
