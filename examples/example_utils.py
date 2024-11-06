import raug
import math
from typing import List


def random(graph: raug.GraphBuilder, trig: raug.Node) -> raug.Node:
    noise = graph.noise_osc()
    snh = graph.sample_and_hold()
    trig.output(0).connect(snh.input("trig"))
    noise.output(0).connect(snh.input("in"))
    return snh


def random_osc(graph: raug.GraphBuilder, freq: raug.Node) -> raug.Node:
    trig_rate = freq.recip()
    trig = graph.metro()
    trig.input(0).connect(trig_rate.output(0))
    return random(graph, trig)


def fm_sine_osc(graph: raug.GraphBuilder, freq: raug.Node, mod: raug.Node) -> raug.Node:
    sr = graph.sample_rate()
    phase = graph.phase_accum()
    increment = freq / sr
    phase.input("increment").connect(increment.output(0))
    sine = (phase * 2.0 * math.pi + mod).sin()
    return sine


def sampler(graph: raug.GraphBuilder, trig: raug.Node, pitch: raug.Node, buffer: raug.Node) -> raug.Node:
    sr = graph.sample_rate()
    pa = graph.saw_osc()
    pa.input(2).connect(trig.output(0))
    length = buffer.output("length").make_node()
    freq = pitch.midi2freq() / 440.0 * sr
    pa_freq = freq / length
    pa.input(0).connect(pa_freq.output(0))
    index = pa * length
    buffer.input("position").connect(index.output(0))
    return buffer.output("out").make_node()


def pick_randomly(graph: raug.GraphBuilder, trig: raug.Node, nodes: List[raug.Node]) -> raug.Node:
    index = (random(graph, trig) * (len(nodes) + 1)) % len(nodes)
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


def repl(runtime: raug.Runtime):
    print('"q" to quit')
    print('"l" to list params')
    while True:
        inp = input("> ").strip()
        if inp == "q":
            break
        if inp == "l":
            for name in runtime.param_names():
                val = runtime.param_named(name).get()
                print(f"{name:<12}{val:>12}")
            continue
        try:
            p, v = inp.split()
        except:
            print("invalid input")
            continue
        p = p.strip()
        v = v.strip()
        if v.startswith("b"):
            v = raug.Bang()
        else:
            v = float(v)

        try:
            param = runtime.param_named(p)
        except:
            print("param not found")
            continue

        try:
            param.set(v)
        except:
            print("invalid value")
            continue
