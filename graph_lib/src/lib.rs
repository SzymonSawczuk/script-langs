use pyo3::prelude::*;
pub mod edge;
pub mod graph;
pub mod interface;
pub mod node;

use interface::{EdgeInt, EdgeString, GraphInt, GraphString, NodeInt, NodeString};

/// Module to represent graphs and make computations on them
#[pymodule]
fn graph_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<GraphInt>()?;
    m.add_class::<GraphString>()?;
    m.add_class::<NodeInt>()?;
    m.add_class::<NodeString>()?;
    m.add_class::<EdgeInt>()?;
    m.add_class::<EdgeString>()?;
    Ok(())
}
