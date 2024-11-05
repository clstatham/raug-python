import raug
import time
import math
from typing import List


def decay_env(graph: raug.GraphBuilder, trig: raug.Output, decay: raug.Node) -> raug.Node:
    sr = graph.sample_rate()
    time = graph.phase_accum()
    time.input(0).connect(sr.recip().output(0))
    time.input(1).connect(trig)

    env = (-time + 1.0) ** decay.recip()
    env = env.smooth()

    return env


def sequencer(graph: raug.GraphBuilder, trig: raug.Output, values: List[float]) -> raug.Node:
    counter = graph.counter()
    trig.connect(counter.input(0))
    counter = (counter - 1) % len(values)

    select = graph.select(len(values))
    merge = graph.merge(len(values))

    counter.output(0).connect(select.input(1))
    select.input(0).set(graph.constant_message(raug.Bang()))

    messages = [graph.message(value) for value in values]

    for i, value in enumerate(messages):
        value.input(0).connect(select.output(i))
        merge.input(i).connect(value.output(0))

    return merge


if __name__ == "__main__":
    graph = raug.GraphBuilder()

    out1 = graph.add_output()
    out2 = graph.add_output()

    sr = graph.sample_rate()
    phase = graph.phase_accum()

    freq_param = raug.Param()
    freq_param.set(440.0)

    amp_param = raug.Param()
    amp_param.set(0.2)

    decay = raug.Param()
    decay.set(0.05)

    freq = graph.param(freq_param).smooth()
    amp = graph.param(amp_param).smooth()

    trig = graph.metro()
    trig.input(0).set(0.125)

    values = [440.0, 660.0, 880.0, 1100.0]
    freq = sequencer(graph, trig.output(0), values)

    increment = freq / sr
    phase.input("increment").connect(increment.output(0))
    sine = (phase * 2.0 * math.pi).sin()

    amp = decay_env(graph, trig.output(0), graph.param(decay)) * amp

    sine = sine * amp

    sine.output(0).connect(out1.input(0))
    sine.output(0).connect(out2.input(0))

    runtime = graph.build_runtime()
    handle = runtime.run()

    time.sleep(1.0)

    handle.stop()
