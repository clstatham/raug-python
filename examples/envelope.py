import raug
import math
from example_utils import repl


def decay_env(graph: raug.GraphBuilder, trig: raug.Node, decay: raug.Node) -> raug.Node:
    sr = graph.sample_rate()
    time = graph.phase_accum()
    time.input(0).connect(sr.recip().output(0))
    time.input(1).connect(trig.output(0))

    time = time % 1.0

    env = (-time + 1.0) ** decay.recip()
    env = env.smooth()

    return env


if __name__ == "__main__":
    graph = raug.GraphBuilder()

    sr = graph.sample_rate()

    out1 = graph.add_audio_output()
    out2 = graph.add_audio_output()

    rate = raug.Param("rate", 0.5)
    rate = graph.add_param(rate).smooth()

    decay1 = raug.Param("decay1", 0.05)
    decay1 = graph.add_param(decay1).smooth()

    decay2 = raug.Param("decay2", 0.1)
    decay2 = graph.add_param(decay2).smooth()

    freq1 = raug.Param("freq1", 440.0)
    freq1 = graph.add_param(freq1).smooth()

    freq2 = raug.Param("freq2", 220.0)
    freq2 = graph.add_param(freq2).smooth()

    trig = graph.metro()
    trig.input(0).connect(rate.output(0))

    amp1 = decay_env(graph, trig, decay1)
    amp2 = decay_env(graph, trig, decay2)

    pa1 = graph.phase_accum()
    pa1.input(0).connect((freq1 / sr).output(0))
    pa1 = pa1 % 1.0

    pa2 = graph.phase_accum()
    pa2.input(0).connect((freq2 / sr).output(0))
    pa2 = pa2 % 1.0

    sine1 = (pa1 * 2.0 * math.pi).sin() * amp1
    sine2 = (pa2 * 2.0 * math.pi + sine1).sin()

    final = sine2 * amp2 * 0.2

    final.output(0).connect(out1.input(0))
    final.output(0).connect(out2.input(0))

    runtime = graph.build_runtime()
    handle = runtime.run()

    repl(runtime)

    handle.stop()
