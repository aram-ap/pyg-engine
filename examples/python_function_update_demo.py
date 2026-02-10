#!/usr/bin/env python3
"""
Function-based update loop demo for pyg_engine.

Shows how to pass a Python callback directly to the engine via
`engine.run(update=...)`.
"""

import math
import time

import pyg_engine as pyg


def main() -> None:
    engine = pyg.Engine(log_level="INFO")

    # User-owned state object exposed to callback as `user_data`.
    state = {"radius": 50.0}

    def update(dt: float, engine: pyg.Engine, user_data: dict[str, float]) -> bool:
        if engine.input.key_pressed(pyg.Keys.ESCAPE):
            return False

        # Mirror the manual loop demo's movement profile for fair FPS comparison.
        t = time.time()
        cx = 640.0 + math.cos(t * 2.0) * 200.0
        cy = 360.0 + math.sin(t * 3.0) * 150.0

        engine.clear_draw_commands()
        engine.draw_circle(cx, cy, user_data["radius"], pyg.Color.CYAN, filled=True)
        engine.draw_line(640.0, 360.0, cx, cy, pyg.Color.WHITE)
        return True

    engine.run(
        title="PyG Engine - Function Update Demo",
        width=1280,
        height=720,
        background_color=pyg.Color.rgb(24, 24, 34),
        vsync=False,
        show_fps_in_title=True,
        update=update,
        max_delta_time=0.08,
        user_data=state,
    )


if __name__ == "__main__":
    main()
