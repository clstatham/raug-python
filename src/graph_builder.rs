use pyo3::prelude::*;
use raug::prelude::*;

use crate::{graph::PyGraph, node_builder::PyNode, runtime::PyRuntime};

#[pyclass(name = "GraphBuilder")]
pub struct PyGraphBuilder(pub(crate) GraphBuilder);

#[pymethods]
impl PyGraphBuilder {
    #[new]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        PyGraphBuilder(GraphBuilder::new())
    }

    pub fn build(&self) -> PyResult<PyGraph> {
        Ok(PyGraph(self.0.build()))
    }

    pub fn build_runtime(&self) -> PyResult<PyRuntime> {
        Ok(PyRuntime(self.0.build_runtime()))
    }

    pub fn add_input(&self) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add_input()))
    }

    pub fn add_output(&self) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add_output()))
    }

    pub fn phase_accum(&self) -> PyResult<PyNode> {
        Ok(PyNode(self.0.phase_accum()))
    }

    pub fn sine_osc(&self) -> PyResult<PyNode> {
        Ok(PyNode(self.0.sine_osc()))
    }

    pub fn saw_osc(&self) -> PyResult<PyNode> {
        Ok(PyNode(self.0.saw_osc()))
    }

    pub fn connect(
        &self,
        src: &PyNode,
        src_output: u32,
        dst: &PyNode,
        dst_input: u32,
    ) -> PyResult<()> {
        self.0.connect(&src.0, src_output, &dst.0, dst_input);
        Ok(())
    }
}
