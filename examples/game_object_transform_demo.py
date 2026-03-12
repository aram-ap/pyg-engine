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


def create_circle_hierarchy() -> tuple[pyg.GameObject, pyg.GameObject]:
    parent = pyg.GameObject("MovingParent")
    parent.position = pyg.Vec2(0.0, 0.0)

    circle = pyg.GameObject("MovingCircle")
    circle.position = pyg.Vec2(0.70, 0.0)
    circle.scale = pyg.Vec2(0.22, 0.22)

    mesh = pyg.MeshComponent("MovingCircleMesh")
    mesh.set_geometry(pyg.Mesh.Circle(1.0, segments=48))
    mesh.set_fill_color(pyg.Color.CYAN)
    mesh.draw_order = 2.2
    circle.add_component(mesh)

    parent.add_child(circle)
    return parent, circle


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

    parent, circle = create_circle_hierarchy()
    parent_id = engine.add_game_object(parent)
    circle_id = engine.add_game_object(circle)
    if parent_id is None or circle_id is None:
        raise RuntimeError("Failed to add hierarchy to runtime scene")

    runtime_parent = engine.objects.get_id(parent_id)
    runtime_circle = engine.objects.get_id(circle_id)
    if runtime_parent is None or runtime_circle is None:
        raise RuntimeError("Failed to resolve hierarchy runtime handles")

    engine.log_info("Running transform update demo. Press ESC to quit.")

    while engine.poll_events():
        if engine.input.key_pressed(pyg.Keys.ESCAPE):
            break

        t = time.time()
        pos_x = math.cos(t * 2.0) * 0.70
        pos_y = math.sin(t * 3.0) * 0.45

        # Parent transform now propagates into the child's world transform.
        runtime_parent.position = pyg.Vec2(pos_x, pos_y)
        runtime_parent.rotation = t * 0.8

        engine.clear_draw_commands()
        engine.draw(
            [
                pyg.Text(
                    "Parent transform drives a child circle through hierarchy propagation.",
                    position=pyg.Vec2(24.0, 24.0),
                    color=pyg.Color.WHITE,
                    font_size=21.0,
                    draw_order=11.0,
                ),
                pyg.Text(
                    f"parent_pos=({pos_x:.2f}, {pos_y:.2f})  child_local=({runtime_circle.position.x:.2f}, {runtime_circle.position.y:.2f})",
                    position=pyg.Vec2(24.0, 56.0),
                    color=pyg.Color.CYAN,
                    font_size=18.0,
                    draw_order=11.0,
                ),
                pyg.Text(
                    "The child keeps a local offset while render/physics use the composed world transform.",
                    position=pyg.Vec2(24.0, 84.0),
                    color=pyg.Color.rgb(180, 180, 190),
                    font_size=16.0,
                    draw_order=11.0,
                ),
            ]
        )

        engine.update()
        engine.render()

    engine.log_info("Transform demo finished.")


if __name__ == "__main__":
    main()
