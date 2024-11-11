import raug

if __name__ == "__main__":
    graph = raug.GraphBuilder()

    out1 = graph.add_audio_output()
    out2 = graph.add_audio_output()

    sine = graph.sine_osc()
    sine.input("frequency").set(440.0)

    sine = sine * 0.2

    sine.output(0).connect(out1.input(0))
    sine.output(0).connect(out2.input(0))

    runtime = graph.build_runtime()
    runtime.run_for(1.0)
