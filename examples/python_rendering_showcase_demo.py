#!/usr/bin/env python3
"""
Unified rendering showcase for pyg_engine.

This demo combines all rendering paths into one interactive scene:
- Mesh + GameObject rendering (solid + textured quads)
- Immediate direct draw primitives
- Gradient background rendering
- Image rendering from file path
- Dynamic image rendering from RGBA bytes
- Bulk draw command submission via DrawCommand builders
- Layer and z-index ordering

Controls:
- WASD / Arrow keys: Move probe
- Left click: Teleport probe to mouse
- Right click: Randomize visualization phase
- Space: Pause/resume animation
- B: Toggle bulk waveform overlay
- P: Toggle direct primitive overlay
- V: Toggle dynamic bytes texture overlay
- Escape: Exit
"""

from __future__ import annotations

import math
import random
import time

from pyg_engine import (
    Color,
    DrawCommand,
    Engine,
    GameObject,
    Keys,
    MeshComponent,
    MouseButton,
    Vec2,
)


def clamp(value: float, minimum: float, maximum: float) -> float:
    return max(minimum, min(value, maximum))


def color_to_rgba8(color: Color) -> tuple[int, int, int, int]:
    return (
        int(clamp(color.r, 0.0, 1.0) * 255.0),
        int(clamp(color.g, 0.0, 1.0) * 255.0),
        int(clamp(color.b, 0.0, 1.0) * 255.0),
        int(clamp(color.a, 0.0, 1.0) * 255.0),
    )


def build_corner_gradient_rgba(
    top_left: Color,
    bottom_left: Color,
    bottom_right: Color,
    top_right: Color,
) -> bytes:
    """
    Build a 2x2 RGBA texture matching pygame's gradient seed:
    row 0: top-left, top-right
    row 1: bottom-left, bottom-right
    """
    tl = color_to_rgba8(top_left)
    bl = color_to_rgba8(bottom_left)
    br = color_to_rgba8(bottom_right)
    tr = color_to_rgba8(top_right)
    return bytes((*tl, *tr, *bl, *br))


def add_mesh_showcase(engine: Engine) -> None:
    """Create a static mesh scene on the left side."""
    panel = GameObject("MeshPanel")
    panel.position = Vec2(-0.45, 0.02)
    panel.scale = Vec2(0.95, 0.78)
    panel_mesh = MeshComponent("MeshPanelMesh")
    panel_mesh.set_geometry_rectangle(1.25, 1.05)
    panel_mesh.set_fill_color(Color(0.10, 0.11, 0.16, 0.92))
    panel_mesh.layer = 0
    panel_mesh.z_index = -0.6
    panel.set_mesh_component(panel_mesh)
    engine.add_game_object(panel)

    textured = GameObject("MeshTextured")
    textured.position = Vec2(-0.62, -0.02)
    textured.scale = Vec2(0.28, 0.46)
    textured.rotation = -0.07
    textured_mesh = MeshComponent("MeshTexturedMesh")
    textured_mesh.set_geometry_rectangle(1.0, 1.0)
    textured_mesh.set_fill_color(Color.WHITE)
    textured_mesh.set_image_path("images/1_lower-res.png")
    textured_mesh.layer = 1
    textured_mesh.z_index = 0.10
    textured.set_mesh_component(textured_mesh)
    engine.add_game_object(textured)

    solid = GameObject("MeshSolid")
    solid.position = Vec2(-0.26, 0.10)
    solid.scale = Vec2(0.26, 0.30)
    solid.rotation = 0.38
    solid_mesh = MeshComponent("MeshSolidMesh")
    solid_mesh.set_geometry_rectangle(1.0, 1.0)
    solid_mesh.set_fill_color(Color(0.95, 0.50, 0.16, 0.95))
    solid_mesh.layer = 2
    solid_mesh.z_index = 0.25
    solid.set_mesh_component(solid_mesh)
    engine.add_game_object(solid)

    accent = GameObject("MeshAccent")
    accent.position = Vec2(-0.31, -0.28)
    accent.scale = Vec2(0.42, 0.10)
    accent_mesh = MeshComponent("MeshAccentMesh")
    accent_mesh.set_geometry_rectangle(1.0, 1.0)
    accent_mesh.set_fill_color(Color(0.15, 0.85, 0.90, 0.85))
    accent_mesh.layer = 3
    accent_mesh.z_index = 0.30
    accent.set_mesh_component(accent_mesh)
    engine.add_game_object(accent)


def build_dynamic_rgba(width: int, height: int, t: float, phase: float) -> bytes:
    """Generate a colorful procedural texture for draw_image_from_bytes."""
    pixels = bytearray(width * height * 4)
    for y in range(height):
        ny = (y / max(1, height - 1)) - 0.5
        for x in range(width):
            nx = (x / max(1, width - 1)) - 0.5
            idx = (y * width + x) * 4

            radial = math.sqrt(nx * nx + ny * ny)
            wave_a = math.sin((nx * 12.0) + (t * 1.7) + phase)
            wave_b = math.cos((ny * 14.0) - (t * 1.3) - phase * 0.6)
            wave_c = math.sin((radial * 30.0) - (t * 3.2) + phase * 0.4)

            r = 0.5 + 0.5 * wave_a
            g = 0.5 + 0.5 * wave_b
            b = 0.5 + 0.5 * wave_c

            # Soft vignette for nicer composition.
            vignette = clamp(1.15 - radial * 1.6, 0.2, 1.0)
            pixels[idx] = int(255.0 * r * vignette)
            pixels[idx + 1] = int(255.0 * g * vignette)
            pixels[idx + 2] = int(255.0 * b * vignette)
            pixels[idx + 3] = 255
    return bytes(pixels)


def build_bulk_overlay(
    width: int,
    height: int,
    t: float,
    probe_x: float,
    probe_y: float,
    bar_count: int,
) -> list[DrawCommand]:
    """Build many commands for one-shot submission via add_draw_commands."""
    commands: list[DrawCommand] = []

    band_top = height * 0.68
    band_bottom = height - 30.0
    amplitude = (band_bottom - band_top) * 0.9
    center_y = band_bottom - 8.0

    left = 40.0
    right = width - 40.0
    step = (right - left) / max(1, bar_count)
    bar_width = max(2.0, step * 0.62)

    for i in range(bar_count):
        x = left + i * step
        wave = math.sin(i * 0.12 + t * 2.4) * 0.5 + 0.5
        h = 18.0 + wave * amplitude
        c = 0.35 + wave * 0.55
        color = Color(0.12, c, 0.95, 0.88)
        commands.append(DrawCommand.rectangle(x, center_y - h, bar_width, h, color, True, 1.0, 9, 0.20))

    prev_x = left
    prev_y = center_y - (
        (math.sin((0 * 0.12) + t * 1.6) * 0.5 + 0.5) * (amplitude * 0.85) + 12.0
    )
    for i in range(1, bar_count, 2):
        x = left + i * step
        y = center_y - (
            (math.sin((i * 0.12) + t * 1.6) * 0.5 + 0.5) * (amplitude * 0.85) + 12.0
        )
        commands.append(
            DrawCommand.line(prev_x, prev_y, x, y, Color(1.0, 0.86, 0.22, 1.0), 2.0, 10, 0.35)
        )
        prev_x, prev_y = x, y

    # Add a ring around the probe using the bulk path too.
    commands.append(
        DrawCommand.circle(
            probe_x,
            probe_y,
            18.0,
            Color(1.0, 1.0, 1.0, 0.8),
            False,
            2.0,
            36,
            12,
            0.92,
        )
    )

    return commands


def main() -> None:
    engine = Engine(log_level="INFO")
    add_mesh_showcase(engine)

    engine.initialize(
        title="PyG Engine - Unified Rendering Showcase",
        width=1360,
        height=840,
        vsync=False,
        redraw_on_change_only=False,
        show_fps_in_title=True,
    )

    print("Unified Rendering Showcase Controls:")
    print("  WASD / Arrow Keys: Move probe")
    print("  Left click: Teleport probe")
    print("  Right click: Randomize visualization phase")
    print("  Space: Pause/Resume animation")
    print("  B / P / V: Toggle bulk/primitives/bytes-texture overlays")
    print("  Escape: Quit")

    # Performance tuning defaults for the showcase:
    # - keep animation at 60 FPS max
    # - update expensive procedural texture at 24 Hz
    # - use a moderate bar count for bulk overlay
    target_fps = 60.0
    target_frame_dt = 1.0 / target_fps
    dynamic_texture_hz = 24.0
    dynamic_texture_dt = 1.0 / dynamic_texture_hz
    bulk_bar_count = 120

    probe_x = 1020.0
    probe_y = 260.0
    move_speed = 420.0

    show_bulk = True
    show_primitives = True
    show_dynamic_texture = True
    paused = False
    sim_time = 0.0
    phase_shift = random.uniform(0.0, 10.0)

    dyn_tex_w = 128
    dyn_tex_h = 128
    dyn_tex_key = "showcase_dynamic_texture"
    cached_dynamic_rgba = build_dynamic_rgba(dyn_tex_w, dyn_tex_h, sim_time, phase_shift)
    dynamic_tex_accumulator = dynamic_texture_dt
    bg_gradient_key = "showcase_bg_gradient"

    # Local edge detection for reliable one-shot controls.
    prev_escape_down = False
    prev_space_down = False
    prev_b_down = False
    prev_p_down = False
    prev_v_down = False
    prev_left_mouse_down = False
    prev_right_mouse_down = False

    while engine.poll_events():
        frame_start = time.perf_counter()
        dt = min(engine.delta_time, 0.1)
        if not paused:
            sim_time += dt

        display_width, display_height = engine.get_display_size()
        mouse_x, mouse_y = engine.input.mouse_position

        escape_down = engine.input.key_down(Keys.ESCAPE)
        space_down = engine.input.key_down(Keys.SPACE)
        b_down = engine.input.key_down("b")
        p_down = engine.input.key_down("p")
        v_down = engine.input.key_down("v")
        left_mouse_down = engine.input.mouse_button_down(MouseButton.LEFT)
        right_mouse_down = engine.input.mouse_button_down(MouseButton.RIGHT)

        if escape_down and not prev_escape_down:
            break
        if space_down and not prev_space_down:
            paused = not paused
        if b_down and not prev_b_down:
            show_bulk = not show_bulk
        if p_down and not prev_p_down:
            show_primitives = not show_primitives
        if v_down and not prev_v_down:
            show_dynamic_texture = not show_dynamic_texture

        move_x = 0.0
        move_y = 0.0
        if engine.input.key_down("a") or engine.input.key_down(Keys.ARROW_LEFT):
            move_x -= 1.0
        if engine.input.key_down("d") or engine.input.key_down(Keys.ARROW_RIGHT):
            move_x += 1.0
        if engine.input.key_down("w") or engine.input.key_down(Keys.ARROW_UP):
            move_y -= 1.0
        if engine.input.key_down("s") or engine.input.key_down(Keys.ARROW_DOWN):
            move_y += 1.0

        probe_x = clamp(probe_x + move_x * move_speed * dt, 0.0, float(display_width))
        probe_y = clamp(probe_y + move_y * move_speed * dt, 0.0, float(display_height))

        if left_mouse_down and not prev_left_mouse_down:
            probe_x = float(mouse_x)
            probe_y = float(mouse_y)
        if right_mouse_down and not prev_right_mouse_down:
            phase_shift = random.uniform(0.0, 12.0)
            # Force a refresh immediately after randomizing phase.
            dynamic_tex_accumulator = dynamic_texture_dt

        pulse = 0.5 + 0.5 * math.sin(sim_time * 1.1 + phase_shift * 0.2)
        # Slightly brighter and more saturated than the previous palette so
        # the engine gradient matches pygame's perceived output better.
        top_left = Color(0.08, 0.10, 0.20 + 0.16 * pulse, 1.0)
        bottom_left = Color(0.10, 0.19 + 0.24 * pulse, 0.36, 1.0)
        bottom_right = Color(0.24 + 0.24 * pulse, 0.10, 0.34, 1.0)
        top_right = Color(0.12, 0.11 + 0.15 * pulse, 0.27 + 0.18 * pulse, 1.0)
        gradient_rgba = build_corner_gradient_rgba(
            top_left,
            bottom_left,
            bottom_right,
            top_right,
        )

        image_size = min(display_width, display_height) * 0.20
        image_x = display_width * 0.72
        image_y = display_height * 0.06

        dyn_size = min(display_width, display_height) * 0.34
        dyn_x = display_width * 0.56
        dyn_y = display_height * 0.24

        engine.clear_draw_commands()

        # Full-screen gradient using the same 2x2 seed approach as pygame clone.
        engine.draw_image_from_bytes(
            0.0,
            0.0,
            float(display_width),
            float(display_height),
            bg_gradient_key,
            gradient_rgba,
            2,
            2,
            layer=-10,
            z_index=0.0,
        )

        # File-path image draw path.
        engine.draw_image(
            image_x,
            image_y,
            image_size,
            image_size,
            "images/1_lower-res.png",
            layer=4,
            z_index=0.25,
        )

        # Dynamic RGBA-bytes image draw path.
        if show_dynamic_texture:
            dynamic_tex_accumulator += dt
            if dynamic_tex_accumulator >= dynamic_texture_dt:
                cached_dynamic_rgba = build_dynamic_rgba(dyn_tex_w, dyn_tex_h, sim_time, phase_shift)
                dynamic_tex_accumulator = 0.0
            engine.draw_image_from_bytes(
                dyn_x,
                dyn_y,
                dyn_size,
                dyn_size,
                dyn_tex_key,
                cached_dynamic_rgba,
                dyn_tex_w,
                dyn_tex_h,
                layer=5,
                z_index=0.32,
            )

        # Immediate primitive layer.
        if show_primitives:
            engine.draw_rectangle(
                24.0,
                24.0,
                display_width * 0.50 - 40.0,
                display_height * 0.60,
                Color(0.90, 0.93, 1.0, 0.85),
                filled=False,
                thickness=2.0,
                layer=14,
                z_index=0.95,
            )
            engine.draw_rectangle(
                dyn_x - 10.0,
                dyn_y - 10.0,
                dyn_size + 20.0,
                dyn_size + 20.0,
                Color(0.90, 0.93, 1.0, 0.85),
                filled=False,
                thickness=2.0,
                layer=14,
                z_index=0.95,
            )
            engine.draw_line(
                probe_x,
                probe_y,
                float(mouse_x),
                float(mouse_y),
                Color(1.0, 1.0, 1.0, 0.45),
                thickness=1.0,
                layer=13,
                z_index=0.90,
            )
            engine.draw_circle(
                probe_x,
                probe_y,
                10.0,
                Color(0.98, 0.42, 0.24, 1.0),
                filled=True,
                layer=13,
                z_index=0.91,
            )
            engine.draw_circle(
                probe_x,
                probe_y,
                26.0,
                Color(1.0, 1.0, 1.0, 0.35),
                filled=False,
                thickness=2.0,
                segments=40,
                layer=13,
                z_index=0.91,
            )

            for i in range(32):
                px = int(36 + (i % 8) * 3 + math.sin(sim_time * 5.0 + i) * 1.2)
                py = int(36 + (i // 8) * 3)
                engine.draw_pixel(px, py, Color.WHITE, layer=15, z_index=1.0)

        if show_bulk:
            commands = build_bulk_overlay(
                display_width,
                display_height,
                sim_time,
                probe_x,
                probe_y,
                bar_count=bulk_bar_count,
            )
            engine.add_draw_commands(commands)

        hud_text = (
            f"probe=({int(probe_x)}, {int(probe_y)})  mouse=({mouse_x}, {mouse_y})\n"
            f"bulk={'on' if show_bulk else 'off'}  primitives={'on' if show_primitives else 'off'}  "
            f"dynamic={'on' if show_dynamic_texture else 'off'}"
        )
        engine.draw_text(
            hud_text,
            30.0,
            display_height * 0.70,
            Color(0.98, 0.99, 1.0, 0.90),
            font_size=16.0,
            letter_spacing=1.0,
            line_spacing=4.0,
            layer=18,
            z_index=1.15,
        )

        engine.draw_text(
            "PyG Text Rendering",
            display_width * 0.66,
            display_height * 0.93,
            Color(0.90, 0.96, 1.0, 0.95),
            font_size=20.0,
            letter_spacing=1.5,
            layer=18,
            z_index=1.16,
        )

        engine.update()
        engine.render()

        # Avoid spin-waiting the CPU in manual-loop mode.
        frame_elapsed = time.perf_counter() - frame_start
        remaining = target_frame_dt - frame_elapsed
        if remaining > 0.0:
            time.sleep(remaining)

        prev_escape_down = escape_down
        prev_space_down = space_down
        prev_b_down = b_down
        prev_p_down = p_down
        prev_v_down = v_down
        prev_left_mouse_down = left_mouse_down
        prev_right_mouse_down = right_mouse_down


if __name__ == "__main__":
    main()
