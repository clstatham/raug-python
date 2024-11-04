use pyo3::prelude::*;
use raug::prelude::*;

#[pyclass(name = "Graph")]
pub struct PyGraph(pub Graph);

#[pymethods]
impl PyGraph {
    #[new]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        PyGraph(Graph::new())
    }
}
