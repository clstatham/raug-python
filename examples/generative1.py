import raug
from typing import List
from example_utils import *
from envelope import decay_env


def random_tones(graph: raug.GraphBuilder, name: str, rate_float: float, modfreqs_float: List[float], freqs_float: List[float], decays_float: List[float]) -> raug.Node:
    # create a master metronome to drive the random selection
    master = graph.metro()
    rate = master.input(0).param(f"rate_{name}")
    rate.set(rate_float)

    # select a random frequency
    freq = pick_randomly(
        graph, master, [graph.constant(f) for f in freqs_float])

    # select a random decay
    decay = pick_randomly(
        graph, master, [graph.constant(d) for d in decays_float])

    # select a random mod frequency
    modfreq = pick_randomly(
        graph, master, [graph.constant(m) for m in modfreqs_float])

    # create the envelope
    env = decay_env(graph, master, decay)

    # create the modulating oscillator
    mod = fm_sine_osc(graph, modfreq, graph.constant(0.0))

    # create the carrier oscillator
    osc = fm_sine_osc(graph, freq, mod)

    return osc * env


if __name__ == "__main__":
    # try changing these values!
    modfreqs_float = [440.0, 600.0, 880.0, 1000.0]
    freqs_float = [440.0, 220.0, 660.0, 330.0]
    decays_float = [0.05, 0.1, 0.2, 0.3]
    rates_float = [0.125, 0.25, 0.5, 1.0]

    modfreqs_float.sort()
    freqs_float.sort()
    decays_float.sort()
    rates_float.sort()

    graph = raug.GraphBuilder()

    out1 = graph.add_output()
    out2 = graph.add_output()

    amp_param = raug.Param("amp")
    amp_param.set(0.2)
    amp = graph.add_param(amp_param)

    sine1 = random_tones(graph, "tone1", 0.125, modfreqs_float,
                         freqs_float, decays_float)

    sine2 = random_tones(graph, "tone2", 0.25, modfreqs_float,
                         freqs_float, decays_float)

    sine3 = random_tones(graph, "tone3", 0.5, modfreqs_float,
                         freqs_float, decays_float)

    oscs = sine1 + sine2 + sine3

    mix = oscs * amp

    master = graph.peak_limiter()
    master.input(0).connect(mix.output(0))

    master.output(0).connect(out1.input(0))
    master.output(0).connect(out2.input(0))

    runtime = graph.build_runtime()
    handle = runtime.run()

    repl(runtime)

    handle.stop()
