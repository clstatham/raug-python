use pyo3::prelude::*;
use raug::prelude::*;

#[pyclass(name = "Message")]
pub enum PyMessage {
    Bang(),
    Float(f64),
    Int(i64),
    String(String),
}

impl TryFrom<Message> for PyMessage {
    type Error = PyErr;
    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message {
            Message::Bang => Ok(PyMessage::Bang()),
            Message::Float(float) => Ok(PyMessage::Float(float)),
            Message::Int(int) => Ok(PyMessage::Int(int)),
            Message::String(string) => Ok(PyMessage::String(string)),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "unsupported message type",
            )),
        }
    }
}
