mod edge;
mod graph;
mod node;

use graph::Graph;

fn main() {
    let mut test = Graph::<String>::default();

    test.add_vertex("x".to_string());
    test.add_vertex("u".to_string());
    test.add_vertex("v".to_string());
    test.add_vertex("y".to_string());
    test.add_vertex("w".to_string());
    test.add_vertex("z".to_string());
    // test.add_vertex("u".to_string());
    // test.add_vertex("y".to_string());

    test.add_edge("u".to_string(), "x".to_string(), 1, true);
    test.add_edge("x".to_string(), "v".to_string(), 1, true);
    test.add_edge("u".to_string(), "v".to_string(), 1, true);
    test.add_edge("v".to_string(), "y".to_string(), 1, true);
    test.add_edge("w".to_string(), "y".to_string(), 1, true);
    test.add_edge("w".to_string(), "z".to_string(), 1, true);
    test.add_edge("z".to_string(), "z".to_string(), 1, true);
    test.add_edge("y".to_string(), "x".to_string(), 1, true);

    test.show_graph();
    println!();

    // test.bfs_algorithm("s".to_string());
    test.dfs_algorithm();
}
