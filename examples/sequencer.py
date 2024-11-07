import raug
import time
import math
from typing import List, Tuple
from example_utils import repl
from envelope import decay_env


def sequencer(graph: raug.GraphBuilder, trig: raug.Output, values: List[raug.Node]) -> Tuple[raug.Node, ...]:
    counter = graph.counter()
    trig.connect(counter.input(0))
    counter = (counter - 1) % len(values)

    select = graph.select(len(values))
    merge = graph.merge(len(values))

    counter.output(0).connect(select.input(1))
    select.input(0).set(graph.constant_message(raug.Bang()))

    registers = [graph.register() for _ in values]
    messages = [graph.message(raug.Bang()) for _ in values]
    for i, (value, reg, message) in enumerate(zip(values, registers, messages)):
        reg.input(0).set(value)
        message.input(0).connect(select.output(i))
        message.input(1).connect(reg.output(0))

    for i, message in enumerate(messages):
        merge.input(i).connect(message.output(0))

    return merge, counter


if __name__ == "__main__":
    graph = raug.GraphBuilder()

    out1 = graph.add_output()
    out2 = graph.add_output()

    sr = graph.sample_rate()
    phase = graph.phase_accum()

    amp_param = raug.Param("amp")
    amp_param.set(0.2)
    amp = graph.add_param(amp_param).smooth()

    decay = raug.Param("decay")
    decay.set(0.05)
    decay = graph.add_param(decay).smooth()

    rate = raug.Param("rate")
    rate.set(0.125)
    rate = graph.add_param(rate).smooth()

    trig = graph.metro()
    trig.input(0).connect(rate.output(0))

    values = [440.0, 660.0, 880.0, 1100.0]
    freq_params = [raug.Param(f"freq{i}") for i in range(len(values))]
    freqs = [graph.add_param(freq) for freq in freq_params]
    for freq_param, value in zip(freq_params, values):
        freq_param.set(value)
    freq, counter = sequencer(graph, trig.output(0), freqs)

    increment = freq / sr
    phase.input("increment").connect(increment.output(0))
    sine = (phase * 2.0 * math.pi).sin()

    amp = decay_env(graph, trig, decay) * amp

    sine = sine * amp

    sine.output(0).connect(out1.input(0))
    sine.output(0).connect(out2.input(0))

    runtime = graph.build_runtime()
    handle = runtime.run()

    repl(runtime)

    handle.stop()
