import os
import raug
from example_utils import *
from envelope import decay_env


def random_cmaj_note(graph: raug.GraphBuilder, trig: raug.Node) -> raug.Node:
    # C major scale
    scale = [60, 62, 64, 65, 67, 69, 71, 72]
    freq = pick_randomly(graph, trig, [graph.constant(f) for f in scale])
    return freq


if __name__ == "__main__":
    graph = raug.GraphBuilder()

    out1 = graph.add_output()
    out2 = graph.add_output()

    amp = raug.Param("amp")
    amp.set(0.2)
    amp = graph.param(amp)

    trig = graph.metro()
    trig.input(0).set(0.5)

    pitch = random_cmaj_note(graph, trig)

    path = os.path.join(os.path.dirname(__file__), "assets/piano1.wav")
    buf = graph.load_buffer(path)

    samp = sampler(graph, trig, pitch, buf)

    # env = decay_env(graph, trig, graph.constant(0.1))

    mix = samp * amp

    mix.output(0).connect(out1.input(0))
    mix.output(0).connect(out2.input(0))

    runtime = graph.build_runtime()
    handle = runtime.run()

    repl(runtime)
