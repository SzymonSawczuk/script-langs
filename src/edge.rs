#[derive(Default, Debug)]
pub struct Edge<T: std::fmt::Display + std::cmp::Eq> {
    weight: usize,
    second_node: T,
}

impl<T> Edge<T>
where
    T: std::fmt::Display + std::cmp::Eq,
{
    pub fn new(new_weight: usize, new_second_node: T) -> Self {
        Edge::<T> {
            weight: new_weight,
            second_node: new_second_node,
        }
    }

    pub fn get_weight(&self) -> usize {
        return self.weight;
    }

    pub fn get_second_node(&self) -> &T {
        return &self.second_node;
    }

    pub fn set_weight(&mut self, new_weight: usize) {
        self.weight = new_weight;
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
        return self.second_node == other.second_node;
    }
}
