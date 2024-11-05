use pyo3::prelude::*;
use raug::prelude::*;

use crate::message::PyMessage;

#[derive(Clone)]
#[pyclass(name = "Node")]
pub struct PyNode(pub(crate) Node);

#[pymethods]
impl PyNode {
    pub fn input(&self, index: Bound<PyAny>) -> PyResult<PyInput> {
        if let Ok(index) = index.extract::<u32>() {
            Ok(PyInput(self.0.input(index)))
        } else if let Ok(index) = index.extract::<String>() {
            Ok(PyInput(self.0.input(&*index)))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "index must be u32 or str",
            ))
        }
    }

    pub fn output(&self, index: Bound<PyAny>) -> PyResult<PyOutput> {
        if let Ok(index) = index.extract::<u32>() {
            Ok(PyOutput(self.0.output(index)))
        } else if let Ok(index) = index.extract::<String>() {
            Ok(PyOutput(self.0.output(&*index)))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "index must be u32 or str",
            ))
        }
    }

    pub fn __add__(&self, other: Bound<PyAny>) -> PyResult<PyNode> {
        if let Ok(other) = other.extract::<PyNode>() {
            Ok(PyNode(self.0.clone() + other.0.clone()))
        } else if let Ok(other) = other.extract::<PyParam>() {
            Ok(PyNode(self.0.clone() + other.0.clone()))
        } else if let Ok(other) = other.extract::<f64>() {
            Ok(PyNode(self.0.clone() + other))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "unsupported operand type(s) for +: 'Node' and 'other'",
            ))
        }
    }

    pub fn __sub__(&self, other: Bound<PyAny>) -> PyResult<PyNode> {
        if let Ok(other) = other.extract::<PyNode>() {
            Ok(PyNode(self.0.clone() - other.0.clone()))
        } else if let Ok(other) = other.extract::<PyParam>() {
            Ok(PyNode(self.0.clone() - other.0.clone()))
        } else if let Ok(other) = other.extract::<f64>() {
            Ok(PyNode(self.0.clone() - other))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "unsupported operand type(s) for -: 'Node' and 'other'",
            ))
        }
    }

    pub fn __mul__(&self, other: Bound<PyAny>) -> PyResult<PyNode> {
        if let Ok(other) = other.extract::<PyNode>() {
            Ok(PyNode(self.0.clone() * other.0.clone()))
        } else if let Ok(other) = other.extract::<PyParam>() {
            Ok(PyNode(self.0.clone() * other.0.clone()))
        } else if let Ok(other) = other.extract::<f64>() {
            Ok(PyNode(self.0.clone() * other))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "unsupported operand type(s) for *: 'Node' and 'other'",
            ))
        }
    }

    pub fn __truediv__(&self, other: &Bound<PyAny>) -> PyResult<PyNode> {
        if let Ok(other) = other.extract::<PyNode>() {
            Ok(PyNode(self.0.clone() / other.0.clone()))
        } else if let Ok(other) = other.extract::<PyParam>() {
            Ok(PyNode(self.0.clone() * other.0.clone()))
        } else if let Ok(other) = other.extract::<f64>() {
            Ok(PyNode(self.0.clone() / other))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "unsupported operand type(s) for /: 'Node' and 'other'",
            ))
        }
    }

    pub fn __pow__(&self, other: &Bound<PyAny>, _modulus: &Bound<PyAny>) -> PyResult<PyNode> {
        if let Ok(other) = other.extract::<PyNode>() {
            Ok(PyNode(self.0.clone().powf(other.0.clone())))
        } else if let Ok(other) = other.extract::<PyParam>() {
            Ok(PyNode(self.0.clone() * other.0.clone()))
        } else if let Ok(other) = other.extract::<f64>() {
            Ok(PyNode(self.0.clone().powf(other)))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "unsupported operand type(s) for ^: 'Node' and 'other'",
            ))
        }
    }

    pub fn __mod__(&self, other: &Bound<PyAny>) -> PyResult<PyNode> {
        if let Ok(other) = other.extract::<PyNode>() {
            Ok(PyNode(self.0.clone() % other.0.clone()))
        } else if let Ok(other) = other.extract::<PyParam>() {
            Ok(PyNode(self.0.clone() % other.0.clone()))
        } else if let Ok(other) = other.extract::<f64>() {
            Ok(PyNode(self.0.clone() % other))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "unsupported operand type(s) for %: 'Node' and 'other'",
            ))
        }
    }

    pub fn __neg__(&self) -> PyNode {
        PyNode(self.0.neg())
    }

    pub fn sin(&self) -> PyNode {
        PyNode(self.0.sin())
    }

    pub fn cos(&self) -> PyNode {
        PyNode(self.0.cos())
    }

    pub fn tan(&self) -> PyNode {
        PyNode(self.0.tan())
    }

    pub fn asin(&self) -> PyNode {
        PyNode(self.0.asin())
    }

    pub fn acos(&self) -> PyNode {
        PyNode(self.0.acos())
    }

    pub fn atan(&self) -> PyNode {
        PyNode(self.0.atan())
    }

    pub fn atan2(&self, other: &PyNode) -> PyNode {
        PyNode(self.0.atan2(other.0.clone()))
    }

    pub fn recip(&self) -> PyNode {
        PyNode(self.0.recip())
    }

    pub fn smooth(&self) -> PyNode {
        PyNode(self.0.smooth())
    }
}

#[pyclass(name = "Input")]
pub struct PyInput(pub(crate) Input);

#[pymethods]
impl PyInput {
    pub fn set(&self, value: Bound<PyAny>) -> PyResult<()> {
        if let Ok(value) = value.extract::<PyNode>() {
            self.0.set(value.0.clone());
            Ok(())
        } else if let Ok(value) = value.extract::<f64>() {
            self.0.set(value);
            Ok(())
        } else if let Ok(value) = value.extract::<i64>() {
            self.0.set(Message::Int(value));
            Ok(())
        } else if let Ok(value) = value.extract::<String>() {
            self.0.set(Message::String(value));
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "value must be f64, i64, or str",
            ))
        }
    }

    pub fn param(&self) -> PyParam {
        PyParam(self.0.param())
    }

    pub fn connect(&self, node: Bound<PyOutput>) -> PyResult<()> {
        self.0.connect(&node.borrow().0);
        Ok(())
    }
}

#[pyclass(name = "Output")]
pub struct PyOutput(pub(crate) Output);

#[pymethods]
impl PyOutput {
    pub fn connect(&self, node: Bound<PyInput>) -> PyResult<()> {
        self.0.connect(&node.borrow().0);
        Ok(())
    }

    pub fn node(&self) -> PyNode {
        PyNode(self.0.node())
    }
}

#[derive(Clone)]
#[pyclass(name = "Param")]
pub struct PyParam(pub(crate) Param);

#[pymethods]
impl PyParam {
    #[new]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        PyParam(Param::new())
    }

    pub fn set(&self, value: Bound<PyAny>) -> PyResult<()> {
        if let Ok(value) = value.extract::<f64>() {
            self.0.set(value);
            Ok(())
        } else if let Ok(value) = value.extract::<i64>() {
            self.0.set(value);
            Ok(())
        } else if let Ok(value) = value.extract::<String>() {
            self.0.set(&*value);
            Ok(())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "value must be f64, i64, or str",
            ))
        }
    }

    pub fn get(&mut self) -> PyResult<Option<PyMessage>> {
        let message = self.0.get();
        match message {
            Some(message) => Ok(Some(PyMessage::try_from(message)?)),
            None => Ok(None),
        }
    }
}
