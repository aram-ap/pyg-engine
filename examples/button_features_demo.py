"""
Button Features Demo - Demonstrates trigger modes and repeat functionality
"""

import pyg_engine as pyg
from pyg_engine import Engine, Button, Label

def main() -> None:
    engine = Engine()

    # Counter for demonstrations
    press_count = [0]
    release_count = [0]
    hold_count = [0]

    # Labels to show counts
    press_label = [None]
    release_label = [None]
    hold_label = [None]

    def on_press_button():
        press_count[0] += 1
        if press_label[0]:
            press_label[0].text = f"Press count: {press_count[0]}"

    def on_release_button():
        release_count[0] += 1
        if release_label[0]:
            release_label[0].text = f"Release count: {release_count[0]}"

    def on_hold_button():
        hold_count[0] += 1
        if hold_label[0]:
            hold_label[0].text = f"Hold count: {hold_count[0]}"

    # Title
    title = Label(
        "Button Features Demo",
        x=400,
        y=30,
        font_size=24,
        align="center",
        depth=1
    )
    title.set_color(0.1, 0.1, 0.1, 1.0)
    engine.ui.add(title)

    # --- PRESS TRIGGER DEMO ---
    desc1 = Label(
        "1. Trigger on Press (fires on mouse down)",
        x=100,
        y=100,
        font_size=14,
        align="left",
        depth=1
    )
    desc1.set_color(0.2, 0.2, 0.2, 1.0)
    engine.ui.add(desc1)

    press_btn = Button(
        "Press Me",
        x=100,
        y=130,
        width=200,
        height=40,
        on_click=on_press_button,
        trigger_on="press",  # Fire on press instead of release
        depth=1
    )
    engine.ui.add(press_btn)

    press_count_label = Label(
        f"Press count: {press_count[0]}",
        x=320,
        y=140,
        font_size=14,
        align="left",
        depth=1
    )
    press_count_label.set_color(0.0, 0.5, 0.0, 1.0)
    engine.ui.add(press_count_label)
    press_label[0] = press_count_label

    # --- RELEASE TRIGGER DEMO ---
    desc2 = Label(
        "2. Trigger on Release (default - fires on mouse up)",
        x=100,
        y=200,
        font_size=14,
        align="left",
        depth=1
    )
    desc2.set_color(0.2, 0.2, 0.2, 1.0)
    engine.ui.add(desc2)

    release_btn = Button(
        "Click Me",
        x=100,
        y=230,
        width=200,
        height=40,
        on_click=on_release_button,
        trigger_on="release",  # Default behavior
        depth=1
    )
    engine.ui.add(release_btn)

    release_count_label = Label(
        f"Release count: {release_count[0]}",
        x=320,
        y=240,
        font_size=14,
        align="left",
        depth=1
    )
    release_count_label.set_color(0.0, 0.5, 0.0, 1.0)
    engine.ui.add(release_count_label)
    release_label[0] = release_count_label

    # --- HOLD/REPEAT DEMO ---
    desc3 = Label(
        "3. Hold & Repeat (repeats every 100ms while held)",
        x=100,
        y=300,
        font_size=14,
        align="left",
        depth=1
    )
    desc3.set_color(0.2, 0.2, 0.2, 1.0)
    engine.ui.add(desc3)

    hold_btn = Button(
        "Hold Me",
        x=100,
        y=330,
        width=200,
        height=40,
        on_click=on_hold_button,
        trigger_on="release",
        repeat_interval_ms=100,  # Repeat every 100ms while held
        depth=1
    )
    engine.ui.add(hold_btn)

    hold_count_label = Label(
        f"Hold count: {hold_count[0]}",
        x=320,
        y=340,
        font_size=14,
        align="left",
        depth=1
    )
    hold_count_label.set_color(0.0, 0.5, 0.0, 1.0)
    engine.ui.add(hold_count_label)
    hold_label[0] = hold_count_label

    # Instructions
    instructions = Label(
        "Try clicking and holding each button to see the differences!",
        x=400,
        y=420,
        font_size=12,
        align="center",
        depth=1
    )
    instructions.set_color(0.4, 0.4, 0.4, 1.0)
    engine.ui.add(instructions)

    print("Button Features Demo started!")
    print("Try different button interactions:")
    print("  - Press button fires on mouse down")
    print("  - Click button fires on mouse up (standard)")
    print("  - Hold button repeats while held down")

    # Run the engine
    engine.run(
        title="PyG Engine - Button Features Demo",
        width=800,
        height=500,
    )


if __name__ == "__main__":
    main()
