import math

from pyg_engine import Color, DrawCommand, Engine


def main() -> None:
    """
    Bulk draw demo for pyg_engine.

    Shows how to use the bulk draw API using DrawCommand objects to draw a scene.
    """

    engine = Engine()
    engine.start_manual(
        title="Bulk Draw Demo",
        width=1100,
        height=700,
        redraw_on_change_only=False,
    )

    bar_count = 220
    bar_width = 4.0
    bar_spacing = 5.0

    background_tl = Color(0.04, 0.04, 0.09, 1.0)
    background_bl = Color(0.06, 0.08, 0.18, 1.0)
    background_br = Color(0.10, 0.05, 0.16, 1.0)
    background_tr = Color(0.07, 0.05, 0.12, 1.0)
    bar_color = Color(0.35, 0.90, 0.95, 0.90)
    line_color = Color(0.95, 0.85, 0.20, 1.0)

    while engine.poll_events():
        display_width, display_height = engine.get_display_size()
        t = engine.elapsed_time

        commands: list[DrawCommand] = [
            DrawCommand.gradient_rect(
                0.0,
                0.0,
                float(display_width),
                float(display_height),
                background_tl,
                background_bl,
                background_br,
                background_tr,
            )
        ]

        center_y = display_height * 0.55
        baseline = display_height * 0.10
        start_x = (display_width - (bar_count * bar_spacing)) * 0.5

        for i in range(bar_count):
            x = start_x + i * bar_spacing
            wave = math.sin(i * 0.11 + t * 2.2) * 0.5 + 0.5
            h = baseline + wave * (display_height * 0.45)
            commands.append(
                DrawCommand.rectangle(
                    float(x),
                    float(center_y - h),
                    bar_width,
                    float(h),
                    bar_color,
                )
            )

        for i in range(1, bar_count):
            x0 = start_x + (i - 1) * bar_spacing
            x1 = start_x + i * bar_spacing
            y0 = center_y - (math.sin((i - 1) * 0.11 + t * 1.6) * 120.0)
            y1 = center_y - (math.sin(i * 0.11 + t * 1.6) * 120.0)
            commands.append(
                DrawCommand.line(float(x0), float(y0), float(x1), float(y1), line_color, 2.0)
            )

        engine.clear_draw_commands()
        engine.add_draw_commands(commands)
        engine.update()
        engine.render()


if __name__ == "__main__":
    main()
