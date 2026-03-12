import pyg_engine as pyg


def main() -> None:
    engine = pyg.Engine()

    # Replace these paths with real TTF/OTF files in your project.
    engine.register_font_family(
        "inter",
        regular="assets/fonts/Inter-Regular.ttf",
        bold="assets/fonts/Inter-Bold.ttf",
        italic="assets/fonts/Inter-Italic.ttf",
        bold_italic="assets/fonts/Inter-BoldItalic.ttf",
    )

    # HUD text using the registered family.
    hud_text = pyg.Text(
        "HUD: Styled Text",
        position=pyg.Vec2(24, 24),
        color=pyg.Color.WHITE,
        font_size=28.0,
        font_family="inter",
        font_weight="bold",
    )

    subtitle = pyg.Text(
        "Kerning + italic example",
        position=pyg.Vec2(24, 60),
        color=pyg.Color.CYAN,
        font_size=18.0,
        font_family="inter",
        font_style="italic",
        kerning=True,
    )

    world_label = pyg.GameObject("WorldLabel")
    world_label.position = pyg.Vec2(0.0, 1.0)
    world_label.scale = pyg.Vec2(0.004, 0.004)

    world_text = pyg.TextMeshComponent(
        "World-space family text",
        font_size=42.0,
        font_family="inter",
        font_weight="bold",
    )
    world_text.color = pyg.Color.WHITE
    world_label.add_text_mesh_component(world_text)
    engine.add_game_object(world_label)

    menu_panel = pyg.Panel(x=20, y=120, width=280, height=140)
    menu_panel.set_background_color(0.1, 0.1, 0.12, 0.95)
    menu_panel.set_border(1.0, 0.4, 0.4, 0.5, 1.0)

    menu_label = pyg.Label(
        "Main Menu",
        x=16,
        y=16,
        font_size=22.0,
        align="left",
        font_family="inter",
        font_weight="bold",
    )
    menu_label.set_color(1.0, 1.0, 1.0, 1.0)

    play_button = pyg.Button(
        "Play",
        x=16,
        y=56,
        width=140,
        height=40,
        font_family="inter",
        font_weight="bold",
    )

    menu_panel.add_children([menu_label, play_button])
    engine.ui.add(menu_panel)

    text_w, text_h = engine.measure_text(
        "Measured Title",
        font_size=24.0,
        font_family="inter",
        font_weight="bold",
    )
    engine.draw_text(
        f"Measured: {text_w:.0f}x{text_h:.0f}",
        24,
        94,
        pyg.Color.YELLOW,
        font_size=16.0,
        font_family="inter",
    )

    engine.draw([hud_text, subtitle])
    engine.camera.position = pyg.Vec2(0.0, 0.0)
    engine.camera.viewport_size = pyg.Vec2(10.0, 5.625)
    engine.camera.aspect_mode = pyg.CameraAspectMode.FIT_BOTH
    engine.run(title="Font Family Demo")


if __name__ == "__main__":
    main()
