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
    # C pentatonic scale
    # scale = [60, 62, 64, 67, 69, 72]
    # shift by 3 semitones to start on E
    scale = [m + 3 for m in scale]
    scale = [m - 12 for m in scale] + scale + \
        [m + 12 for m in scale] + [m + 24 for m in scale]
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
    amp_decay = pick_randomly(
        graph, trig, [graph.constant(d) for d in decays_float])

    # select a random mod ratio
    ratio = pick_randomly(
        graph, trig, [graph.constant(m) for m in ratios_float])

    # select a random amplitude
    amp = pick_randomly(
        graph, trig, [graph.constant(a) for a in amps_float])

    # create the amplitude envelope
    amp_env = decay_env(graph, trig, amp_decay)

    # select a random decay
    filt_decay = pick_randomly(
        graph, trig, [graph.constant(d) for d in decays_float])

    # create the filter envelope
    filt_env = decay_env(graph, trig, filt_decay)

    # select a random scale
    scales = [0.25, 0.5, 1.0]
    scale = pick_randomly(
        graph, trig, [graph.constant(s) for s in scales])

    # scale the filter envelope from [0, 1] to [200, 20000]
    filt_env = filt_env * 19800 * scale + 200

    # create the modulating oscillator
    mod = graph.bl_saw_osc()
    mod.input(0).connect((ratio * freq).output(0))

    # create the carrier oscillator
    osc = fm_sine_osc(graph, freq, mod * 0.1)

    # create the filter
    filt = graph.moog_ladder()
    filt.input("in").connect(osc.output(0))
    filt.input("cutoff").connect(filt_env.output(0))
    filt.input("resonance").set(0.1)

    return filt * amp_env * amp


if __name__ == "__main__":
    # try changing these values!
    num_tones = 12
    freqs_float = scale_freqs()
    ratios_float = [0.25, 0.5, 1.0, 2.0]
    decays_float = [0.1, 0.1, 0.2, 0.5]
    amps_float = [0.125, 0.25, 0.5, 0.8]
    rates_float = [1./8, 1./4, 1./2, 1.0, 2.0]

    graph = raug.GraphBuilder()

    out1 = graph.add_output()
    out2 = graph.add_output()

    amp = graph.add_param(raug.Param("amp", 0.5))

    tones = [random_tones(graph, rates_float, ratios_float,
                          freqs_float, decays_float, amps_float) for _ in range(num_tones)]

    oscs = tones[0]
    for tone in tones[1:]:
        oscs = oscs + tone

    mix = oscs * amp

    master = graph.peak_limiter()
    master.input(0).connect(mix.output(0))

    master.output(0).connect(out1.input(0))
    master.output(0).connect(out2.input(0))

    runtime = graph.build_runtime()

    # runtime.run_offline_to_file("target/generative1_6.wav", 60.0)

    handle = runtime.run()

    repl(runtime)

    handle.stop()
