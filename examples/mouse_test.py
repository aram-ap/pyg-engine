#!/usr/bin/env python3
"""
Simple mouse input test to verify the mouse system is working.
"""

import pyg_engine
from pyg_engine import Engine, Input, Runnable

engine = Engine()
input = engine.input

def test_mouse_class(engine):
    """Test the new Mouse class functionality."""
    mouse = input.mouse

    # Get relative movement (only print when there's movement)
    rel = mouse.get_rel()
    if rel != (0, 0):
        print(f"Mouse relative movement: {rel}")
        print(f"Mouse position: {mouse.get_pos()}")

    # Get scroll wheel
    scroll = mouse.get_scroll()
    if scroll != (0, 0):
        print(f"Mouse scroll: {scroll}")

    # Check button events (only print when buttons are pressed/released)
    if mouse.get_button_down(0):  # Left button
        print("Left mouse button was pressed")
    if mouse.get_button_up(0):
        print("Left mouse button was released")
    if mouse.get_button_down(1):  # Middle button
        print("Middle mouse button was pressed")
    if mouse.get_button_up(1):
        print("Middle mouse button was released")
    if mouse.get_button_down(2):  # Right button
        print("Right mouse button was pressed")
    if mouse.get_button_up(2):
        print("Right mouse button was released")

def test_legacy_system(engine):
    """Test the legacy mouse system for comparison."""
    # Test legacy axis-based mouse input
    mouse_rel_x = input.get_raw_axis(Input.Axis.MOUSE_REL_X)
    mouse_rel_y = input.get_raw_axis(Input.Axis.MOUSE_REL_Y)

    if mouse_rel_x != 0 or mouse_rel_y != 0:
        print(f"Legacy system - Mouse relative: ({mouse_rel_x}, {mouse_rel_y})")

engine.add_runnable(test_mouse_class)
engine.add_runnable(test_legacy_system)

print("Mouse test started. Move the mouse, click buttons, and scroll to test the new Mouse class.")
print("Press Q to quit.")

engine.start()
