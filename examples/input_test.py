import pyg_engine
from pyg_engine import Engine, Input, Runnable


engine = Engine()
input = engine.input

def get_A(engine):
    if(input.get(Input.Keybind.A)):
        print("A is pressed")

def get_Q(engine):
    if(input.get(Input.Keybind.Q)):
        print("Q is pressed")
        engine.stop()

def get_mouse_left(engine):
    if(input.get(Input.Keybind.MOUSE_LEFT)):
        print("Left mouse button is pressed")

def get_mouse_right(engine):
    if(input.get(Input.Keybind.MOUSE_RIGHT)):
        print("Right mouse button is pressed")

def get_horizontal_axis(engine):
    horizontal_value = input.get_axis(Input.Axis.HORIZONTAL)
    if horizontal_value != 0:
        print(f"Horizontal axis: {horizontal_value}")

def get_mouse_movement(engine):
    mouse_rel = input.mouse.get_rel()
    mouse_rel_x = mouse_rel[0]
    mouse_rel_y = mouse_rel[1]
    if mouse_rel_x != 0 or mouse_rel_y != 0 or input.get(Input.Keybind.MOUSE_LEFT):
        mouse_pos = input.mouse.get_pos()
        mouse_x = mouse_pos[0]
        mouse_y = mouse_pos[1]
        print(f"Mouse movement detected:")
        print(f"  Relative: X={mouse_rel_x:.2f}, Y={mouse_rel_y:.2f}")
        print(f"  Absolute: X={mouse_x:.0f}, Y={mouse_y:.0f}")

engine.add_runnable(get_A)
engine.add_runnable(get_Q)
engine.add_runnable(get_mouse_left)
engine.add_runnable(get_mouse_right)
engine.add_runnable(get_horizontal_axis)
engine.add_runnable(get_mouse_movement)

engine.start()

