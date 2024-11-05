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

impl TryFrom<Message> for PyMessage {
    type Error = PyErr;
    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message {
            Message::Bang => Ok(PyMessage::Bang(PyBang)),
            Message::Float(float) => Ok(PyMessage::Float(float)),
            Message::Int(int) => Ok(PyMessage::Int(int)),
            Message::String(string) => Ok(PyMessage::String(string)),
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
            PyMessage::String(string) => Message::String(string),
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
}
