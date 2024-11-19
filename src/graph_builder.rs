use std::fs::File;

use dynamics::PeakLimiter;
use pyo3::prelude::*;
use raug::prelude::*;

use crate::{
    graph::PyGraph,
    message::PySignal,
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

    pub fn add_audio_input(&self) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add_audio_input()))
    }

    pub fn add_audio_output(&self) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add_audio_output()))
    }

    pub fn add_midi_input(&self, name: &str) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add_midi_input(name)))
    }

    pub fn write_dot(&self, path: &str) -> PyResult<()> {
        self.0.write_dot(&mut File::create(path)?).unwrap();
        Ok(())
    }

    pub fn print(&self) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add(Print::new(SignalType::Float))))
    }

    pub fn sample_rate(&self) -> PyResult<PyNode> {
        Ok(PyNode(self.0.sample_rate()))
    }

    pub fn phase_accum(&self) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add(PhaseAccumulator::default())))
    }

    #[pyo3(signature = (frequency=440.0))]
    pub fn sine_osc(&self, frequency: Float) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add(SineOscillator::new(frequency))))
    }

    #[pyo3(signature = (frequency=440.0))]
    pub fn saw_osc(&self, frequency: Float) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add(SawOscillator::new(frequency))))
    }

    #[pyo3(signature = (frequency=440.0))]
    pub fn bl_saw_osc(&self, frequency: Float) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add(BlSawOscillator::new(frequency))))
    }

    #[pyo3(signature = (frequency=440.0, pulse_width=0.5))]
    pub fn bl_square_osc(&self, frequency: Float, pulse_width: Float) -> PyResult<PyNode> {
        Ok(PyNode(
            self.0.add(BlSquareOscillator::new(frequency, pulse_width)),
        ))
    }

    pub fn constant(&self, value: Float) -> PyResult<PyNode> {
        Ok(PyNode(self.0.constant(value)))
    }

    pub fn add_param(&self, node: &PyParam) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add_param(node.0.clone())))
    }

    pub fn load_buffer(&self, path: &str) -> PyResult<PyNode> {
        let buffer = Buffer::load_wav(path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let buffer = AudioBuffer::new(buffer);
        Ok(PyNode(self.0.add(buffer)))
    }

    pub fn buffer(&self, contents: Vec<Float>) -> PyResult<PyNode> {
        let samples: Vec<_> = contents.into_iter().collect();
        let buffer = Buffer::from_slice(&samples);
        let buffer = AudioBuffer::new(buffer);
        Ok(PyNode(self.0.add(buffer)))
    }

    pub fn message(&self, message: Bound<PyAny>) -> PyResult<PyNode> {
        let message = PySignal::new(message)?;
        Ok(PyNode(self.0.add(Message::new_any(message.into_inner()))))
    }

    #[pyo3(signature = (period=1.0))]
    pub fn metro(&self, period: Float) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add(Metro::new(period))))
    }

    pub fn counter(&self) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add(Counter::default())))
    }

    pub fn noise_osc(&self) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add(NoiseOscillator)))
    }

    pub fn sample_and_hold(&self) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add(SampleAndHold::default())))
    }

    pub fn change_detector(&self) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add(Changed::default())))
    }

    #[pyo3(signature = (context="<unknown>"))]
    pub fn check_finite(&self, context: &str) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add(CheckFinite::new(context))))
    }

    #[pyo3(signature = (threshold=1.0, attack=0.01, release=0.1))]
    pub fn peak_limiter(
        &self,
        threshold: Float,
        attack: Float,
        release: Float,
    ) -> PyResult<PyNode> {
        let mut processor = PeakLimiter::default();
        processor.threshold = threshold;
        processor.attack = attack;
        processor.release = release;
        Ok(PyNode(self.0.add(processor)))
    }

    #[pyo3(signature = (cutoff=1000.0, resonance=0.1))]
    pub fn moog_ladder(&self, cutoff: Float, resonance: Float) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add(MoogLadder::new(cutoff, resonance))))
    }

    #[pyo3(signature = (cutoff=1000.0, q=0.1))]
    pub fn biquad_lowpass(&self, cutoff: Float, q: Float) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add(AutoBiquad::lowpass(cutoff, q))))
    }

    #[pyo3(signature = (cutoff=1000.0, q=0.1))]
    pub fn biquad_highpass(&self, cutoff: Float, q: Float) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add(AutoBiquad::highpass(cutoff, q))))
    }

    #[pyo3(signature = (cutoff=1000.0, q=0.1))]
    pub fn biquad_bandpass(&self, cutoff: Float, q: Float) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add(AutoBiquad::bandpass(cutoff, q))))
    }

    #[pyo3(signature = (cutoff=1000.0, q=0.1))]
    pub fn biquad_notch(&self, cutoff: Float, q: Float) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add(AutoBiquad::notch(cutoff, q))))
    }

    #[pyo3(signature = (cutoff=1000.0, q=0.1, gain=0.0))]
    pub fn biquad_peak(&self, cutoff: Float, q: Float, gain: Float) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add(AutoBiquad::peak(cutoff, q, gain))))
    }

    #[pyo3(signature = (cutoff=1000.0, q=0.1, gain=0.0))]
    pub fn biquad_lowshelf(&self, cutoff: Float, q: Float, gain: Float) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add(AutoBiquad::low_shelf(cutoff, q, gain))))
    }

    #[pyo3(signature = (cutoff=1000.0, q=0.1, gain=0.0))]
    pub fn biquad_highshelf(&self, cutoff: Float, q: Float, gain: Float) -> PyResult<PyNode> {
        Ok(PyNode(self.0.add(AutoBiquad::high_shelf(cutoff, q, gain))))
    }
}
