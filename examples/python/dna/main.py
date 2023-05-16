#!/usr/bin/env python3

"""
The example from our Getting Started page.

`examples/python/dna/main.py`
"""

from math import tau

import numpy as np
import depthai_viewer as viewer
from rerun_demo.data import build_color_spiral
from rerun_demo.util import bounce_lerp, interleave
from scipy.spatial.transform import Rotation

_, unknown = __import__("argparse").ArgumentParser().parse_known_args()
[__import__("logging").warning(f"unknown arg: {arg}") for arg in unknown]

viewer.init("DNA Abacus")

viewer.spawn()
viewer.set_time_seconds("stable_time", 0)

NUM_POINTS = 100

# points and colors are both np.array((NUM_POINTS, 3))
points1, colors1 = build_color_spiral(NUM_POINTS)
points2, colors2 = build_color_spiral(NUM_POINTS, angular_offset=tau * 0.5)
viewer.log_points("dna/structure/left", points1, colors=colors1, radii=0.08)
viewer.log_points("dna/structure/right", points2, colors=colors2, radii=0.08)

points = interleave(points1, points2)
viewer.log_line_segments("dna/structure/scaffolding", points, color=[128, 128, 128])

time_offsets = np.random.rand(NUM_POINTS)
for i in range(400):
    time = i * 0.01
    viewer.set_time_seconds("stable_time", time)

    times = np.repeat(time, NUM_POINTS) + time_offsets
    beads = [bounce_lerp(points1[n], points2[n], times[n]) for n in range(NUM_POINTS)]
    colors = [[int(bounce_lerp(80, 230, times[n] * 2))] for n in range(NUM_POINTS)]
    viewer.log_points("dna/structure/scaffolding/beads", beads, radii=0.06, colors=np.repeat(colors, 3, axis=-1))

    viewer.log_rigid3(
        "dna/structure",
        parent_from_child=(
            [0, 0, 0],
            Rotation.from_euler("z", time / 4.0 * tau).as_quat(),
        ),
    )
