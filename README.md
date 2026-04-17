# State Space Visualiser

A double pendulum simulation for all pendula in angle space for a specified resolution.
Simulates progression of pendula and renders out frames based on a defined colourmap, which colours each pixel (pendulum) $p_{ij}$ uniquely from its position $(\theta_1, \theta_2)$.

![Frame 64 of a high fidelity simulation](/output/frame_0064.png)

See `output/` for more output. These were generated at 1024x1024 resolution at 120 updates per second, with every 4th frame rendered (30fps), for 4 minutes of simulation. This took around 6 hours (single threaded).

The colourmap of angle space (pendulum position) to colour is the same as the first rendered frame, `frame_0001.png`, as this is just the mapping before any movement. The map is toroidal (bidirectionally cyclic) and unique from $[2\pi, 2\pi) \to C$, where $C$ is the set of all unsigned 8-bit integer 3-tuples (so, RGB).

Parallelisation is a strong area for improvement; even dividing the angle space into a 2x2 grid across 4 threads would massively bump performance.

>## Warning for local use:
>Generates large files. The prompted `ffmpeg` concatenation reduces size significantly (~20x) but a high-quality render (like in `output`) will generate ~20G of pngs, with a ~1G mp4 after concatenation.
