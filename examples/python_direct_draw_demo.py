#!/usr/bin/env python3
"""
Python direct-draw demo for pyg_engine.

Demonstrates immediate-mode drawing from Python:
- pixels
- lines
- rectangles (filled + outline)
- circles (filled + outline)
"""

import pyg_engine as pyg


def populate_direct_draw_scene(engine: pyg.Engine) -> None:
    """Queue immediate draw commands before running the window loop."""
    # Pixel block
    for x in range(16, 46):
        for y in range(16, 46):
            engine.draw_pixel(x, y, pyg.Color.WHITE, draw_order=2.8)

    # Lines
    engine.draw_line(80, 100, 620, 160, pyg.Color.CYAN, thickness=2.0, draw_order=1.1)
    engine.draw_line(80, 170, 620, 320, pyg.Color.LIME, thickness=3.0, draw_order=1.2)

    # Rectangles
    engine.draw_rectangle(100, 400, 220, 120, pyg.Color.ORANGE, filled=True, draw_order=1.3)
    engine.draw_rectangle(
        360,
        400,
        220,
        120,
        pyg.Color.WHITE,
        filled=False,
        thickness=3.0,
        draw_order=2.5,
    )

    # Circles
    engine.draw_circle(860, 220, 90, pyg.Color.MAGENTA, filled=True, draw_order=1.25)
    engine.draw_circle(
        1060,
        220,
        90,
        pyg.Color.YELLOW,
        filled=False,
        thickness=4.0,
        segments=48,
        draw_order=2.45,
    )


def main() -> None:
    engine = pyg.Engine(log_level="INFO")
    populate_direct_draw_scene(engine)

    # Direct draw coordinates are in pixel space.
    engine.run(
        title="PyG Engine - Python Direct Draw Demo",
        width=1280,
        height=720,
        background_color=pyg.Color.rgb(18, 18, 28),
        redraw_on_change_only=True,
    )


if __name__ == "__main__":
    main()
