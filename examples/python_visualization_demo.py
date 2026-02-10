import math

from pyg_engine import Color, Engine


def make_checkerboard_rgba(width: int, height: int, cell_size: int = 8) -> bytes:
    data = bytearray(width * height * 4)
    for y in range(height):
        for x in range(width):
            idx = (y * width + x) * 4
            is_dark = ((x // cell_size) + (y // cell_size)) % 2 == 0
            if is_dark:
                r, g, b = (30, 150, 255)
            else:
                r, g, b = (230, 245, 255)
            data[idx : idx + 4] = bytes((r, g, b, 255))
    return bytes(data)


def main() -> None:
    engine = Engine()
    engine.initialize(title="Visualization/Texture Demo", width=960, height=600)

    texture_width = 128
    texture_height = 128
    checkerboard = make_checkerboard_rgba(texture_width, texture_height)

    while engine.poll_events():
        display_width, display_height = engine.get_display_size()
        t = engine.elapsed_time

        # Animated corner colors show smooth gradient interpolation.
        pulse = 0.5 + 0.5 * math.sin(t * 1.4)
        top_left_color = Color(0.06, 0.08, 0.15 + 0.2 * pulse, 1.0)
        bottom_left_color = Color(0.08, 0.35 + 0.25 * pulse, 0.45, 1.0)
        bottom_right_color = Color(0.5 + 0.25 * pulse, 0.15, 0.35, 1.0)
        top_right_color = Color(0.12, 0.1 + 0.2 * pulse, 0.35 + 0.3 * pulse, 1.0)

        image_size = 240.0
        image_x = display_width * 0.5 - image_size * 0.5
        image_y = display_height * 0.5 - image_size * 0.5

        engine.clear_draw_commands()
        engine.draw_gradient_rect(
            0.0,
            0.0,
            float(display_width),
            float(display_height),
            top_left_color,
            bottom_left_color,
            bottom_right_color,
            top_right_color,
        )
        engine.draw_image_from_bytes(
            image_x,
            image_y,
            image_size,
            image_size,
            "demo_checkerboard",
            checkerboard,
            texture_width,
            texture_height,
            layer=1,
        )
        engine.draw_text(
            "Visualization/Texture Demo",
            24.0,
            24.0,
            Color(0.97, 0.99, 1.0, 0.98),
            font_size=24.0,
            letter_spacing=1.0,
            layer=10,
            z_index=1.0,
        )
        engine.draw_text(
            f"elapsed: {t:0.2f}s\nfont: built-in open-source default",
            24.0,
            58.0,
            Color(0.90, 0.95, 1.0, 0.95),
            font_size=16.0,
            line_spacing=4.0,
            layer=10,
            z_index=1.01,
        )

        engine.update()
        engine.render()


if __name__ == "__main__":
    main()
