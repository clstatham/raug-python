import raug
from typing import List
from example_utils import random_osc, repl, fm_sine_osc, random
from envelope import decay_env


def pick_randomly(graph: raug.GraphBuilder, trig: raug.Node, nodes: List[raug.Node]) -> raug.Node:
    index = (random(graph, trig) * len(nodes)).floor()
    select = graph.select(len(nodes))
    select.input("in").connect(graph.constant_message(raug.Bang()).output(0))
    select.input("index").connect(index.output(0))

    merge = graph.merge(len(nodes))

    msgs = [graph.message(raug.Bang()) for _ in nodes]
    for i, (node, msg) in enumerate(zip(nodes, msgs)):
        msg.input(0).connect(select.output(i))
        msg.input(1).connect(node.output(0))
        merge.input(i).connect(msg.output(0))

    return merge


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
    amp = graph.param(amp_param).smooth()

    rate = graph.constant(rates_float[0])

    clock1 = graph.metro()
    clock1.input(0).connect(rate.output(0))

    clock2_rate = pick_randomly(
        graph, clock1, [graph.constant(r) for r in rates_float])

    clock = graph.metro()
    clock.input(0).connect(clock2_rate.output(0))

    modfreqs = [raug.Param(f"modfreq{i}") for i in range(len(modfreqs_float))]
    for i, freq in enumerate(modfreqs):
        freq.set(modfreqs_float[i])

    modulators = [
        fm_sine_osc(graph, graph.param(modfreq), graph.constant(0.0)) for modfreq in modfreqs
    ]

    decays = [raug.Param(f"decay{i}") for i in range(len(decays_float))]
    for i, decay in enumerate(decays):
        decay.set(decays_float[i])

    envelopes = [decay_env(graph, clock, graph.param(decay))
                 for decay in decays]

    freqs = [raug.Param(f"freq{i}") for i in range(len(freqs_float))]
    for i, freq in enumerate(freqs):
        freq.set(freqs_float[i])

    oscs = [
        fm_sine_osc(graph, graph.param(freq),
                    pick_randomly(graph, clock, modulators))
        for freq in freqs
    ]

    sine = pick_randomly(graph, clock, oscs) * \
        pick_randomly(graph, clock, envelopes)

    mix = sine * amp

    mix.output(0).connect(out1.input(0))
    mix.output(0).connect(out2.input(0))

    runtime = graph.build_runtime()
    handle = runtime.run()

    repl(runtime)

    handle.stop()
