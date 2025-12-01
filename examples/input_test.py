import pyg
from pyg import *

class InputTest(GameObject):
    def __init__(self, engine):
        # Input Test Class logs every axis, mouse, keyboard, and joystick event

        super.__init__(engine)
        pyg.log("Created Input Test GameObject")

        # Input axis are re-bindable
        input = self.engine.input
        axis = input.axis
        axis.change_binding(Axis.Fire1, {Mouse.LB, Joystick.RT, KB.F})

        # Joystick axiscontain deadzone modifiers
        input.joystick.deadzone = 0.02

        # Input event assignments
        input.events.on_keydown(KB.R, ("R") -> do_something(str))

        # Call engine quit event on escape
        input.events.on_keydown(KB.ESC, engine.stop())

    def do_something(self, key: str):
        pyg.log(f"Key '{str}' pressed")

    def start(self):
        pyg.log("Starting Input Test GameObject")

    def update(self, deltatime: DeltaTime):
        input = self.engine.input

        # NOTE: All axis events are all floats [-1,1]
        #       All Keyboard events are bool True/False
        #       All Mouse Locations are Vector2: (x, y)
        #           Mouse Buttons are bool True/False

        # --------------------------------------------------------------------

        # Axis contains bindings for "Horizontal", "Vertical", "Fire", etc.
        axis_events = input.axis

        # Keyboard events include all alphanumeric, keyup, keydown, hold, etc.
        keyboard_events = input.keyboard

        # Mouse include mouse window/world position, button up/down/hold, etc.
        mouse_events = input.mouse

        # --------------------------------------------------------------------

        # Horizontal (key A/D, key <-/->, Joystick <-/->)
        horizontal = axis_events.get(Axis.Horizontal)

        # Vertical (key W/S, key up/down, Joystick up/down)
        vertical = axis_events.get(Axis.Vertical)

        # Fire [1-4], but we'll just show fire 1 (Left mouse click,
        #           Right joystick trigger, enter)
        fire1 = axis_events.get(Axis.Fire1)

        # Jump (space key, 'A' joystick)
        jump = axis_events(Axis.Jump)

        # Others include crouch, sprint, back

        # NOTE: You can set bindings, add keybindings like this:
        #                {Mouse Left Button, Joystick Right Trigger, Keyboard F}
        # axis_events.change_binding(Axis.Fire1, {Mouse.LB, Joystick.RT, KB.F})

        if(abs(horizontal) > 0):
            pyg.log(f"Horizontal Axis: ({horizontal})")

        if(abs(vertical) > 0):
            pyg.log(f"Vertical Axis: ({vertical})")

        if(abs(fire1) > 0):
            pyg.log(f"Fire1 Axis: ({fire1})")

        if(abs(jump) > 0):
            pyg.log(f"Fire1 Axis: ({jump})")

        # --------------------------------------------------------------------

        # You can get individual keydown
        key_i = keyboard_events.get(KB.I)

        if key_i:
            pyg.log("Key I is pressed")

        # get_keys() returns an array of currently down keys
        keys_down = keyboard_events.get_keys()

        down_str = ""
        for key in keys_down:
            down_str += f"{key}, "

        if len(keys_down > 0):
            pyg.log(f"Keys down: {down_str}")

        # --------------------------------------------------------------------

        if mouse_events.get(Mouse.LB):
            pyg.log("Left Mouse Button Pressed")

        if mouse_events.get(Mouse.RB):
            pyg.log("Right Mouse Button Pressed")

        if mouse_events.get(Mouse.MB):
            pyg.log("Middle Mouse Button Pressed")

        # Check if the mouse is moving
        if mouse_events.delta.x ** 2 + mouse_events.delta.y ** 2 > 0:
            # Window position in px (Vector2)
            pos = mouse_events.position

            # Mouse location in worldspace (Vector2)
            pos_worldspace = mouse_events.to_worldspace(pos)

            pyg.log(f"Mouse Moved! Position: ({pos.x}, {pos.y}), World: ({pos_worldspace.x}, {pos_worldspace.y}), Delta: ({mouse_events.delta.x}, {mouse_events.delta.y})")

        if mouse_events.scroll.delta ** 2 > 0:
            pyg.log(f"Scroll wheel moved! Delta: ({mouse_events.scroll.delta})")

def main():
    print("Starting input test!")

    engine = pyg.Engine()
    engine.window.size = (800, 600)

    input_test_obj = InputTest(engine)

    # We can directly attach the gameobject to the default scene
    engine.scene.add_gameobject(input_test_obj)

    # Start the engine and mechanics
    engine.begin()

if __name__ == "__main__":
    main()
