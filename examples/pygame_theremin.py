import pygame
import raug
import math

SCREEN_WIDTH = 800
SCREEN_HEIGHT = 600

if __name__ == "__main__":
    pygame.display.init()

    screen = pygame.display.set_mode((SCREEN_WIDTH, SCREEN_HEIGHT))
    pygame.display.set_caption("Raug Pygame Theremin")

    graph = raug.GraphBuilder()

    out1 = graph.add_output()
    out2 = graph.add_output()

    sr = graph.sample_rate()
    phase = graph.phase_accum()

    freq_param = raug.Param("freq")
    freq_param.set(440.0)

    amp_param = raug.Param("amp")
    amp_param.set(0.2)

    freq = graph.param(freq_param).smooth()
    amp = graph.param(amp_param).smooth()

    increment = freq / sr
    phase.input("increment").connect(increment.output(0))
    sine = (phase * 2.0 * math.pi).sin() * amp

    sine.output(0).connect(out1.input(0))
    sine.output(0).connect(out2.input(0))

    runtime = graph.build_runtime()
    handle = runtime.run()

    running = True
    while running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
            if event.type == pygame.MOUSEMOTION:
                freq_param.set(440.0 + (event.pos[0] / SCREEN_WIDTH) * 880.0)
                amp_param.set(0.2 - (event.pos[1] / SCREEN_HEIGHT) * 0.2)
            if event.type == pygame.WINDOWLEAVE:
                amp_param.set(0.0)

        screen.fill((0, 0, 0))
        pygame.display.flip()

    handle.stop()
    pygame.quit()
