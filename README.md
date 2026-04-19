# State Space Visualiser

A double pendulum simulation for all pendula in angle space for a specified resolution.

<p align='center'>
  <img src="/output/render_512_10.gif" alt="Compressed gif of render" width="256">
</p>

Simulates progression of pendula and renders out frames based on a defined colourmap, which colours each pixel (pendulum) $p_{ij}$ uniquely from its position $(\theta_1, \theta_2)$.

![Frame 64 of a high fidelity simulation](/output/frame_0064.png)

See `output/` and `renders/` for more output. These are 1024x1024 resolution generated at 120 updates per second and 30fps (rendering every 4th frame), and 4 minutes worth of simulation. This took around 6 hours to complete (single threaded). It's worth noting that the `interesting' parts of the simulation are really only in the first 30 seconds, so a high quality and fun-to-look-at simulation can take as little as a few minutes.

The colourmap of angle space (pendulum position) to colour is the same as the first rendered frame, `frame_0001.png`, as this is just the mapping before any movement. The map is toroidal (bidirectionally cyclic) and unique (injective) from $[2\pi, 2\pi) \to C$, where $C$ is the set of all unsigned 8-bit integer 3-tuples (so, RGB).
This mapping, while mathematically satisfying, could be more visually appealing. It's pretty muddy and grey for late timesteps at the moment; more contrast and generally richer colours could be fun.

Parallelisation is a strong area for improvement; even dividing the angle space into a 2x2 grid across 4 threads would massively bump performance. This is also a natural fit for a compute shader.

>## Warning for local use:
>Generates large files. The prompted `ffmpeg` concatenation after rendering reduces size significantly (~20x) but a high-quality render (like in `output`) will generate ~20G of pngs, with a ~1G mp4 after concatenation.
