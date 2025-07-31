"""
Example demonstrating the improved enum-based event system.
This shows how to use the new InputEvent enums.
"""

import sys
import os
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from pyg_engine import Engine, GameObject, Collider, BoxCollider, Input, RigidBody, Size
from pygame import Vector2

def collision_enter_callback(collision_info):
    """Callback for collision enter events."""
    print(f"Collision ENTER: {collision_info.other_gameobject.name} collided with {collision_info.other_collider.game_object.name}")
    print(f"  Contact point: {collision_info.contact_point}")
    print(f"  Penetration depth: {collision_info.penetration_depth}")

def collision_stay_callback(collision_info):
    """Callback for collision stay events."""
    print(f"Collision STAY: {collision_info.other_gameobject.name} staying in contact")

def collision_exit_callback(collision_info):
    """Callback for collision exit events."""
    print(f"Collision EXIT: {collision_info.other_gameobject.name} separated")

def trigger_enter_callback(collision_info):
    """Callback for trigger enter events."""
    print(f"Trigger ENTER: {collision_info.other_gameobject.name} entered trigger")

def trigger_exit_callback(collision_info):
    """Callback for trigger exit events."""
    print(f"Trigger EXIT: {collision_info.other_gameobject.name} left trigger")

# Note: Penetration and contact point callbacks are not available in the current collision system

def input_event_callback(event_type, key=None):
    """Generic input event callback."""
    if event_type == Input.InputEvent.KEY_DOWN:
        print(f"Key DOWN: {key}")
    elif event_type == Input.InputEvent.KEY_UP:
        print(f"Key UP: {key}")
    elif event_type == Input.InputEvent.MOUSE_BUTTON_DOWN:
        print(f"Mouse button DOWN: {key}")
    elif event_type == Input.InputEvent.MOUSE_BUTTON_UP:
        print(f"Mouse button UP: {key}")
    elif event_type == Input.InputEvent.MOUSE_SCROLL_UP:
        print("Mouse scroll UP")
    elif event_type == Input.InputEvent.MOUSE_SCROLL_DOWN:
        print("Mouse scroll DOWN")
    elif event_type == Input.InputEvent.WINDOW_FOCUS_GAINED:
        print("Window focus GAINED")
    elif event_type == Input.InputEvent.WINDOW_FOCUS_LOST:
        print("Window focus LOST")

def main():
    """Main function demonstrating the enum-based event system."""
    print("=== Enum-Based Event System Example ===")
    print("This example demonstrates the improved event system using enums.")
    print("Press WASD to move the player, mouse to look around.")
    print("Watch the console for collision and input events.\n")

    # Create engine
    engine = Engine(size=Size(w=800, h=600), windowName="Enum Event System Example")

    # Create player object
    player = GameObject(name="Player", position=Vector2(400, 300))
    player.add_component(BoxCollider, width=50, height=50)
    player.add_component(RigidBody, mass=1.0)
    engine.addGameObject(player)

    # Create some walls
    walls = []
    wall_positions = [
        (200, 200), (600, 200), (400, 100), (400, 500)
    ]

    for i, pos in enumerate(wall_positions):
        wall = GameObject(name=f"Wall_{i}", position=Vector2(pos[0], pos[1]))
        wall.add_component(BoxCollider, width=100, height=20)
        wall.add_component(RigidBody, mass=0.0)  # Static wall
        engine.addGameObject(wall)
        walls.append(wall)

    # Create a trigger zone
    trigger = GameObject(name="TriggerZone", position=Vector2(400, 400))
    trigger.add_component(BoxCollider, width=80, height=80, is_trigger=True)
    engine.addGameObject(trigger)

    # Add collision callbacks using the standard collision system
    player_collider = player.get_component(BoxCollider)
    if player_collider:
        player_collider.add_collision_callback('enter', collision_enter_callback)
        player_collider.add_collision_callback('stay', collision_stay_callback)
        player_collider.add_collision_callback('exit', collision_exit_callback)

    # Add trigger callbacks
    trigger_collider = trigger.get_component(BoxCollider)
    if trigger_collider:
        trigger_collider.add_collision_callback('enter', trigger_enter_callback)
        trigger_collider.add_collision_callback('exit', trigger_exit_callback)

    # Add input event callbacks
    def check_input_events():
        """Check for input events and call callbacks."""
        input = engine.input

        # Check for key events
        for key in [Input.Keybind.W, Input.Keybind.A, Input.Keybind.S, Input.Keybind.D,
                   Input.Keybind.K_SPACE, Input.Keybind.K_ESCAPE]:
            if input.get_event_state(Input.InputEvent.KEY_DOWN, key.value):
                input_event_callback(Input.InputEvent.KEY_DOWN, key.name)
            if input.get_event_state(Input.InputEvent.KEY_UP, key.value):
                input_event_callback(Input.InputEvent.KEY_UP, key.name)

        # Check for mouse events
        for button in [0, 1, 2]:  # Left, middle, right mouse buttons
            if input.get_event_state(Input.InputEvent.MOUSE_BUTTON_DOWN, button):
                input_event_callback(Input.InputEvent.MOUSE_BUTTON_DOWN, f"Button_{button}")
            if input.get_event_state(Input.InputEvent.MOUSE_BUTTON_UP, button):
                input_event_callback(Input.InputEvent.MOUSE_BUTTON_UP, f"Button_{button}")

        # Check for scroll events
        if input.get_event_state(Input.InputEvent.MOUSE_SCROLL_UP):
            input_event_callback(Input.InputEvent.MOUSE_SCROLL_UP)
        if input.get_event_state(Input.InputEvent.MOUSE_SCROLL_DOWN):
            input_event_callback(Input.InputEvent.MOUSE_SCROLL_DOWN)

        # Check for window events
        if input.get_event_state(Input.InputEvent.WINDOW_FOCUS_GAINED):
            input_event_callback(Input.InputEvent.WINDOW_FOCUS_GAINED)
        if input.get_event_state(Input.InputEvent.WINDOW_FOCUS_LOST):
            input_event_callback(Input.InputEvent.WINDOW_FOCUS_LOST)

    # Add the input event checker to the engine's update loop
    def update_with_input_check(engine):
        check_input_events()

    # Add the input checker to the engine's runnable system
    engine.add_runnable(update_with_input_check, 'update')

    # Run the engine
    try:
        engine.start()
    except KeyboardInterrupt:
        print("\nExample terminated by user.")
    finally:
        print("\n=== Example Complete ===")
        print("The enum-based event system provides:")
        print("- Type-safe event handling with InputEvent enums")
        print("- Enhanced input events (MOUSE_MOTION, WINDOW_FOCUS, etc.)")
        print("- Backward compatibility with legacy string-based events")
        print("- Better error handling and debugging")

if __name__ == "__main__":
    main()
