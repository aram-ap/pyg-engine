#!/usr/bin/env python3
"""
Python shape-first draw demo for pyg_engine.

Demonstrates immediate-mode drawing from Python with `engine.draw(...)`:
- lines
- rectangles
- circles
- arcs
- polygons
- immediate meshes
"""

import math

import pyg_engine as pyg


def populate_direct_draw_scene(engine: pyg.Engine) -> None:
    """Queue immediate draw shapes before running the window loop."""
    engine.draw(
        [
            pyg.Line(
                start=pyg.Vec2(80, 100),
                end=pyg.Vec2(620, 160),
                color=pyg.Color.CYAN,
                thickness=2.0,
                draw_order=1.1,
            ),
            pyg.Line(
                start=pyg.Vec2(80, 170),
                end=pyg.Vec2(620, 320),
                color=pyg.Color.LIME,
                thickness=3.0,
                draw_order=1.2,
            ),
            pyg.Rect(
                position=pyg.Vec2(100, 400),
                width=220,
                height=120,
                color=pyg.Color.ORANGE,
                draw_order=1.3,
            ),
            pyg.Rect(
                position=pyg.Vec2(360, 400),
                width=220,
                height=120,
                color=pyg.Color.WHITE,
                filled=False,
                thickness=3.0,
                draw_order=2.5,
            ),
            pyg.Circle(
                position=pyg.Vec2(860, 220),
                radius=90,
                color=pyg.Color.MAGENTA,
                draw_order=1.25,
            ),
            pyg.Circle(
                position=pyg.Vec2(1060, 220),
                radius=90,
                color=pyg.Color.YELLOW,
                filled=False,
                thickness=4.0,
                segments=48,
                draw_order=2.45,
            ),
            pyg.Arc(
                position=pyg.Vec2(900, 520),
                radius=72,
                start_angle=math.radians(15),
                end_angle=math.radians(300),
                color=pyg.Color.rgb(80, 220, 255),
                filled=False,
                thickness=8.0,
                segments=40,
                draw_order=2.6,
            ),
            pyg.Polygon(
                points=[
                    pyg.Vec2(720, 420),
                    pyg.Vec2(820, 360),
                    pyg.Vec2(900, 450),
                    pyg.Vec2(860, 560),
                    pyg.Vec2(740, 540),
                ],
                color=pyg.Color.rgb(120, 90, 255),
                draw_order=1.9,
            ),
            pyg.Mesh(
                vertices=[
                    pyg.Vec2(1020, 470),
                    pyg.Vec2(1180, 420),
                    pyg.Vec2(1160, 620),
                    pyg.Vec2(980, 600),
                ],
                indices=[0, 1, 2, 0, 2, 3],
                color=pyg.Color.rgb(255, 140, 80),
                draw_order=1.8,
            ),
        ]
    )

    engine.draw(
        pyg.Text(
            "engine.draw(shape)",
            position=pyg.Vec2(24, 52),
            color=pyg.Color.WHITE,
            font_size=28.0,
        )
    )


def main() -> None:
    engine = pyg.Engine(log_level="INFO")
    populate_direct_draw_scene(engine)

    # Immediate draw shape coordinates are in screen-space pixels.
    engine.run(
        title="PyG Engine - Python Direct Draw Demo",
        width=1280,
        height=720,
        background_color=pyg.Color.rgb(18, 18, 28),
        redraw_on_change_only=True,
    )


if __name__ == "__main__":
    main()
