use crate::edge::Edge;
use pyo3::prelude::*;

#[derive(Default, Debug, FromPyObject)]
pub struct Node<T> {
    #[pyo3(item)]
    value: T,
    #[pyo3(item)]
    edges: Vec<Edge<T>>,
}

impl<T> Node<T>
where
    T: std::fmt::Display + std::cmp::Eq + Clone,
{
    pub fn new(new_value: T) -> Self {
        Node::<T> {
            value: new_value,
            edges: Vec::new(),
        }
    }

    pub fn get_value(&self) -> &T {
        return &self.value;
    }

    pub fn get_edges(&self) -> &Vec<Edge<T>> {
        return &self.edges;
    }

    pub fn get_edges_mut(&mut self) -> &mut Vec<Edge<T>> {
        return &mut self.edges;
    }

    pub fn set_value(&mut self, new_value: T) {
        self.value = new_value;
    }

    pub fn add_edge(&mut self, weight: usize, second_node: T) {
        self.edges
            .push(Edge::<T>::new(weight, self.value.clone(), second_node));
    }

    ///Remove all edges with typed value of second vertex
    pub fn remove_all_edges(&mut self, value_of_second_node: &T) {
        let mut indexes_to_remove: Vec<usize> = Vec::new();
        for (index, edge) in self.edges.iter().enumerate() {
            if edge.get_second_node() == value_of_second_node {
                indexes_to_remove.push(index);
            }
        }
        indexes_to_remove.iter().for_each(|x| {
            self.edges.remove(*x);
            ()
        });
    }
}
