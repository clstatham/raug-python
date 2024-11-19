use pyo3::prelude::*;
use raug::prelude::*;

#[pyclass(name = "Signal")]
pub struct PySignal(pub(crate) AnySignal);

#[pymethods]
impl PySignal {
    #[new]
    pub fn new(value: Bound<PyAny>) -> PyResult<Self> {
        if let Ok(value) = value.extract::<f64>() {
            Ok(PySignal(AnySignal::Float(Some(value))))
        } else if let Ok(value) = value.extract::<bool>() {
            Ok(PySignal(AnySignal::Bool(Some(value))))
        } else if let Ok(value) = value.extract::<i64>() {
            Ok(PySignal(AnySignal::Int(Some(value))))
        } else if let Ok(value) = value.extract::<String>() {
            Ok(PySignal(AnySignal::String(Some(value))))
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err("Invalid type"))
        }
    }
}

impl PySignal {
    pub fn into_inner(self) -> AnySignal {
        self.0
    }

    pub fn try_to_pyobject(&self, py: Python) -> PyResult<PyObject> {
        match &self.0 {
            AnySignal::Float(Some(f)) => Ok(f.to_object(py)),
            AnySignal::Int(Some(i)) => Ok(i.to_object(py)),
            AnySignal::Bool(Some(b)) => Ok(b.to_object(py)),
            AnySignal::String(Some(s)) => Ok(s.to_object(py)),
            _ => Err(pyo3::exceptions::PyTypeError::new_err("Invalid type")),
        }
    }
}
