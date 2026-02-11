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
    camera_aspect_mode = pyg.CameraAspectMode.FIT_BOTH
    engine.set_camera_aspect_mode(camera_aspect_mode)
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
    engine.input.set_action_keys("aspect_stretch", [pyg.Keys.F1])
    engine.input.set_action_keys("aspect_match_horizontal", [pyg.Keys.F2])
    engine.input.set_action_keys("aspect_match_vertical", [pyg.Keys.F3])
    engine.input.set_action_keys("aspect_fit_both", [pyg.Keys.F4])
    engine.input.set_action_keys("aspect_fill_both", [pyg.Keys.F5])

    engine.log_info(
        "Camera demo running. Uses input axes: Horizontal/Vertical + CameraZoom."
    )

    camera_speed = 9.0
    zoom_speed = 1.8
    wheel_zoom_sensitivity = 0.10
    hud_update_interval = 1.0 / 60.0
    last_hud_update_time = 0.0
    is_drag_panning = False
    last_drag_world: pyg.Vec2 | None = None
    camera_line = ""
    viewport_line = ""
    axes_line = ""
    mouse_line = ""
    origin_line = ""
    aspect_line = ""

    while engine.poll_events():
        if engine.input.action_pressed("quit"):
            break

        dt = max(engine.delta_time, 0.0)
        # Get camera position and input axes.
        cam = engine.get_camera_position()
        move_x = engine.input.axis("Horizontal")
        move_y = engine.input.axis("Vertical")
        zoom_axis = engine.input.axis("CameraZoom")
        _, wheel_y = engine.input.mouse_wheel

        # Aspect mode controls
        if engine.input.action_pressed("aspect_stretch"):
            camera_aspect_mode = pyg.CameraAspectMode.STRETCH
            engine.set_camera_aspect_mode(camera_aspect_mode)
        elif engine.input.action_pressed("aspect_match_horizontal"):
            camera_aspect_mode = pyg.CameraAspectMode.MATCH_HORIZONTAL
            engine.set_camera_aspect_mode(camera_aspect_mode)
        elif engine.input.action_pressed("aspect_match_vertical"):
            camera_aspect_mode = pyg.CameraAspectMode.MATCH_VERTICAL
            engine.set_camera_aspect_mode(camera_aspect_mode)
        elif engine.input.action_pressed("aspect_fit_both"):
            camera_aspect_mode = pyg.CameraAspectMode.FIT_BOTH
            engine.set_camera_aspect_mode(camera_aspect_mode)
        elif engine.input.action_pressed("aspect_fill_both"):
            camera_aspect_mode = pyg.CameraAspectMode.FILL_BOTH
            engine.set_camera_aspect_mode(camera_aspect_mode)

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

        # Mouse wheel zoom: scroll up zooms in, scroll down zooms out.
        if wheel_y != 0.0:
            wheel_scale = 1.0 - wheel_y * wheel_zoom_sensitivity
            if wheel_scale > 0.0:
                viewport_width = min(96.0, max(6.0, viewport_width * wheel_scale))
                viewport_height = min(54.0, max(3.5, viewport_height * wheel_scale))
                engine.set_camera_viewport_size(viewport_width, viewport_height)

        t = time.time()
        orbiter.position = pyg.Vec2(math.cos(t * 0.9) * 7.0, math.sin(t * 1.2) * 3.8)

        mouse_x, mouse_y = engine.input.mouse_position
        mouse_world = engine.screen_to_world(float(mouse_x), float(mouse_y))

        # Click-and-drag panning in world-space.
        if engine.input.mouse_button_down(pyg.MouseButton.LEFT):
            if not is_drag_panning:
                is_drag_panning = True
                last_drag_world = mouse_world
            elif last_drag_world is not None:
                drag_dx = last_drag_world.x - mouse_world.x
                drag_dy = last_drag_world.y - mouse_world.y
                if drag_dx != 0.0 or drag_dy != 0.0:
                    cam = pyg.Vec2(cam.x + drag_dx, cam.y + drag_dy)
                    engine.set_camera_position(cam)
                    mouse_world = engine.screen_to_world(float(mouse_x), float(mouse_y))
                last_drag_world = mouse_world
        else:
            is_drag_panning = False
            last_drag_world = None

        # Get mouse-world position on screen.
        origin_screen_x, origin_screen_y = engine.world_to_screen(pyg.Vec2(0.0, 0.0))

        # Update HUD text. Throttling to avoid excessive updates as text drawing is expensive.
        # Note: draw_text() only updates if the text has changed, hence this if statement.
        now = time.perf_counter()
        if now - last_hud_update_time >= hud_update_interval:
            last_hud_update_time = now
            camera_line = (
                f"camera_id={engine.camera_object_id}  camera_pos=({cam.x:.2f}, {cam.y:.2f})"
            )
            viewport_line = f"viewport_world=({viewport_width:.2f}, {viewport_height:.2f})"
            axes_line = (
                f"axes: Horizontal={move_x:.2f} Vertical={move_y:.2f} "
                f"CameraZoom={zoom_axis:.2f} WheelY={wheel_y:.2f} DragPan={is_drag_panning}"
            )
            mouse_line = (
                f"mouse_screen=({mouse_x:.1f}, {mouse_y:.1f}) -> "
                f"mouse_world=({mouse_world.x:.2f}, {mouse_world.y:.2f})"
            )
            origin_line = f"world_origin_on_screen=({origin_screen_x:.1f}, {origin_screen_y:.1f})"
            aspect_line = (
                "aspect_mode="
                f"{camera_aspect_mode} "
                "(F1 stretch, F2 match_horizontal, F3 match_vertical, F4 fit_both, F5 fill_both)"
            )

        # Draw HUD text. 
        engine.clear_draw_commands()
        engine.draw_text(
            "Worldspace Camera Demo (WASD/Arrows pan, Q/E or wheel zoom, LMB drag pan, ESC quit)",
            18.0,
            18.0,
            pyg.Color.WHITE,
            font_size=18.0,
            draw_order=20.0,
        )
        engine.draw_text(
            camera_line,
            18.0,
            44.0,
            pyg.Color.CYAN,
            font_size=16.0,
            draw_order=20.0,
        )
        engine.draw_text(
            viewport_line,
            18.0,
            66.0,
            pyg.Color.rgb(180, 220, 255),
            font_size=16.0,
            draw_order=20.0,
        )
        engine.draw_text(
            axes_line,
            18.0,
            88.0,
            pyg.Color.rgb(180, 210, 180),
            font_size=15.0,
            draw_order=20.0,
        )
        engine.draw_text(
            mouse_line,
            18.0,
            108.0,
            pyg.Color.rgb(220, 220, 220),
            font_size=15.0,
            draw_order=20.0,
        )
        engine.draw_text(
            origin_line,
            18.0,
            128.0,
            pyg.Color.YELLOW,
            font_size=15.0,
            draw_order=20.0,
        )
        engine.draw_text(
            aspect_line,
            18.0,
            148.0,
            pyg.Color.rgb(255, 200, 130),
            font_size=14.0,
            draw_order=20.0,
        )

        engine.update()
        engine.render()

    engine.log_info("Camera demo finished.")


if __name__ == "__main__":
    main()
