use std::fs::File;

use pyo3::prelude::*;
use raug::prelude::*;

use crate::{
    graph::PyGraph,
    message::{PyBang, PyMessage},
    node_builder::{PyNode, PyParam},
    runtime::PyRuntime,
};

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

    pub fn write_dot(&self, path: &str) -> PyResult<()> {
        self.0.write_dot(&mut File::create(path)?);
        Ok(())
    }

    pub fn print(&self) -> PyResult<PyNode> {
        Ok(PyNode(self.0.print(None, None)))
    }

    pub fn sample_rate(&self) -> PyResult<PyNode> {
        Ok(PyNode(self.0.sample_rate()))
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

    pub fn constant(&self, value: f64) -> PyResult<PyNode> {
        Ok(PyNode(self.0.constant(value)))
    }

    pub fn load_buffer(&self, path: &str) -> PyResult<PyNode> {
        let buffer = Buffer::load_wav(path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let reader = BufferReader::new(buffer);
        Ok(PyNode(self.0.add_processor(reader)))
    }

    pub fn buffer(&self, contents: Vec<f64>) -> PyResult<PyNode> {
        let samples: Vec<_> = contents.into_iter().map(Sample::new).collect();
        let buffer = Buffer::from_slice(&samples);
        let reader = BufferReader::new(buffer);
        Ok(PyNode(self.0.add_processor(reader)))
    }

    pub fn message(&self, message: Bound<PyAny>) -> PyResult<PyNode> {
        if let Ok(message) = message.extract::<PyMessage>() {
            Ok(PyNode(self.0.message(message)))
        } else if message.extract::<PyBang>().is_ok() {
            Ok(PyNode(self.0.message(Message::Bang)))
        } else if let Ok(message) = message.extract::<f64>() {
            Ok(PyNode(self.0.message(message)))
        } else if let Ok(message) = message.extract::<i64>() {
            Ok(PyNode(self.0.message(message)))
        } else if let Ok(message) = message.extract::<String>() {
            Ok(PyNode(self.0.message(message)))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "message must be f64, i64, or str",
            ))
        }
    }

    pub fn constant_message(&self, message: Bound<PyAny>) -> PyResult<PyNode> {
        if let Ok(message) = message.extract::<PyMessage>() {
            Ok(PyNode(self.0.constant_message(message)))
        } else if message.extract::<PyBang>().is_ok() {
            Ok(PyNode(self.0.constant_message(Message::Bang)))
        } else if let Ok(message) = message.extract::<f64>() {
            Ok(PyNode(self.0.constant_message(message)))
        } else if let Ok(message) = message.extract::<i64>() {
            Ok(PyNode(self.0.constant_message(message)))
        } else if let Ok(message) = message.extract::<String>() {
            Ok(PyNode(self.0.constant_message(message)))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "message must be f64, i64, or str",
            ))
        }
    }

    pub fn register(&self) -> PyResult<PyNode> {
        Ok(PyNode(self.0.register()))
    }

    pub fn metro(&self) -> PyResult<PyNode> {
        Ok(PyNode(self.0.metro()))
    }

    pub fn select(&self, num_outputs: usize) -> PyResult<PyNode> {
        Ok(PyNode(self.0.select(num_outputs)))
    }

    pub fn merge(&self, num_inputs: usize) -> PyResult<PyNode> {
        Ok(PyNode(self.0.merge(num_inputs)))
    }

    pub fn counter(&self) -> PyResult<PyNode> {
        Ok(PyNode(self.0.counter()))
    }

    pub fn noise_osc(&self) -> PyResult<PyNode> {
        Ok(PyNode(self.0.noise_osc()))
    }

    pub fn sample_and_hold(&self) -> PyResult<PyNode> {
        Ok(PyNode(self.0.sample_and_hold()))
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

    pub fn param(&self, node: &PyParam) -> PyResult<PyNode> {
        Ok(PyNode(self.0.param(&node.0)))
    }
}
