#!/usr/bin/env python3
"""
Visual Runnable System Demo with Camera Controls and Physics
Demonstrates the runnable system with camera controls, mouse-based spawning, and proper physics.
"""

import pygame as pg
from pygame import Color
from pyg_engine import Engine, Priority, Input
from pyg_engine import Vector2, Size, BasicShape
from pyg_engine import GameObject
from pyg_engine import RigidBody, BoxCollider, CircleCollider, Materials

def main():
    """Visual demo showing runnable system with camera controls, mouse spawning, and physics."""

    # Create engine
    engine = Engine(fpsCap=120, size=Size(w=2560, h=1600), windowName="Physics Camera Demo",
                    backgroundColor = "#1E201E", displayMode=pg.RESIZABLE)
    engine.globals.set("paused", False)

    # ===== CAMERA CONTROLS =====

    # 1. Camera movement with WASD
    def camera_movement(engine):
        input = engine.input
        camera_speed = 200 * engine.dt()  # 200 pixels per second

        if input.get(Input.Keybind.W):
            engine.camera.position.y += camera_speed
        if input.get(Input.Keybind.S):
            engine.camera.position.y -= camera_speed
        if input.get(Input.Keybind.A):
            engine.camera.position.x -= camera_speed
        if input.get(Input.Keybind.D):
            engine.camera.position.x += camera_speed

    engine.add_runnable(camera_movement, 'update', Priority.HIGH)

    # 2. Camera zoom with mouse wheel (FIXED - use modern input system)
    def camera_zoom(engine):
        # Check for mouse wheel events
        if engine.input.get_event_state('mouse_scroll_up'):
            zoom_factor = 1.1
            engine.camera.zoom *= zoom_factor
            # Clamp zoom between 0.1 and 5.0
            engine.camera.zoom = max(0.1, min(5.0, engine.camera.zoom))
            print(f"Camera zoom: {engine.camera.zoom:.2f}")
        elif engine.input.get_event_state('mouse_scroll_down'):
            zoom_factor = 0.9
            engine.camera.zoom *= zoom_factor
            # Clamp zoom between 0.1 and 5.0
            engine.camera.zoom = max(0.1, min(5.0, engine.camera.zoom))
            print(f"Camera zoom: {engine.camera.zoom:.2f}")

    engine.add_runnable(camera_zoom, 'update', Priority.HIGH)

    # 3. Reset camera position
    def reset_camera(engine):
        engine.camera.position = Vector2(0, 0)
        engine.camera.zoom = 1.0
        print("Camera reset!")

    engine.add_runnable(reset_camera, 'key_press', Priority.NORMAL, key=Input.Keybind.R.value)

    # ===== MOUSE-BASED SPAWNING WITH PHYSICS =====

    # 4. Spawn system with different object types
    def spawn_at_mouse(engine):
        import random
        import math
        from pygame import Vector2, Color

        # Get current spawn mode
        spawn_mode = engine.globals.get('spawn_mode', 'circle', 'spawning')

        # Get mouse position in world coordinates
        mouse_pos = engine.input.mouse.get_pos()
        mouse_world_pos = engine.camera.screen_to_world(mouse_pos)


        # Pleasant colors using pygame color strings
        pleasant_colors = [
            "lightpink", "lightblue", "lightgreen", "peachpuff",
            "plum", "powderblue", "bisque", "aliceblue",
            "lavender", "lightcyan", "lightyellow", "lightcoral"
        ]

        if spawn_mode == 'circle':
            # Spawn single circle
            color = Color(random.choice(pleasant_colors))

            circle = GameObject("PhysicsCircle",
                              position=mouse_world_pos,
                              basicShape=BasicShape.Circle,
                              color=color,
                              size=Vector2(30, 30))

            # Add physics components
            circle.add_component(RigidBody,
                               mass=1.0,
                               gravity_scale=1.0,
                               drag=0.15,
                               use_gravity=True,
                               lock_rotation=False)

            circle.add_component(CircleCollider,
                               radius=15,
                               material=Materials.DEFAULT,
                               collision_layer="Player")

            engine.addGameObject(circle)
            print(f"Spawned physics circle at: ({mouse_world_pos.x:.1f}, {mouse_world_pos.y:.1f})")

        elif spawn_mode == 'rectangle':
            # Spawn single rectangle
            color = Color(random.choice(pleasant_colors))

            rect = GameObject("PhysicsRect",
                             position=mouse_world_pos,
                             basicShape=BasicShape.Rectangle,
                             color=color,
                             size=Vector2(40, 40))

            # Add physics components
            rect.add_component(RigidBody,
                              mass=1.0,
                              gravity_scale=1.0,
                              drag=0.15,
                              use_gravity=True,
                              lock_rotation=False)

            rect.add_component(BoxCollider,
                              width=40,
                              height=40,
                              material=Materials.DEFAULT,
                              collision_layer="Player")

            engine.addGameObject(rect)
            print(f"Spawned physics rectangle at: ({mouse_world_pos.x:.1f}, {mouse_world_pos.y:.1f})")

        elif spawn_mode == 'formation':
            # Spawn formation around mouse
            radius = 80

            for i in range(6):
                angle = (i * 60) * math.pi / 180
                x = mouse_world_pos.x + radius * math.cos(angle)
                y = mouse_world_pos.y + radius * math.sin(angle)

                color = Color(pleasant_colors[i % len(pleasant_colors)])

                obj = GameObject(f"PhysicsFormation_{i}",
                                position=Vector2(x, y),
                                basicShape=BasicShape.Circle if i % 2 == 0 else BasicShape.Rectangle,
                                color=color,
                                size=Vector2(20, 20))

                # Add physics components
                obj.add_component(RigidBody,
                                 mass=1.0,
                                 gravity_scale=1.0,
                                 drag=0.15,
                                 use_gravity=True,
                                 lock_rotation=False)

                if i % 2 == 0:  # Circle
                    obj.add_component(CircleCollider,
                                     radius=10,
                                     material=Materials.DEFAULT,
                                     collision_layer="Player")
                else:  # Rectangle
                    obj.add_component(BoxCollider,
                                     width=20,
                                     height=20,
                                     material=Materials.DEFAULT,
                                     collision_layer="Player")

                engine.addGameObject(obj)

            print(f"Spawned physics formation around: ({mouse_world_pos.x:.1f}, {mouse_world_pos.y:.1f})")

    # Add mouse-based spawning using proper mouse input system
    def handle_mouse_spawn(engine):
        if engine.globals.get("paused"):
            return
        # Check for left mouse button just pressed
        if engine.input.get_event_state('mouse_button_down', 0):
            spawn_at_mouse(engine)

    engine.add_runnable(handle_mouse_spawn, 'update', Priority.NORMAL)

    # 5. Spawn mode switching
    def switch_spawn_mode(engine):
        if engine.globals.get("paused"):
            return
        current_mode = engine.globals.get('spawn_mode', 'circle', 'spawning')

        # Cycle through modes: circle -> rectangle -> formation -> circle
        if current_mode == 'circle':
            new_mode = 'rectangle'
        elif current_mode == 'rectangle':
            new_mode = 'formation'
        else:  # formation
            new_mode = 'circle'

        engine.globals.set('spawn_mode', new_mode, 'spawning')
        print(f"Spawn mode switched to: {new_mode}")

    engine.add_runnable(switch_spawn_mode, 'key_press', Priority.NORMAL, key=Input.Keybind.T.value)

    # 7. Create floor (like in test.py)
    def create_floor(engine):
        floor = GameObject(
            name="floor",
            basicShape=BasicShape.Rectangle,
            position=Vector2(0, -100),
            size=Vector2(800, 20),
            color=Color("saddlebrown")  # Using pygame color string
        )

        # Add physics components
        floor.add_component(RigidBody,
                           is_kinematic=True,
                           use_gravity=False)

        floor.add_component(BoxCollider,
                           width=800,
                           height=20,
                           material=Materials.DEFAULT,
                           collision_layer="Environment")

        engine.addGameObject(floor)
        print("Created floor!")

    engine.add_runnable(create_floor, 'start', Priority.CRITICAL, max_runs=1)

    # ===== PLATFORM CONTROLS =====

    # 8. Platform rotation with arrow keys
    def platform_rotation(engine):
        if engine.globals.get("paused"):
            return
        input = engine.input
        rotation_speed = 90  # degrees per second

        # Find the platform (floor)
        platform = None
        for obj in engine.getGameObjects():
            if obj.name == "floor":
                platform = obj
                break

        if platform:
            # Get the rigidbody component
            rigidbody = platform.get_component(RigidBody)
            if rigidbody and rigidbody.body:
                # Calculate angular velocity in radians per second
                angular_velocity = 0
                if input.get(Input.Keybind.K_LEFT):
                    angular_velocity = rotation_speed * 3.14159 / 180  # Convert to radians
                elif input.get(Input.Keybind.K_RIGHT):
                    angular_velocity = -rotation_speed * 3.14159 / 180  # Convert to radians

                # Set the angular velocity for smooth rotation
                rigidbody.body.angular_velocity = angular_velocity

                # Update the visual rotation to match physics
                platform.rotation = -rigidbody.body.angle * 180 / 3.14159

    engine.add_runnable(platform_rotation, 'update', Priority.HIGH)

    # 9. Reset platform rotation
    def reset_platform(engine):
        if engine.globals.get("paused"):
            return
        # Find the platform (floor)
        platform = None
        for obj in engine.getGameObjects():
            if obj.name == "floor":
                platform = obj
                break

        if platform:
            # Reset both visual and physics rotation
            platform.rotation = 0  # Reset rotation to 0 degrees

            # Update physics body rotation if it exists
            rigidbody = platform.get_component(RigidBody)
            if rigidbody and rigidbody.body:
                rigidbody.body.angle = 0  # Reset to 0 radians
                rigidbody.body.angular_velocity = 0  # Stop any rotation

            print("Platform rotation reset!")

    engine.add_runnable(reset_platform, 'key_press', Priority.NORMAL, key=Input.Keybind.P.value)

    # ===== EXPLOSION EFFECT =====

    # 10. Explosion effect at mouse position
    def explosion_effect(engine):
        if engine.globals.get("paused"):
            return

        # Get mouse position in world coordinates
        mouse_pos = engine.input.mouse.get_pos()
        mouse_world_pos = engine.camera.screen_to_world(mouse_pos)
        explosion_radius = 150
        explosion_force = 800

        # Find all physics objects near the mouse
        for obj in engine.getGameObjects():
            if obj.name == "floor":  # Skip the platform
                continue

            rigidbody = obj.get_component(RigidBody)
            if not rigidbody or not rigidbody.body:
                continue

            # Calculate distance from mouse to object
            distance = (obj.position - mouse_world_pos).length()

            if distance <= explosion_radius:
                # Calculate direction from mouse to object
                direction = (obj.position - mouse_world_pos).normalize()

                # Apply force inversely proportional to distance
                force_multiplier = 1.0 - (distance / explosion_radius)
                force = direction * explosion_force * force_multiplier

                # Apply impulse to the physics body
                rigidbody.body.apply_impulse_at_local_point((force.x, force.y), (0, 0))

        print(f"Explosion at mouse position: ({mouse_world_pos.x:.1f}, {mouse_world_pos.y:.1f})")

    # Add explosion effect using proper mouse input system
    def handle_mouse_explosion(engine):
        if engine.globals.get("paused"):
            return
        # Check for right mouse button just pressed
        if engine.input.get_event_state('mouse_button_down', 2):  # Right mouse button
            explosion_effect(engine)

    engine.add_runnable(handle_mouse_explosion, 'update', Priority.NORMAL)

    # ===== GRAVITY SYSTEM =====

    # 11. Gravity toggle system
    def toggle_gravity(engine):
        if engine.globals.get("paused"):
            return
        # Get current gravity state
        gravity_enabled = engine.globals.get('gravity_enabled', True, 'physics')

        # Toggle gravity
        gravity_enabled = not gravity_enabled
        engine.globals.set('gravity_enabled', gravity_enabled, 'physics')

        # Update global gravity instead of changing body types
        if gravity_enabled:
            engine.physics_system.set_gravity(0, -500)  # Normal gravity
        else:
            engine.physics_system.set_gravity(0, 0)  # No gravity

        # Update all physics objects' gravity scale
        for obj in engine.getGameObjects():
            if obj.name != "floor":  # Don't change floor
                rigidbody = obj.get_component(RigidBody)
                if rigidbody:
                    rigidbody.use_gravity = gravity_enabled
                    # Set gravity scale to 0 when disabled, 1 when enabled
                    rigidbody.gravity_scale = 1.0 if gravity_enabled else 0.0
                    if rigidbody.body:
                        rigidbody.body.gravity_scale = rigidbody.gravity_scale

        status = "enabled" if gravity_enabled else "disabled"
        print(f"Gravity {status}!")

    engine.add_runnable(toggle_gravity, 'key_press', Priority.NORMAL, key=Input.Keybind.G.value)

    # 12. Auto-delete objects that fall too far
    def auto_delete_fallen_objects(engine):
        if engine.globals.get("paused"):
            return
        fall_threshold = -500  # Objects below this Y position will be deleted

        objects_to_remove = []
        for obj in engine.getGameObjects():
            if obj.name != "floor" and obj.position.y < fall_threshold:
                objects_to_remove.append(obj)

        for obj in objects_to_remove:
            engine.removeGameObject(obj)
            print(f"Auto-deleted {obj.name} that fell too far (Y: {obj.position.y:.1f})")

    engine.add_runnable(auto_delete_fallen_objects, 'update', Priority.HIGH)

    # ===== VISUAL EFFECTS =====

    # 13. performance monitoring
    def render_performance_stats(engine):
        font = pg.font.Font(None, 24)
        fps = engine.clock.get_fps()
        object_count = len(engine.getGameObjects())

        stats_lines = [
            f"FPS: {fps:.1f}",
            f"Objects: {object_count}",
            f"Camera: ({engine.camera.position.x:.0f}, {engine.camera.position.y:.0f})",
            f"Zoom: {engine.camera.zoom:.2f}"
        ]

        for i, line in enumerate(stats_lines):
            text = font.render(line, True, pg.Color(255, 255, 255))
            engine.screen.blit(text, (10, 10 + i * 25))

    engine.add_runnable(render_performance_stats, 'render', Priority.CRITICAL)

    # 14. Enhanced info display
    def render_detailed_info(engine):
        font = pg.font.Font(None, 20)
        mouse_screen = engine.input.mouse.get_pos()
        mouse_world = engine.camera.screen_to_world(mouse_screen)
        gravity_enabled = engine.globals.get('gravity_enabled', True, 'physics')
        spawn_mode = engine.globals.get('spawn_mode', 'circle', 'spawning')

        info_lines = [
            f"Mouse Screen: ({mouse_screen[0]:.0f}, {mouse_screen[1]:.0f})",
            f"Mouse World: ({mouse_world[0]:.1f}, {mouse_world[1]:.1f})",
            f"Gravity: {'ON' if gravity_enabled else 'OFF'}",
            f"Spawn Mode: {spawn_mode.upper()}",
            f"Paused: {'YES' if engine.globals.get("paused") else 'NO'}"
        ]

        for i, line in enumerate(info_lines):
            text = font.render(line, True, pg.Color(255, 255, 0))
            engine.screen.blit(text, (10, 120 + i * 20))

    engine.add_runnable(render_detailed_info, 'render', Priority.CRITICAL)

    # 15. Instructions display
    def render_instructions(engine):
        font = pg.font.Font(None, 20)
        instructions = [
            "Camera Controls:",
            "  WASD - Move camera",
            "  Mouse Wheel - Zoom",
            "  R - Reset camera",
            "",
            "Platform Controls:",
            "  LEFT/RIGHT Arrow - Rotate platform",
            "  P - Reset platform rotation",
            "",
            "Physics Spawning:",
            "  LEFT CLICK - Spawn object at mouse",
            "  RIGHT CLICK - Explosion effect at mouse",
            "  T - Switch spawn mode (circle/rectangle/formation)",
            "  G - Toggle gravity",
            "  Q - Remove all objects",
            "  SPACE - Pause physics",
            "  ESC - Quit",
            "",
            "Auto-cleanup: Objects below Y=-500 are deleted"
        ]

        for i, instruction in enumerate(instructions):
            color = pg.Color(200, 200, 200) if instruction else pg.Color(100, 100, 100)
            text = font.render(instruction, True, color)
            engine.screen.blit(text, (10, 221 + i * 20))

    engine.add_runnable(render_instructions, 'render', Priority.LOW)

    # 16. Remove all objects
    def remove_all_objects(engine):
        if engine.globals.get("paused"):
            return
        objects = engine.getGameObjects()
        for obj in objects[:]:
            if obj.name != "floor":  # Don't remove the floor
                engine.removeGameObject(obj)
        print("Removed all objects!")

    engine.add_runnable(remove_all_objects, 'key_press', Priority.NORMAL, key=Input.Keybind.Q.value)

    # 17. Pastel color assignment for objects
    def assign_pastel_colors(engine):
        if engine.globals.get("paused"):
            return
        import random

        # Pastel color palette
        pastel_colors = [
            pg.Color(255, 182, 193),  # Light pink
            pg.Color(173, 216, 230),  # Light blue
            pg.Color(144, 238, 144),  # Light green
            pg.Color(255, 218, 185),  # Peach
            pg.Color(221, 160, 221),  # Plum
            pg.Color(176, 224, 230),  # Powder blue
            pg.Color(255, 255, 224),  # Light yellow
            pg.Color(240, 248, 255),  # Alice blue
            pg.Color(255, 228, 196),  # Bisque
            pg.Color(230, 230, 250),  # Lavender
        ]

        for obj in engine.getGameObjects():
            if obj.name == "floor":  # Don't change floor color
                continue

            # Assign a random pastel color based on object id for consistency
            color_index = id(obj) % len(pastel_colors)
            obj.color = pastel_colors[color_index]

    engine.add_runnable(assign_pastel_colors, 'update', Priority.NORMAL)

    # 18. Object rotation
    def rotate_objects(engine):
        if engine.globals.get("paused"):
            return
        for obj in engine.getGameObjects():
            if obj.name == "floor":  # Don't rotate the floor
                continue

            obj.rotation += 0.5  # Slower rotation

    engine.add_runnable(rotate_objects, 'update', Priority.NORMAL)

    # 19. ESC key handler
    def handle_escape(engine):
        print("ESC pressed - quitting...")
        engine.stop()

    engine.add_runnable(handle_escape, 'key_press', Priority.CRITICAL, key=Input.Keybind.K_ESCAPE.value)

    def toggle_physics(engine):
        if engine.globals.get("paused"):
            engine.unpause_physics()
            engine.globals.set("paused", False)
            print("Physics unpaused!")
        else:
            engine.pause_physics()
            engine.globals.set("paused", True)
            print("Physics paused!")

    engine.add_runnable(toggle_physics, 'key_press', Priority.HIGH, key=Input.Keybind.K_SPACE.value)



    # Start the engine
    print("Starting Physics Camera Demo...")
    print("Camera Controls:")
    print("  WASD - Move camera")
    print("  Mouse Wheel - Zoom")
    print("  R - Reset camera")
    print("")
    print("Platform Controls:")
    print("  LEFT/RIGHT Arrow - Rotate platform")
    print("  P - Reset platform rotation")
    print("")
    print("Physics Spawning:")
    print("  LEFT CLICK - Spawn object at mouse")
    print("  RIGHT CLICK - Explosion effect at mouse")
    print("  T - Switch spawn mode (circle/rectangle/formation)")
    print("  G - Toggle gravity")
    print("  Q - Remove all objects")
    print("  SPACE - Pause physics")
    print("  ESC - Quit")
    print("")
    print("Auto-cleanup: Objects below Y=-500 are automatically deleted")

    engine.start()

if __name__ == "__main__":
    main()
