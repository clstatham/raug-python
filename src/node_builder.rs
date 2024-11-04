use pyo3::prelude::*;
use raug::prelude::*;

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
        } else if let Ok(other) = other.extract::<f64>() {
            Ok(PyNode(self.0.clone() * other))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "unsupported operand type(s) for *: 'Node' and 'other'",
            ))
        }
    }

    pub fn __div__(&self, other: &Bound<PyAny>) -> PyResult<PyNode> {
        if let Ok(other) = other.extract::<PyNode>() {
            Ok(PyNode(self.0.clone() / other.0.clone()))
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
        } else if let Ok(other) = other.extract::<f64>() {
            Ok(PyNode(self.0.clone().powf(other)))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "unsupported operand type(s) for ^: 'Node' and 'other'",
            ))
        }
    }

    pub fn __neg__(&self) -> PyNode {
        PyNode(self.0.neg())
    }
}

#[pyclass(name = "Input")]
pub struct PyInput(pub(crate) Input);

#[pymethods]
impl PyInput {
    pub fn set(&self, value: f64) {
        self.0.set(value);
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
}
