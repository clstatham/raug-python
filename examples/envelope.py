import raug
import math


def decay_env(graph, trig, decay):
    sr = graph.sample_rate()
    time = graph.phase_accum()
    time.input(0).connect(sr.recip().output(0))
    time.input(1).connect(trig.output(0))

    env = (-time + 1.0) ** decay.recip()
    env = env.smooth()

    return env


if __name__ == "__main__":
    graph = raug.GraphBuilder()

    out1 = graph.add_output()
    out2 = graph.add_output()

    decay1 = raug.Param()
    decay1.set(0.05)

    decay2 = raug.Param()
    decay2.set(0.1)

    trig = graph.metro()
    trig.input(0).set(0.5)

    amp1 = decay_env(graph, trig, graph.param(decay1))
    amp2 = decay_env(graph, trig, graph.param(decay2))

    sr = graph.sample_rate()
    pa = graph.phase_accum()
    pa.input(0).connect(sr.recip().output(0))
    pa = pa % 1.0

    freq1 = raug.Param()
    freq1.set(880.0)

    freq2 = raug.Param()
    freq2.set(220.0)

    sine1 = (pa * freq1 * 2.0 * math.pi).sin() * amp1
    sine2 = (pa * freq2 * 2.0 * math.pi + sine1).sin()

    final = sine2 * amp2

    final.output(0).connect(out1.input(0))
    final.output(0).connect(out2.input(0))

    runtime = graph.build_runtime()
    handle = runtime.run()

    input("Press Enter to stop...")
    handle.stop()
