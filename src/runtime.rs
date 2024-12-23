use pyo3::prelude::*;
use raug::prelude::*;

use crate::graph::PyGraph;

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
            self.0
                .run(AudioBackend::Default, AudioDevice::Default, None)
                .unwrap(),
        ))
    }

    pub fn run_for(&mut self, duration: Float) -> PyResult<()> {
        self.0
            .run_for(
                Duration::from_secs_f64(duration as f64),
                AudioBackend::Default,
                AudioDevice::Default,
                None,
            )
            .unwrap();
        Ok(())
    }

    pub fn run_offline_to_file(&mut self, path: &str, duration: Float) -> PyResult<()> {
        self.0
            .run_offline_to_file(
                path,
                Duration::from_secs_f64(duration as f64),
                48_000.0,
                512,
            )
            .unwrap();
        Ok(())
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
