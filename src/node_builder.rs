use pyo3::prelude::*;
use raug::prelude::*;

use crate::message::PyMessageExt;

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

    pub fn make_register(&self) -> PyNode {
        PyNode(self.0.make_register())
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

    pub fn atan2(&self, other: &PyNode) -> PyNode {
        PyNode(self.0.atan2(other.0.clone()))
    }

    pub fn recip(&self) -> PyNode {
        PyNode(self.0.recip())
    }

    pub fn smooth(&self) -> PyNode {
        PyNode(self.0.smooth())
    }

    pub fn midi2freq(&self) -> PyNode {
        PyNode(self.0.midi2freq())
    }

    pub fn freq2midi(&self) -> PyNode {
        PyNode(self.0.freq2midi())
    }

    pub fn floor(&self) -> PyNode {
        PyNode(self.0.floor())
    }

    pub fn ceil(&self) -> PyNode {
        PyNode(self.0.ceil())
    }

    pub fn round(&self) -> PyNode {
        PyNode(self.0.round())
    }
}

#[pyclass(name = "Input")]
pub struct PyInput(pub(crate) Input);

#[pymethods]
impl PyInput {
    pub fn set(&self, value: Bound<PyAny>) -> PyResult<()> {
        let message = Message::try_from_pyany(value)?;
        self.0.set(message);
        Ok(())
    }

    pub fn param(&self, name: String, initial_value: Bound<PyAny>) -> PyResult<PyParam> {
        let initial_value = Message::try_from_pyany(initial_value)?;
        Ok(PyParam(self.0.param(name, Some(initial_value))))
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

    pub fn make_node(&self) -> PyNode {
        PyNode(self.0.make_node())
    }

    pub fn make_register(&self) -> PyNode {
        PyNode(self.0.make_register())
    }
}

#[derive(Clone)]
#[pyclass(name = "Param")]
pub struct PyParam(pub(crate) Param);

#[pymethods]
impl PyParam {
    #[new]
    #[allow(clippy::new_without_default)]
    pub fn new(name: String, initial_value: Bound<PyAny>) -> PyResult<Self> {
        let initial_value = Message::try_from_pyany(initial_value)?;
        Ok(Self(Param::new(name, Some(initial_value))))
    }

    pub fn set(&self, value: Bound<PyAny>) -> PyResult<()> {
        let message = Message::try_from_pyany(value)?;
        self.0.set(message);
        Ok(())
    }

    pub fn get(&mut self, py: Python) -> PyResult<PyObject> {
        let message = self.0.get();
        if let Some(message) = message {
            message.try_to_pyobject(py)
        } else {
            Ok(py.None())
        }
    }
}
