import raug
import math


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
