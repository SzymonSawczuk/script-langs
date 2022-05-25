use crate::edge::Edge;
use crate::node::Node;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Graph<T: std::cmp::Eq + std::hash::Hash + Clone + std::fmt::Display + std::fmt::Debug> {
    vertexes: HashMap<T, Node<T>>,
}

impl<T> Graph<T>
where
    T: std::cmp::Eq + std::hash::Hash + Clone + std::fmt::Display + std::fmt::Debug,
{
    pub fn add_vertex(&mut self, value: T) {
        if self.vertexes.contains_key(&value) {
            return;
        }
        self.vertexes.insert(value.clone(), Node::<T>::new(value));
    }

    pub fn show_graph(&self) {
        for vertex in &self.vertexes {
            println!("{}: {:?}", vertex.1.get_value(), vertex.1.get_edges());
        }
    }

    pub fn add_edge(&mut self, value_one: T, value_two: T, weight: usize, is_directed: bool) {
        if !self.vertexes.contains_key(&value_one) || !self.vertexes.contains_key(&value_two) {
            return;
        }

        if !self
            .vertexes
            .get(&value_one)
            .unwrap()
            .get_edges()
            .contains(&Edge::<T>::new(0, value_two.clone()))
        {
            self.vertexes
                .get_mut(&value_one)
                .unwrap()
                .add_edge(weight, value_two.clone());
        }

        if !is_directed
            && !self
                .vertexes
                .get(&value_two)
                .unwrap()
                .get_edges()
                .contains(&Edge::<T>::new(0, value_one.clone()))
        {
            self.vertexes
                .get_mut(&value_two)
                .unwrap()
                .add_edge(weight, value_one.clone());
        }
    }

    pub fn remove_vertex(&mut self, value: T) {
        if !self.vertexes.contains_key(&value) {
            return;
        }

        for vertex in &mut self.vertexes {
            if *vertex.0 != value {
                vertex.1.remove_all_edges(&value);
            }
        }

        self.vertexes.remove(&value);
    }

    pub fn remove_edge(&mut self, value_one: T, value_two: T, is_directed: bool) {
        if !self.vertexes.contains_key(&value_one) || !self.vertexes.contains_key(&value_two) {
            return;
        }

        self.vertexes
            .get_mut(&value_one)
            .unwrap()
            .remove_all_edges(&value_two);

        if !is_directed {
            self.vertexes
                .get_mut(&value_two)
                .unwrap()
                .remove_all_edges(&value_one);
        }
    }

    pub fn bfs_algorithm(&self, start_value: T) {
        let mut queue: Vec<T> = Vec::new();
        let mut info: HashMap<T, (String, usize, T)> = HashMap::new();
        for vertex in &self.vertexes {
            if vertex.0 != &start_value {
                info.insert(
                    vertex.0.clone(),
                    ("WHITE".to_string(), std::usize::MAX, vertex.0.clone()),
                );
            }
        }
        info.insert(
            start_value.clone(),
            ("GREY".to_string(), 0, start_value.clone()),
        );

        queue.push(start_value);

        while queue.len() != 0 {
            let first_element_in_queue = queue.remove(0);
            let step_first_element = info.get(&first_element_in_queue).unwrap().1;

            for adjusted_nodes in self
                .vertexes
                .get(&first_element_in_queue)
                .unwrap()
                .get_edges()
            {
                if info.get_mut(&adjusted_nodes.get_second_node()).unwrap().0 == "WHITE".to_string()
                {
                    *info.get_mut(&adjusted_nodes.get_second_node()).unwrap() = (
                        "GREY".to_string(),
                        step_first_element + 1,
                        first_element_in_queue.clone(),
                    );
                    queue.push(adjusted_nodes.get_second_node().clone());
                }
            }
            info.get_mut(&first_element_in_queue).unwrap().0 = "BLACK".to_string();
        }

        println!("{:#?}", info);
    }

    pub fn dfs_algorithm(&self) {
        let mut info: HashMap<T, (String, (usize, usize), T)> = HashMap::new();

        for vertex in &self.vertexes {
            info.insert(
                vertex.0.clone(),
                ("WHITE".to_string(), (0, 0), vertex.0.clone()),
            );
        }

        let mut time = 0;

        for vertex in &self.vertexes {
            if info.get(&vertex.0).unwrap().0 == "WHITE".to_string() {
                self.dfs_visit(&mut info, vertex.0.clone(), &mut time);
            }
        }

        println!("{:#?}", info);
    }

    pub fn dfs_visit(
        &self,
        info: &mut HashMap<T, (String, (usize, usize), T)>,
        vertex: T,
        time: &mut usize,
    ) {
        info.get_mut(&vertex).unwrap().0 = "GREY".to_string();
        *time += 1;
        info.get_mut(&vertex).unwrap().1 .0 = *time;

        for adjusted_nodes in self.vertexes.get(&vertex).unwrap().get_edges() {
            if info.get_mut(&adjusted_nodes.get_second_node()).unwrap().0 == "WHITE".to_string() {
                info.get_mut(&adjusted_nodes.get_second_node()).unwrap().2 = vertex.clone();
                self.dfs_visit(info, adjusted_nodes.get_second_node().clone(), time);
            }
        }

        info.get_mut(&vertex).unwrap().0 = "BLACK".to_string();
        *time += 1;
        info.get_mut(&vertex).unwrap().1 .1 = *time;
    }
}
