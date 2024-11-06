use pyo3::prelude::*;
use raug::prelude::*;

macro_rules! builtin {
    ($name:ident, $name_str:literal, $py_name:ident => $($new:tt)*) => {
        #[pyclass(name = $name_str)]
        #[derive(Clone)]
        pub struct $py_name(#[allow(unused)] pub(crate) $name);

        #[pymethods]
        impl $py_name {
            $($new)*

            pub fn __repr__(&self) -> String {
                stringify!($name).to_string()
            }
        }

        impl std::ops::Deref for $py_name {
            type Target = $name;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<$name> for $py_name {
            fn from(node: $name) -> Self {
                Self(node)
            }
        }
    };
}

builtin!(SineOscillator, "SineOscillator", PySineOscillator =>
#[new]
#[pyo3(signature = (frequency=440.0))]
pub fn new(frequency: f64) -> Self {
    PySineOscillator(SineOscillator::new(frequency))
});

builtin!(SawOscillator, "SawOscillator", PySawOscillator =>
#[new]
#[pyo3(signature = (frequency=440.0))]
pub fn new(frequency: f64) -> Self {
    PySawOscillator(SawOscillator::new(frequency))
});

builtin!(BlSawOscillator, "BlSawOscillator", PyBlSawOscillator =>
#[new]
#[pyo3(signature = (frequency=440.0))]
pub fn new(frequency: f64) -> Self {
    PyBlSawOscillator(BlSawOscillator::new(frequency))
});
