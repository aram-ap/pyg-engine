#!/usr/bin/env python3
"""
Pygame clone of the unified rendering showcase.

This reproduces the same visual ideas as
`examples/python_rendering_showcase_demo.py`, but uses pygame directly for
side-by-side comparison.

Controls:
- WASD / Arrow keys: Move probe
- Left click: Teleport probe to mouse
- Right click: Randomize visualization phase
- Space: Pause/resume animation
- B: Toggle bulk waveform overlay
- P: Toggle primitive overlay
- V: Toggle dynamic bytes texture overlay
- Escape: Exit
"""

from __future__ import annotations

import argparse
import math
import os
import random
from pathlib import Path

try:
    import pygame
except ImportError as exc:
    raise ImportError(
        "pygame is required for this demo. Install with: pip install pygame"
    ) from exc


def clamp(value: float, minimum: float, maximum: float) -> float:
    return max(minimum, min(value, maximum))


def colorf(r: float, g: float, b: float) -> tuple[int, int, int]:
    return (
        int(clamp(r, 0.0, 1.0) * 255.0),
        int(clamp(g, 0.0, 1.0) * 255.0),
        int(clamp(b, 0.0, 1.0) * 255.0),
    )


def lerp(a: float, b: float, t: float) -> float:
    return a + (b - a) * t


def lerp_color(
    c0: tuple[int, int, int],
    c1: tuple[int, int, int],
    t: float,
) -> tuple[int, int, int]:
    return (
        int(lerp(c0[0], c1[0], t)),
        int(lerp(c0[1], c1[1], t)),
        int(lerp(c0[2], c1[2], t)),
    )


def build_dynamic_rgba(width: int, height: int, t: float, phase: float) -> bytes:
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
            vignette = clamp(1.15 - radial * 1.6, 0.2, 1.0)

            pixels[idx] = int(255.0 * r * vignette)
            pixels[idx + 1] = int(255.0 * g * vignette)
            pixels[idx + 2] = int(255.0 * b * vignette)
            pixels[idx + 3] = 255
    return bytes(pixels)


def build_bulk_overlay_commands(
    width: int,
    height: int,
    t: float,
    probe_x: float,
    probe_y: float,
    bar_count: int,
) -> list[tuple]:
    commands: list[tuple] = []

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
        commands.append(
            ("rect", x, center_y - h, bar_width, h, colorf(0.12, c, 0.95))
        )

    prev_x = left
    prev_y = center_y - (
        (math.sin((0 * 0.12) + t * 1.6) * 0.5 + 0.5) * (amplitude * 0.85) + 12.0
    )
    for i in range(1, bar_count, 2):
        x = left + i * step
        y = center_y - (
            (math.sin((i * 0.12) + t * 1.6) * 0.5 + 0.5) * (amplitude * 0.85) + 12.0
        )
        commands.append(("line", prev_x, prev_y, x, y, (255, 220, 56), 2))
        prev_x, prev_y = x, y

    commands.append(("circle", probe_x, probe_y, 18.0, (220, 220, 220), 2))
    return commands


def draw_bulk_overlay(surface: pygame.Surface, commands: list[tuple]) -> None:
    for command in commands:
        if command[0] == "rect":
            _, x, y, w, h, color = command
            pygame.draw.rect(
                surface,
                color,
                pygame.Rect(int(x), int(y), int(max(1.0, w)), int(max(1.0, h))),
            )
        elif command[0] == "line":
            _, x0, y0, x1, y1, color, thickness = command
            pygame.draw.line(
                surface,
                color,
                (int(x0), int(y0)),
                (int(x1), int(y1)),
                int(thickness),
            )
        elif command[0] == "circle":
            _, x, y, radius, color, thickness = command
            pygame.draw.circle(
                surface,
                color,
                (int(x), int(y)),
                int(radius),
                int(thickness),
            )


def rotated_rect_points(
    center: tuple[float, float],
    size: tuple[float, float],
    angle_deg: float,
) -> list[tuple[int, int]]:
    cx, cy = center
    hw = size[0] * 0.5
    hh = size[1] * 0.5
    angle = math.radians(angle_deg)
    cos_a = math.cos(angle)
    sin_a = math.sin(angle)

    corners = [(-hw, -hh), (hw, -hh), (hw, hh), (-hw, hh)]
    points: list[tuple[int, int]] = []
    for x, y in corners:
        rx = cx + x * cos_a - y * sin_a
        ry = cy + x * sin_a + y * cos_a
        points.append((int(rx), int(ry)))
    return points


def build_placeholder_texture(
    width: int,
    height: int,
    base_a: tuple[int, int, int],
    base_b: tuple[int, int, int],
) -> pygame.Surface:
    surface = pygame.Surface((width, height), pygame.SRCALPHA)
    cell = max(6, min(width, height) // 12)
    for y in range(height):
        for x in range(width):
            idx = ((x // cell) + (y // cell)) % 2
            color = base_a if idx == 0 else base_b
            surface.set_at((x, y), (*color, 255))
    return surface.convert_alpha()


def load_texture_with_fallback(
    path: Path,
    fallback_size: tuple[int, int],
    fallback_a: tuple[int, int, int],
    fallback_b: tuple[int, int, int],
) -> pygame.Surface:
    try:
        return pygame.image.load(str(path)).convert_alpha()
    except Exception as image_exc:
        # Some pygame wheels are built without PNG support. If Pillow is
        # available, decode via Pillow and upload raw RGBA bytes to pygame.
        try:
            from PIL import Image  # type: ignore

            pil_image = Image.open(path).convert("RGBA")
            raw = pil_image.tobytes()
            surface = pygame.image.frombuffer(raw, pil_image.size, "RGBA").copy()
            return surface.convert_alpha()
        except Exception as pil_exc:
            print(
                f"[WARN] failed to load texture '{path}' with pygame ({image_exc}) "
                f"and Pillow ({pil_exc}). Using procedural placeholder."
            )
            return build_placeholder_texture(
                fallback_size[0],
                fallback_size[1],
                fallback_a,
                fallback_b,
            )


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Pygame unified rendering showcase clone")
    parser.add_argument("--width", type=int, default=1360, help="Window width")
    parser.add_argument("--height", type=int, default=840, help="Window height")
    parser.add_argument("--fps", type=float, default=60.0, help="Target FPS cap")
    parser.add_argument("--window-x", type=int, default=None, help="Window X position")
    parser.add_argument("--window-y", type=int, default=None, help="Window Y position")
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    if args.window_x is not None and args.window_y is not None:
        os.environ["SDL_VIDEO_WINDOW_POS"] = f"{args.window_x},{args.window_y}"

    pygame.init()
    pygame.display.set_caption("Pygame - Unified Rendering Showcase Clone | FPS: 0.0")
    screen = pygame.display.set_mode(
        (args.width, args.height),
        pygame.RESIZABLE | pygame.DOUBLEBUF,
    )
    clock = pygame.time.Clock()
    font = None
    try:
        pygame.font.init()
        font = pygame.font.SysFont("monospace", 16)
    except Exception as exc:
        print(f"[WARN] pygame font unavailable ({exc}). Text overlay disabled.")

    repo_root = Path(__file__).resolve().parents[1]
    image_main_path = repo_root / "images/1_lower-res.png"
    image_lowres_path = repo_root / "images/1_lower-res.png"
    if not image_main_path.exists() or not image_lowres_path.exists():
        raise FileNotFoundError(
            "Expected image assets not found. Ensure images/1.png and images/1_lower-res.png exist."
        )

    image_main = load_texture_with_fallback(
        image_main_path,
        fallback_size=(256, 256),
        fallback_a=(245, 210, 245),
        fallback_b=(240, 240, 190),
    )
    image_lowres = load_texture_with_fallback(
        image_lowres_path,
        fallback_size=(128, 128),
        fallback_a=(235, 165, 235),
        fallback_b=(220, 220, 165),
    )

    cached_main_size: tuple[int, int] | None = None
    cached_main_scaled: pygame.Surface | None = None
    cached_lowres_size: tuple[int, int] | None = None
    cached_lowres_rotated: pygame.Surface | None = None
    cached_dynamic_size: tuple[int, int] | None = None
    cached_dynamic_scaled: pygame.Surface | None = None

    probe_x = 1020.0
    probe_y = 260.0
    move_speed = 420.0

    show_bulk = True
    show_primitives = True
    show_dynamic_texture = True
    paused = False
    sim_time = 0.0
    phase_shift = random.uniform(0.0, 10.0)

    dynamic_texture_hz = 24.0
    dynamic_texture_dt = 1.0 / dynamic_texture_hz
    dynamic_tex_accumulator = dynamic_texture_dt
    bulk_bar_count = 120

    dyn_tex_w = 128
    dyn_tex_h = 128
    dynamic_bytes = build_dynamic_rgba(dyn_tex_w, dyn_tex_h, sim_time, phase_shift)
    dynamic_surface = pygame.image.frombuffer(
        dynamic_bytes, (dyn_tex_w, dyn_tex_h), "RGBA"
    ).copy()

    caption_timer = 0.0
    running = True

    print("Pygame Showcase Controls:")
    print("  WASD / Arrow Keys: Move probe")
    print("  Left click: Teleport probe")
    print("  Right click: Randomize visualization phase")
    print("  Space: Pause/Resume animation")
    print("  B / P / V: Toggle bulk/primitives/bytes-texture overlays")
    print("  Escape: Quit")

    while running:
        dt = min(clock.tick(max(1, int(args.fps))) / 1000.0, 0.1)
        caption_timer += dt
        if caption_timer >= 0.20:
            pygame.display.set_caption(
                f"Pygame - Unified Rendering Showcase Clone | FPS: {clock.get_fps():.1f}"
            )
            caption_timer = 0.0

        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
            elif event.type == pygame.KEYDOWN:
                if event.key == pygame.K_ESCAPE:
                    running = False
                elif event.key == pygame.K_SPACE:
                    paused = not paused
                elif event.key == pygame.K_b:
                    show_bulk = not show_bulk
                elif event.key == pygame.K_p:
                    show_primitives = not show_primitives
                elif event.key == pygame.K_v:
                    show_dynamic_texture = not show_dynamic_texture
            elif event.type == pygame.MOUSEBUTTONDOWN:
                if event.button == 1:
                    probe_x, probe_y = event.pos
                elif event.button == 3:
                    phase_shift = random.uniform(0.0, 12.0)
                    dynamic_tex_accumulator = dynamic_texture_dt
            elif event.type == pygame.VIDEORESIZE:
                screen = pygame.display.set_mode(
                    (event.w, event.h),
                    pygame.RESIZABLE | pygame.DOUBLEBUF,
                )

        if not paused:
            sim_time += dt

        width, height = screen.get_size()
        mouse_x, mouse_y = pygame.mouse.get_pos()
        keys = pygame.key.get_pressed()

        move_x = 0.0
        move_y = 0.0
        if keys[pygame.K_a] or keys[pygame.K_LEFT]:
            move_x -= 1.0
        if keys[pygame.K_d] or keys[pygame.K_RIGHT]:
            move_x += 1.0
        if keys[pygame.K_w] or keys[pygame.K_UP]:
            move_y -= 1.0
        if keys[pygame.K_s] or keys[pygame.K_DOWN]:
            move_y += 1.0

        probe_x = clamp(probe_x + move_x * move_speed * dt, 0.0, float(width))
        probe_y = clamp(probe_y + move_y * move_speed * dt, 0.0, float(height))

        pulse = 0.5 + 0.5 * math.sin(sim_time * 1.1 + phase_shift * 0.2)
        tl = colorf(0.04, 0.05, 0.11 + 0.10 * pulse)
        bl = colorf(0.07, 0.11 + 0.18 * pulse, 0.23)
        br = colorf(0.17 + 0.18 * pulse, 0.07, 0.22)
        tr = colorf(0.08, 0.07 + 0.10 * pulse, 0.16 + 0.12 * pulse)

        gradient_seed = pygame.Surface((2, 2))
        gradient_seed.set_at((0, 0), tl)
        gradient_seed.set_at((0, 1), bl)
        gradient_seed.set_at((1, 1), br)
        gradient_seed.set_at((1, 0), tr)
        gradient_surface = pygame.transform.smoothscale(gradient_seed, (width, height))
        screen.blit(gradient_surface, (0, 0))

        panel_rect = pygame.Rect(
            24,
            24,
            int(width * 0.50 - 40.0),
            int(height * 0.60),
        )
        pygame.draw.rect(screen, (9, 11, 18), panel_rect)
        pygame.draw.rect(screen, (220, 230, 240), panel_rect, 1)

        textured_center = (int(width * 0.18), int(height * 0.46))
        textured_size = (
            max(32, int(panel_rect.width * 0.31)),
            max(32, int(panel_rect.height * 0.43)),
        )
        if cached_lowres_size != textured_size:
            lowres_scaled = pygame.transform.smoothscale(image_lowres, textured_size)
            cached_lowres_rotated = pygame.transform.rotozoom(lowres_scaled, 8.0, 1.0)
            cached_lowres_size = textured_size
        if cached_lowres_rotated is not None:
            text_rect = cached_lowres_rotated.get_rect(center=textured_center)
            screen.blit(cached_lowres_rotated, text_rect)

        solid_poly = rotated_rect_points(
            center=(width * 0.32, height * 0.42),
            size=(panel_rect.width * 0.36, panel_rect.height * 0.25),
            angle_deg=-25.0,
        )
        pygame.draw.polygon(screen, (242, 186, 104), solid_poly)

        accent_rect = pygame.Rect(
            int(width * 0.24),
            int(height * 0.60),
            int(panel_rect.width * 0.42),
            int(panel_rect.height * 0.09),
        )
        pygame.draw.rect(screen, (74, 206, 222), accent_rect)

        image_size = int(min(width, height) * 0.20)
        if cached_main_size != (image_size, image_size):
            cached_main_scaled = pygame.transform.smoothscale(
                image_main,
                (image_size, image_size),
            )
            cached_main_size = (image_size, image_size)
        if cached_main_scaled is not None:
            screen.blit(cached_main_scaled, (int(width * 0.72), int(height * 0.06)))

        dyn_size = int(min(width, height) * 0.34)
        dyn_x = int(width * 0.56)
        dyn_y = int(height * 0.24)

        if show_dynamic_texture:
            dynamic_tex_accumulator += dt
            dynamic_updated = False
            if dynamic_tex_accumulator >= dynamic_texture_dt:
                dynamic_bytes = build_dynamic_rgba(dyn_tex_w, dyn_tex_h, sim_time, phase_shift)
                dynamic_surface = pygame.image.frombuffer(
                    dynamic_bytes,
                    (dyn_tex_w, dyn_tex_h),
                    "RGBA",
                ).copy()
                dynamic_tex_accumulator = 0.0
                dynamic_updated = True

            if (
                cached_dynamic_size != (dyn_size, dyn_size)
                or cached_dynamic_scaled is None
                or dynamic_updated
            ):
                cached_dynamic_scaled = pygame.transform.smoothscale(
                    dynamic_surface,
                    (dyn_size, dyn_size),
                )
                cached_dynamic_size = (dyn_size, dyn_size)

            if cached_dynamic_scaled is not None:
                screen.blit(cached_dynamic_scaled, (dyn_x, dyn_y))

        if show_primitives:
            pygame.draw.rect(screen, (230, 238, 255), panel_rect, 2)
            pygame.draw.rect(
                screen,
                (230, 238, 255),
                pygame.Rect(dyn_x - 10, dyn_y - 10, dyn_size + 20, dyn_size + 20),
                2,
            )
            pygame.draw.line(
                screen,
                (220, 220, 220),
                (int(probe_x), int(probe_y)),
                (int(mouse_x), int(mouse_y)),
                1,
            )
            pygame.draw.circle(screen, (250, 107, 61), (int(probe_x), int(probe_y)), 10, 0)
            pygame.draw.circle(screen, (235, 235, 235), (int(probe_x), int(probe_y)), 26, 2)

            for i in range(32):
                px = int(36 + (i % 8) * 3 + math.sin(sim_time * 5.0 + i) * 1.2)
                py = int(36 + (i // 8) * 3)
                if 0 <= px < width and 0 <= py < height:
                    screen.set_at((px, py), (255, 255, 255))

        if show_bulk:
            commands = build_bulk_overlay_commands(
                width,
                height,
                sim_time,
                probe_x,
                probe_y,
                bulk_bar_count,
            )
            draw_bulk_overlay(screen, commands)

        if font is not None:
            status_lines = [
                "Pygame clone | B:bulk  P:primitives  V:bytes texture  Space:pause",
                f"bulk={show_bulk} primitives={show_primitives} bytes={show_dynamic_texture}",
            ]
            for i, line in enumerate(status_lines):
                text = font.render(line, True, (220, 230, 245))
                screen.blit(text, (20, 20 + i * 18))

        pygame.display.flip()

    pygame.quit()


if __name__ == "__main__":
    main()
