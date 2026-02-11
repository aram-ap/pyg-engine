#!/usr/bin/env python3
"""
Python world-space camera demo for pyg_engine.

Demonstrates:
- world-space GameObject positions relative to world origin (0, 0)
- active camera movement (WASD / Arrow keys)
- camera viewport sizing in world units
- screen_to_world / world_to_screen conversion helpers
- screen-space UI draw commands coexisting with world-space objects
"""

import math
import time

import pyg_engine as pyg


def create_marker(name: str, position: pyg.Vec2, color: pyg.Color, size: float) -> pyg.GameObject:
    marker = pyg.GameObject(name)
    marker.position = position
    marker.scale = pyg.Vec2(size, size)

    mesh = pyg.MeshComponent(f"{name}Mesh")
    mesh.set_geometry_circle(1.0, segments=40)
    mesh.set_fill_color(color)
    marker.set_mesh_component(mesh)
    return marker


def main() -> None:
    engine = pyg.Engine(log_level="INFO")
    engine.start_manual(
        title="PyG Engine - Worldspace Camera Demo",
        width=1280,
        height=720,
        background_color=pyg.Color.rgb(16, 22, 30),
        redraw_on_change_only=False,
        show_fps_in_title=True,
        vsync=False,
    )

    # Camera viewport is measured in world units visible across the screen.
    viewport_width = 24.0
    viewport_height = 13.5
    engine.set_camera_viewport_size(viewport_width, viewport_height)
    engine.set_camera_position(pyg.Vec2(0.0, 0.0))
    engine.set_camera_background_color(pyg.Color.rgb(16, 22, 30))

    # Place world markers around the origin.
    markers = [
        create_marker("Origin", pyg.Vec2(0.0, 0.0), pyg.Color.YELLOW, 0.45),
        create_marker("East", pyg.Vec2(6.0, 0.0), pyg.Color.RED, 0.35),
        create_marker("West", pyg.Vec2(-6.0, 0.0), pyg.Color.CYAN, 0.35),
        create_marker("North", pyg.Vec2(0.0, 4.0), pyg.Color.GREEN, 0.35),
        create_marker("South", pyg.Vec2(0.0, -4.0), pyg.Color.MAGENTA, 0.35),
    ]
    for marker in markers:
        if engine.add_game_object(marker) is None:
            raise RuntimeError(f"Failed to add marker '{marker.name}'")

    orbiter = create_marker("Orbiter", pyg.Vec2(4.0, 0.0), pyg.Color.WHITE, 0.28)
    if engine.add_game_object(orbiter) is None:
        raise RuntimeError("Failed to add orbiter marker")

    # Demonstrate custom, rebindable axis setup.
    engine.input.set_axis_keys(
        "CameraZoom",
        positive_keys=[pyg.Keys.E],  # zoom out
        negative_keys=[pyg.Keys.Q],  # zoom in
        sensitivity=1.0,
    )
    engine.input.set_action_keys("quit", [pyg.Keys.ESCAPE])

    engine.log_info(
        "Camera demo running. Uses input axes: Horizontal/Vertical + CameraZoom."
    )

    camera_speed = 9.0
    zoom_speed = 1.8
    while engine.poll_events():
        if engine.input.action_pressed("quit"):
            break

        dt = max(engine.delta_time, 0.0)
        cam = engine.get_camera_position()
        move_x = engine.input.axis("Horizontal")
        move_y = engine.input.axis("Vertical")
        zoom_axis = engine.input.axis("CameraZoom")

        if move_x != 0.0 or move_y != 0.0:
            length = math.sqrt(move_x * move_x + move_y * move_y)
            move_x /= length
            move_y /= length
            cam = pyg.Vec2(
                cam.x + move_x * camera_speed * dt,
                cam.y + move_y * camera_speed * dt,
            )
            engine.set_camera_position(cam)

        # Continuous viewport zoom driven by custom input axis.
        if zoom_axis != 0.0:
            zoom_scale = 1.0 + zoom_axis * zoom_speed * dt
            if zoom_scale > 0.0:
                viewport_width = min(96.0, max(6.0, viewport_width * zoom_scale))
                viewport_height = min(54.0, max(3.5, viewport_height * zoom_scale))
                engine.set_camera_viewport_size(viewport_width, viewport_height)

        t = time.time()
        orbiter.position = pyg.Vec2(math.cos(t * 0.9) * 7.0, math.sin(t * 1.2) * 3.8)

        mouse_x, mouse_y = engine.input.mouse_position
        mouse_world = engine.screen_to_world(float(mouse_x), float(mouse_y))
        origin_screen_x, origin_screen_y = engine.world_to_screen(pyg.Vec2(0.0, 0.0))

        engine.clear_draw_commands()
        engine.draw_text(
            "Worldspace Camera Demo (axes: Horizontal/Vertical + CameraZoom, action: quit)",
            18.0,
            18.0,
            pyg.Color.WHITE,
            font_size=18.0,
            draw_order=20.0,
        )
        engine.draw_text(
            f"camera_id={engine.camera_object_id}  camera_pos=({cam.x:.2f}, {cam.y:.2f})",
            18.0,
            44.0,
            pyg.Color.CYAN,
            font_size=16.0,
            draw_order=20.0,
        )
        engine.draw_text(
            f"viewport_world=({viewport_width:.2f}, {viewport_height:.2f})",
            18.0,
            66.0,
            pyg.Color.rgb(180, 220, 255),
            font_size=16.0,
            draw_order=20.0,
        )
        engine.draw_text(
            f"axes: Horizontal={move_x:.2f} Vertical={move_y:.2f} CameraZoom={zoom_axis:.2f}",
            18.0,
            88.0,
            pyg.Color.rgb(180, 210, 180),
            font_size=15.0,
            draw_order=20.0,
        )
        engine.draw_text(
            f"mouse_screen=({mouse_x:.1f}, {mouse_y:.1f}) -> mouse_world=({mouse_world.x:.2f}, {mouse_world.y:.2f})",
            18.0,
            108.0,
            pyg.Color.rgb(220, 220, 220),
            font_size=15.0,
            draw_order=20.0,
        )
        engine.draw_text(
            f"world_origin_on_screen=({origin_screen_x:.1f}, {origin_screen_y:.1f})",
            18.0,
            128.0,
            pyg.Color.YELLOW,
            font_size=15.0,
            draw_order=20.0,
        )

        engine.update()
        engine.render()

    engine.log_info("Camera demo finished.")


if __name__ == "__main__":
    main()
