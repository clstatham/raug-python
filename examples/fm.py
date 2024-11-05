import raug
import time
import math

if __name__ == "__main__":
    graph = raug.GraphBuilder()

    out1 = graph.add_output()
    out2 = graph.add_output()

    sr = graph.sample_rate()
    pa = graph.phase_accum()
    pa.input(0).connect(sr.recip().output(0))
    pa = pa % 1.0

    freq1 = raug.Param("freq1")
    freq1.set(440.0)

    freq2 = raug.Param("freq2")
    freq2.set(220.0)

    sine1 = (pa * freq1 * 2.0 * math.pi).sin()
    sine2 = (pa * freq2 * 2.0 * math.pi + sine1).sin()

    final = sine2 * 0.2

    final.output(0).connect(out1.input(0))
    final.output(0).connect(out2.input(0))

    runtime = graph.build_runtime()
    handle = runtime.run()

    time.sleep(1.0)

    freq2.set(330.0)

    time.sleep(1.0)

    freq1.set(880.0)

    time.sleep(1.0)

    handle.stop()
