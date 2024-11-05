use pyo3::prelude::*;
use raug::prelude::*;

use crate::{graph::PyGraph, node_builder::PyParam};

#[pyclass(name = "Runtime")]
pub struct PyRuntime(pub(crate) Runtime);

#[pymethods]
impl PyRuntime {
    #[new]
    #[allow(clippy::new_without_default)]
    pub fn new(graph: Bound<PyGraph>) -> Self {
        PyRuntime(Runtime::new(graph.borrow().0.clone()))
    }

    pub fn run(&mut self) -> PyResult<PyRuntimeHandle> {
        Ok(PyRuntimeHandle(
            self.0.run(Backend::Default, Device::Default).unwrap(),
        ))
    }

    pub fn run_for(&mut self, duration: f64) -> PyResult<()> {
        self.0
            .run_for(
                Duration::from_secs_f64(duration),
                Backend::Default,
                Device::Default,
            )
            .unwrap();
        Ok(())
    }

    pub fn param_names(&self) -> Vec<String> {
        let mut names: Vec<_> = self
            .0
            .param_iter()
            .map(|(name, _)| name.to_string())
            .collect();
        names.sort();
        names
    }

    pub fn param_named(&self, name: &str) -> PyResult<PyParam> {
        Ok(PyParam(self.0.param_named(name).unwrap().clone()))
    }
}

#[pyclass(name = "RuntimeHandle")]
pub struct PyRuntimeHandle(pub(crate) RuntimeHandle);

#[pymethods]
impl PyRuntimeHandle {
    pub fn stop(&self) {
        self.0.stop();
    }
}
