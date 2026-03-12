"""
UI Demo - Demonstrates the PyG Engine UI system
Shows buttons, panels, labels, and root-panel tree composition.
"""

from pyg_engine import Engine, Button, Panel, Label, Keys

def main() -> None:
    engine = Engine(log_level="info")

    click_count = [0]  # Use list to allow modification in closure
    label_obj = [None]  # Store label reference

    def on_increment_click(engine: Engine):
        """Handle increment button click."""
        engine.log(f"Incremented! Count: {click_count[0]}")
        click_count[0] += 1
        if label_obj[0]:
            label_obj[0].text = f"Clicks: {click_count[0]}"

    def on_decrement_click(engine: Engine):
        """Handle decrement button click."""
        engine.log(f"Decremented! Count: {click_count[0]}")
        click_count[0] -= 1
        if label_obj[0]:
            label_obj[0].text = f"Clicks: {click_count[0]}"

    def on_reset_click(engine: Engine):
        """Handle reset button click."""
        engine.log("Reset counter!")
        click_count[0] = 0
        if label_obj[0]:
            label_obj[0].text = f"Clicks: {click_count[0]}"

    def update(dt: float, engine: Engine, user_data):
        if engine.input.key_down(Keys.ESCAPE):
            return False

    # Create a panel as a container
    panel = Panel(x=50, y=50, width=700, height=500, depth=0)
    panel.set_background_color(0.95, 0.95, 0.95, 1.0)
    panel.set_border(2, 0.3, 0.3, 0.3, 1.0)

    # Title label
    title = Label(
        "PyG Engine UI Demo",
        x=350,
        y=30,
        font_size=24,
        align="center",
        depth=1
    )
    title.set_color(0.1, 0.1, 0.1, 1.0)

    # Description label
    desc = Label(
        "Click the buttons below to test the UI system",
        x=380,
        y=110,
        font_size=12,
        align="center",
        depth=1
    )
    desc.set_color(0.4, 0.4, 0.4, 1.0)

    # Counter label
    counter_label = Label(
        f"Clicks: {click_count[0]}",
        x=350,
        y=70,
        font_size=18,
        align="center",
        depth=1
    )
    counter_label.set_color(0.0, 0.5, 0.0, 1.0)
    label_obj[0] = counter_label  # Store reference

    # Button row 1
    btn1 = Button(
        "Increment",
        x=100,
        y=200,
        width=150,
        height=40,
        on_click=on_increment_click,
        trigger_on="press",
        depth=1
    )

    btn2 = Button(
        "Decrement",
        x=275,
        y=200,
        width=150,
        height=40,
        on_click=on_decrement_click,
        trigger_on="press",
        depth=1
    )

    btn3 = Button(
        "Reset",
        x=450,
        y=200,
        width=150,
        height=40,
        on_click=on_reset_click,
        trigger_on="press",
        depth=1
    )

    # Info panel
    info_panel = Panel(x=50, y=300, width=600, height=150, depth=0.5)
    info_panel.set_background_color(0.9, 0.95, 1.0, 1.0)
    info_panel.set_border(1, 0.4, 0.6, 0.8, 1.0)

    # Info labels
    info1 = Label(
        "Features Demonstrated:",
        x=20,
        y=20,
        font_size=14,
        align="left",
        depth=1
    )
    info1.set_color(0.0, 0.0, 0.0, 1.0)

    info2 = Label(
        "- Clickable buttons with callbacks",
        x=30,
        y=50,
        font_size=12,
        align="left",
        depth=1
    )
    info2.set_color(0.2, 0.2, 0.2, 1.0)

    info3 = Label(
        "- Nested panels with borders and backgrounds",
        x=30,
        y=75,
        font_size=12,
        align="left",
        depth=1
    )
    info3.set_color(0.2, 0.2, 0.2, 1.0)

    info4 = Label(
        "- Dynamic text updates",
        x=30,
        y=100,
        font_size=12,
        align="left",
        depth=1
    )
    info4.set_color(0.2, 0.2, 0.2, 1.0)

    info5 = Label(
        "- Depth-based layering",
        x=30,
        y=125,
        font_size=12,
        align="left",
        depth=1
    )
    info5.set_color(0.2, 0.2, 0.2, 1.0)
    info_panel.add_children([info1, info2, info3, info4, info5])
    panel.add_children([title, desc, counter_label, btn1, btn2, btn3, info_panel])
    engine.ui.add(panel)

    engine.log("UI Demo started!")
    engine.log("Click the buttons to interact with the UI.")
    engine.log("Press ESC or close window to quit.")

    # Run the engine
    engine.run(
        title="PyG Engine - UI Demo",
        width=800,
        height=600,
        update=update,
    )

if __name__ == "__main__":
    main()
