use pyo3::prelude::*;

use crate::edge::Edge;
use crate::graph::Graph;
use crate::node::Node;

macro_rules! create_interface_graph {
    ($name: ident, $type: ty) => {
        #[pyclass]
        pub struct $name {
            pub inner: Graph<$type>,
        }
        #[pymethods]
        impl $name {
            #[new]
            pub fn new() -> Self {
                Self {
                    inner: Graph::<$type>::new(),
                }
            }

            ///Add new vertex
            pub fn add_vertex(&mut self, value: $type) {
                self.inner.add_vertex(value);
            }

            ///Show structure of graph
            pub fn show_graph(&self) {
                self.inner.show_graph();
            }

            /// Add edge by passing values of ends and weight of edge (optionally is it directed (default false))
            #[args(is_directed = "false")]
            pub fn add_edge(
                &mut self,
                value_one: $type,
                value_two: $type,
                weight: usize,
                is_directed: bool,
            ) {
                self.inner
                    .add_edge(value_one, value_two, weight, is_directed);
            }

            ///Remove vertex with typed value
            pub fn remove_vertex(&mut self, value: $type) {
                self.inner.remove_vertex(value);
            }

            ///Remove edge with typed informations
            pub fn remove_edge(&mut self, value_one: $type, value_two: $type, is_directed: bool) {
                self.inner.remove_edge(value_one, value_two, is_directed);
            }

            ///Breadth-first search starting from start_value of vertex
            pub fn bfs_algorithm(&self, start_value: $type) {
                self.inner.bfs_algorithm(start_value);
            }

            ///Depth-first search
            pub fn dfs_algorithm(&self) {
                self.inner.dfs_algorithm();
            }
        }
    };
}

create_interface_graph!(GraphInt, i64);
create_interface_graph!(GraphString, String);

macro_rules! create_interface_node {
    ($name: ident, $type: ty) => {
        #[pyclass]
        pub struct $name {
            pub inner: Node<$type>,
        }
        #[pymethods]
        impl $name {
            #[new]
            pub fn new(new_value: $type) -> Self {
                Self {
                    inner: Node::<$type>::new(new_value),
                }
            }

            pub fn get_value(&self) -> PyResult<$type> {
                return Ok(self.inner.get_value().clone());
            }

            pub fn set_value(&mut self, new_value: $type) {
                self.inner.set_value(new_value);
            }
        }
    };
}

create_interface_node!(NodeInt, i64);
create_interface_node!(NodeString, String);

macro_rules! create_interface_edge {
    ($name: ident, $type: ty) => {
        #[pyclass]
        pub struct $name {
            pub inner: Edge<$type>,
        }
        #[pymethods]
        impl $name {
            #[new]
            pub fn new(new_weight: usize, new_first_node: $type, new_second_node: $type) -> Self {
                Self {
                    inner: Edge::<$type>::new(new_weight, new_first_node, new_second_node),
                }
            }

            pub fn get_weight(&self) -> usize {
                return self.inner.get_weight();
            }

            pub fn get_first_node(&self) -> $type {
                return self.inner.get_first_node().clone();
            }

            pub fn get_second_node(&self) -> $type {
                return self.inner.get_second_node().clone();
            }

            pub fn set_weight(&mut self, new_weight: usize) {
                self.inner.set_weight(new_weight);
            }

            pub fn set_first_node(&mut self, new_first_node: $type) {
                self.inner.set_first_node(new_first_node);
            }

            pub fn set_second_node(&mut self, new_second_node: $type) {
                self.inner.set_second_node(new_second_node);
            }
        }
    };
}

create_interface_edge!(EdgeInt, i64);
create_interface_edge!(EdgeString, String);
