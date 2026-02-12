"""
UI Demo - Demonstrates the PyG Engine UI system
Shows buttons, panels, and labels with different styling and layouts.
"""

import math
import time
import pyg_engine as pyg
from pyg_engine import Engine, Button, Panel, Label, Color, MouseButton


def main() -> None:
    engine = Engine()

    click_count = [0]  # Use list to allow modification in closure
    label_obj = [None]  # Store label reference

    def update(dt: float, engine: pyg.Engine, user_data: dict[str, float]) -> bool:
        # if engine.input.key_down(pyg.Keys.ESCAPE):
        #     exit()
        # if engine.input.mouse_button_down(MouseButton.LEFT):
        #     engine.log("Mouse button down")
        pass

    def on_increment_click():
        """Handle increment button click."""
        print(f"Incremented! Count: {click_count[0]}")
        click_count[0] += 1
        if label_obj[0]:
            label_obj[0].text = f"Clicks: {click_count[0]}"

    def on_decrement_click():
        """Handle decrement button click."""
        print(f"Decremented! Count: {click_count[0]}")
        click_count[0] -= 1
        if label_obj[0]:
            label_obj[0].text = f"Clicks: {click_count[0]}"

    def on_reset_click():
        """Handle reset button click."""
        print("Reset counter!")
        click_count[0] = 0
        if label_obj[0]:
            label_obj[0].text = f"Clicks: {click_count[0]}"

    # Create a panel as a container
    panel = Panel(x=50, y=50, width=700, height=500, depth=0)
    panel.set_background_color(0.95, 0.95, 0.95, 1.0)
    panel.set_border(2, 0.3, 0.3, 0.3, 1.0)
    panel.add_to_engine(engine)

    # Title label
    title = Label(
        "PyG Engine UI Demo",
        x=400,
        y=80,
        font_size=24,
        align="center",
        depth=1
    )
    title.set_color(0.1, 0.1, 0.1, 1.0)
    title.add_to_engine(engine)

    # Description label
    desc = Label(
        "Click the buttons below to test the UI system",
        x=400,
        y=120,
        font_size=14,
        align="center",
        depth=1
    )
    desc.set_color(0.4, 0.4, 0.4, 1.0)
    desc.add_to_engine(engine)

    # Counter label
    counter_label = Label(
        f"Clicks: {click_count[0]}",
        x=400,
        y=120,
        font_size=18,
        align="center",
        depth=1
    )
    counter_label.set_color(0.0, 0.5, 0.0, 1.0)
    counter_label.add_to_engine(engine)
    label_obj[0] = counter_label  # Store reference

    # Button row 1
    btn1 = Button(
        "Increment",
        x=150,
        y=250,
        width=150,
        height=40,
        on_click=on_increment_click,
        depth=1
    )
    btn1.add_to_engine(engine)

    # engine.draw_circle(
    #         250,
    #         350,
    #         30,
    #         pyg.Color.RED,
    #         thickness=4.0,
    #         segments=48,
    #         draw_order=4,
    #         )


    btn2 = Button(
        "Decrement",
        x=325,
        y=250,
        width=150,
        height=40,
        on_click=on_decrement_click,
        depth=1
    )
    btn2.add_to_engine(engine)

    btn3 = Button(
        "Reset",
        x=500,
        y=250,
        width=150,
        height=40,
        on_click=on_reset_click,
        depth=1
    )
    btn3.add_to_engine(engine)

    # Info panel
    info_panel = Panel(x=100, y=350, width=600, height=150, depth=0.5)
    info_panel.set_background_color(0.9, 0.95, 1.0, 1.0)
    info_panel.set_border(1, 0.4, 0.6, 0.8, 1.0)
    info_panel.add_to_engine(engine)

    # Info labels
    info1 = Label(
        "Features Demonstrated:",
        x=120,
        y=370,
        font_size=14,
        align="left",
        depth=1
    )
    info1.set_color(0.0, 0.0, 0.0, 1.0)
    info1.add_to_engine(engine)

    info2 = Label(
        "- Clickable buttons with callbacks",
        x=130,
        y=400,
        font_size=12,
        align="left",
        depth=1
    )
    info2.set_color(0.2, 0.2, 0.2, 1.0)
    info2.add_to_engine(engine)

    info3 = Label(
        "- Nested panels with borders and backgrounds",
        x=130,
        y=425,
        font_size=12,
        align="left",
        depth=1
    )
    info3.set_color(0.2, 0.2, 0.2, 1.0)
    info3.add_to_engine(engine)

    info4 = Label(
        "- Dynamic text updates",
        x=130,
        y=450,
        font_size=12,
        align="left",
        depth=1
    )
    info4.set_color(0.2, 0.2, 0.2, 1.0)
    info4.add_to_engine(engine)

    info5 = Label(
        "- Depth-based layering",
        x=130,
        y=475,
        font_size=12,
        align="left",
        depth=1
    )
    info5.set_color(0.2, 0.2, 0.2, 1.0)
    info5.add_to_engine(engine)

    print("UI Demo started!")
    print("Click the buttons to interact with the UI.")
    print("Press ESC or close window to quit.")

    # Run the engine
    engine.run(
        title="PyG Engine - UI Demo",
        width=800,
        height=600,
        update=update
    )


if __name__ == "__main__":
    main()
