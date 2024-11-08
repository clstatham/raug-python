use pyo3::prelude::*;
use raug::prelude::*;

#[pyclass(name = "Message")]
#[derive(Clone)]
pub enum PyMessage {
    Bang(PyBang),
    Float(f64),
    Int(i64),
    String(String),
}

#[pymethods]
impl PyMessage {
    pub fn __repr__(&self) -> String {
        match self {
            PyMessage::Bang(_) => "Bang".to_string(),
            PyMessage::Float(float) => format!("Float({})", float),
            PyMessage::Int(int) => format!("Int({})", int),
            PyMessage::String(string) => format!("String({})", string),
        }
    }
}

impl TryFrom<Message> for PyMessage {
    type Error = PyErr;
    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message {
            Message::Bang => Ok(PyMessage::Bang(PyBang)),
            Message::Float(float) => Ok(PyMessage::Float(float)),
            Message::Int(int) => Ok(PyMessage::Int(int)),
            Message::String(string) => Ok(PyMessage::String(string.to_string())),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "unsupported message type",
            )),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Message> for PyMessage {
    fn into(self) -> Message {
        match self {
            PyMessage::Bang(_) => Message::Bang,
            PyMessage::Float(float) => Message::Float(float),
            PyMessage::Int(int) => Message::Int(int),
            PyMessage::String(string) => Message::from(string),
        }
    }
}

pub trait PyMessageExt {
    fn try_from_pyany(message: Bound<PyAny>) -> PyResult<Self>
    where
        Self: Sized;

    fn try_to_pyobject(&self, py: Python) -> PyResult<PyObject>
    where
        Self: Sized;
}

impl PyMessageExt for Message {
    fn try_from_pyany(message: Bound<PyAny>) -> PyResult<Self> {
        if let Ok(message) = message.extract::<PyMessage>() {
            Ok(message.into())
        } else if message.extract::<PyBang>().is_ok() {
            Ok(Message::Bang)
        } else if let Ok(message) = message.extract::<f64>() {
            Ok(Message::Float(message))
        } else if let Ok(message) = message.extract::<i64>() {
            Ok(Message::Int(message))
        } else if let Ok(message) = message.extract::<String>() {
            Ok(Message::String(message))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "message must be Bang, f64, i64, or str",
            ))
        }
    }

    fn try_to_pyobject(&self, py: Python) -> PyResult<PyObject>
    where
        Self: Sized,
    {
        match self {
            Message::Bang => Ok(PyBang {}.into_py(py)),
            Message::Float(float) => Ok(float.to_object(py)),
            Message::Int(int) => Ok(int.to_object(py)),
            Message::String(string) => Ok(string.to_object(py)),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "unsupported message type",
            )),
        }
    }
}

#[pyclass(name = "Bang")]
#[derive(Clone)]
pub struct PyBang;

#[pymethods]
impl PyBang {
    #[new]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        PyBang
    }

    pub fn __repr__(&self) -> String {
        "Bang".to_string()
    }
}
