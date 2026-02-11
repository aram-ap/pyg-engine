#!/usr/bin/env python3
"""
Python GameObject transform update demo for pyg_engine.

Demonstrates:
- creating one circle MeshComponent on a GameObject
- adding it once to the runtime scene
- updating only its transform each frame via `circle.position = Vec2(...)`
"""

import math
import time

import pyg_engine as pyg


def create_circle_object() -> pyg.GameObject:
    circle = pyg.GameObject("MovingCircle")
    circle.position = pyg.Vec2(0.0, 0.0)
    circle.scale = pyg.Vec2(0.22, 0.22)

    mesh = pyg.MeshComponent("MovingCircleMesh")
    mesh.set_geometry_circle(1.0, segments=48)
    mesh.set_fill_color(pyg.Color.CYAN)
    mesh.draw_order = 2.2
    circle.set_mesh_component(mesh)

    return circle


def main() -> None:
    engine = pyg.Engine(log_level="INFO")

    engine.start_manual(
        title="PyG Engine - GameObject Transform Demo",
        width=1280,
        height=720,
        background_color=pyg.Color.rgb(18, 18, 30),
        vsync=False,
        redraw_on_change_only=False,
        show_fps_in_title=True,
    )

    circle = create_circle_object()
    if engine.add_game_object(circle) is None:
        raise RuntimeError("Failed to add circle GameObject to runtime scene")

    engine.log_info("Running transform update demo. Press ESC to quit.")

    while engine.poll_events():
        if engine.input.key_pressed(pyg.Keys.ESCAPE):
            break

        t = time.time()
        pos_x = math.cos(t * 2.0) * 0.70
        pos_y = math.sin(t * 3.0) * 0.45

        # Property updates now propagate to the runtime object after add_game_object.
        circle.position = pyg.Vec2(pos_x, pos_y)

        engine.clear_draw_commands()
        engine.draw_text(
            "One circle mesh object. Only transform changes per frame.",
            24.0,
            24.0,
            pyg.Color.WHITE,
            font_size=21.0,
            draw_order=11.0,
        )
        engine.draw_text(
            f"position=({pos_x:.2f}, {pos_y:.2f})  |  ESC to quit",
            24.0,
            56.0,
            pyg.Color.CYAN,
            font_size=18.0,
            draw_order=11.0,
        )
        engine.draw_text(
            "Mesh transforms currently use clip-space style coordinates (~[-1, 1]).",
            24.0,
            84.0,
            pyg.Color.rgb(180, 180, 190),
            font_size=16.0,
            draw_order=11.0,
        )

        engine.update()
        engine.render()

    engine.log_info("Transform demo finished.")


if __name__ == "__main__":
    main()
