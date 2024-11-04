use pyo3::prelude::*;

pub mod graph;
pub mod graph_builder;
pub mod node_builder;
pub mod runtime;

/// A Python module implemented in Rust.
#[pymodule(name = "raug")]
fn raug(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<graph::PyGraph>()?;
    m.add_class::<graph_builder::PyGraphBuilder>()?;
    m.add_class::<node_builder::PyNode>()?;
    m.add_class::<node_builder::PyInput>()?;
    m.add_class::<node_builder::PyOutput>()?;
    m.add_class::<runtime::PyRuntime>()?;
    m.add_class::<runtime::PyRuntimeHandle>()?;

    Ok(())
}
