import raug
from typing import List
from example_utils import *
from envelope import decay_env


def midi_to_freq(midi: float) -> float:
    return 440.0 * 2.0 ** ((midi - 69.0) / 12.0)


def scale_freqs() -> List[float]:
    # C minor scale
    scale = [60, 62, 63, 65, 67, 68, 70, 72]
    # C major scale
    # scale = [60, 62, 64, 65, 67, 69, 71, 72]
    scale = [m - 12 for m in scale] + scale + [m + 12 for m in scale]
    return [midi_to_freq(m) for m in scale]


def random_tones(graph: raug.GraphBuilder, rates_float: List[float], ratios_float: List[float], freqs_float: List[float], decays_float: List[float], amps_float: List[float]) -> raug.Node:
    mast = graph.metro()
    mast.input(0).set(rates_float[0])

    # select a random rate
    rate = pick_randomly(
        graph, mast, [graph.constant(r) for r in rates_float])

    trig = graph.metro()
    trig.input(0).connect(rate.output(0))

    # select a random frequency
    freq = pick_randomly(
        graph, trig, [graph.constant(f) for f in freqs_float])

    # select a random decay
    decay = pick_randomly(
        graph, trig, [graph.constant(d) for d in decays_float])

    # select a random mod frequency
    ratio = pick_randomly(
        graph, trig, [graph.constant(m) for m in ratios_float])

    # select a random amplitude
    amp = pick_randomly(
        graph, trig, [graph.constant(a) for a in amps_float])

    # create the envelope
    env = decay_env(graph, trig, decay)

    # create the modulating oscillator
    mod = graph.bl_saw_osc()
    mod.input(0).connect((ratio * freq).output(0))

    # create the carrier oscillator
    osc = fm_sine_osc(graph, freq, mod * 0.1)

    return osc * env * amp


if __name__ == "__main__":
    # try changing these values!
    freqs_float = scale_freqs()
    ratios_float = [0.5, 1.0, 2.0, 3.0, 4.0, 5.0]
    decays_float = [0.05, 0.1, 0.2, 0.3]
    amps_float = [0.125, 0.25, 0.5, 0.8]
    rates_float = [0.125, 0.25, 0.5, 1.0]

    graph = raug.GraphBuilder()

    out1 = graph.add_output()
    out2 = graph.add_output()

    amp = graph.add_param(raug.Param("amp", 0.5))

    sine1 = random_tones(graph, rates_float, ratios_float,
                         freqs_float, decays_float, amps_float)

    sine2 = random_tones(graph, rates_float, ratios_float,
                         freqs_float, decays_float, amps_float)

    sine3 = random_tones(graph, rates_float, ratios_float,
                         freqs_float, decays_float, amps_float)

    oscs = sine1 + sine2 + sine3

    mix = oscs * amp

    master = graph.peak_limiter()
    master.input(0).connect(mix.output(0))

    master.output(0).connect(out1.input(0))
    master.output(0).connect(out2.input(0))

    runtime = graph.build_runtime()

    runtime.run_offline_to_file("target/generative1.wav", 60.0 * 5.0)

    handle = runtime.run()

    repl(runtime)

    handle.stop()
