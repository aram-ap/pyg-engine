#!/usr/bin/env python3
"""
Interactive Collision Detection Demo

Controls:
- Arrow Keys / WASD: Move the player circle
- ESC: Quit

The player circle changes color when colliding with obstacles.
Demonstrates:
- Built-in collision detection system (AABB Tree + SAT)
- ColliderComponent with collision callbacks
- Layer-based collision filtering
- Visual feedback on collision
- User input for movement
"""

import pyg_engine as pyg


class Player:
    """Player-controlled circle with collision detection"""

    def __init__(self, engine, x, y, radius):
        self.obj = pyg.GameObject("Player")
        self.obj.position = pyg.Vec2(x, y)
        self.obj.scale = pyg.Vec2(radius * 2, radius * 2)
        self.radius = radius
        self.speed = 5.0
        self.is_colliding = False

        # Create mesh
        self.obj.set_mesh_geometry_circle(radius)
        self.obj.set_mesh_fill_color(pyg.Color.rgb(100, 200, 255))  # Blue
        self.obj.set_mesh_draw_order(2.0)

        # Add collider component with callbacks!
        collider = pyg.Collider("PlayerCollider")
        collider.set_shape(pyg.ColliderShape.circle(0.5))
        collider.set_layer(pyg.PhysicsLayers.PLAYER)
        collider.set_collision_mask(
            pyg.PhysicsLayers.create_mask([pyg.PhysicsLayers.ENVIRONMENT])
        )
        collider.set_trigger(True)  # No physics response, just detection

        # Set collision callbacks
        collider.set_on_collision_enter(self.on_collision_enter)
        collider.set_on_collision_exit(self.on_collision_exit)

        self.obj.add_component(collider)

        print(f"  ✓ Created player at ({x}, {y}) with collision callbacks")

    def on_collision_enter(self, other_id, normal_x, normal_y, penetration):
        """Called when collision starts"""
        self.is_colliding = True
        self.obj.set_mesh_fill_color(pyg.Color.rgb(255, 100, 100))  # Red
        print(f"  Collision! Hit object {other_id}, penetration: {penetration:.2f}")

    def on_collision_exit(self, other_id):
        """Called when collision ends"""
        self.is_colliding = False
        self.obj.set_mesh_fill_color(pyg.Color.rgb(100, 200, 255))  # Blue
        print(f"  Collision ended with object {other_id}")

    def update(self, engine, dt):
        """Update player position based on input"""
        # Get input using axis (Horizontal and Vertical are pre-configured)
        dx = engine.input.axis("Horizontal")
        dy = engine.input.axis("Vertical")

        # Move player
        if dx != 0.0 or dy != 0.0:
            new_pos = self.obj.position + pyg.Vec2(dx * self.speed * dt, dy * self.speed * dt)
            self.obj.position = new_pos


class CircleObstacle:
    """Static circular obstacle"""

    def __init__(self, engine, x, y, radius, color):
        self.obj = pyg.GameObject(f"Circle_{id(self)}")
        self.obj.position = pyg.Vec2(x, y)
        self.obj.scale = pyg.Vec2(radius * 2, radius * 2)

        # Create mesh
        self.obj.set_mesh_geometry_circle(radius)
        self.obj.set_mesh_fill_color(color)
        self.obj.set_mesh_draw_order(1.0)

        # Add collider component
        collider = pyg.Collider("ObstacleCollider")
        collider.set_shape(pyg.ColliderShape.circle(radius))
        collider.set_layer(pyg.PhysicsLayers.ENVIRONMENT)
        collider.set_trigger(True)
        self.obj.add_component(collider)


class BoxObstacle:
    """Static box obstacle"""

    def __init__(self, engine, x, y, width, height, color):
        self.obj = pyg.GameObject(f"Box_{id(self)}")
        self.obj.position = pyg.Vec2(x, y)
        self.obj.scale = pyg.Vec2(width, height)

        # Create mesh
        self.obj.set_mesh_geometry_rectangle(1.0, 1.0)
        self.obj.set_mesh_fill_color(color)
        self.obj.set_mesh_draw_order(1.0)

        # Add collider component
        collider = pyg.Collider("BoxCollider")
        collider.set_shape(pyg.ColliderShape.box_shape(0.5, 0.5))
        collider.set_layer(pyg.PhysicsLayers.ENVIRONMENT)
        collider.set_trigger(True)
        self.obj.add_component(collider)


def main():
    print("\n" + "="*70)
    print(" " * 15 + "Collision Detection Demo")
    print("="*70 + "\n")

    # Create engine
    engine = pyg.Engine(log_level="INFO")

    engine.start_manual(
        title="Collision Detection Demo - Move with Arrow Keys/WASD",
        width=1280,
        height=720,
        background_color=pyg.Color.rgb(20, 20, 30),
        vsync=False,
        show_fps_in_title=True,
    )

    # Set camera
    engine.set_camera_viewport_size(16.0, 9.0)

    print("Creating scene...")

    # Create player
    player = Player(engine, 0.0, 0.0, 0.4)
    if engine.add_game_object(player.obj) is None:
        raise RuntimeError("Failed to add player")

    # Create obstacles
    obstacles = []

    # Circle obstacles
    print("  Creating obstacles...")
    obstacles.append(CircleObstacle(engine, -3.0, 2.0, 0.6, pyg.Color.rgb(200, 150, 100)))
    obstacles.append(CircleObstacle(engine, 3.0, -2.0, 0.5, pyg.Color.rgb(150, 200, 100)))
    obstacles.append(CircleObstacle(engine, -4.0, -3.0, 0.7, pyg.Color.rgb(200, 100, 150)))
    obstacles.append(CircleObstacle(engine, 4.5, 2.5, 0.8, pyg.Color.rgb(100, 150, 200)))

    # Box obstacles
    obstacles.append(BoxObstacle(engine, 0.0, 3.0, 3.0, 0.6, pyg.Color.rgb(180, 180, 100)))
    obstacles.append(BoxObstacle(engine, 0.0, -3.5, 4.0, 0.8, pyg.Color.rgb(100, 180, 180)))
    obstacles.append(BoxObstacle(engine, -5.5, 0.0, 1.0, 4.0, pyg.Color.rgb(180, 100, 180)))
    obstacles.append(BoxObstacle(engine, 5.5, 0.0, 1.0, 4.0, pyg.Color.rgb(150, 150, 180)))

    # Add obstacles to engine
    for obstacle in obstacles:
        if engine.add_game_object(obstacle.obj) is None:
            raise RuntimeError(f"Failed to add obstacle")

    print(f"  ✓ Created {len(obstacles)} obstacles with colliders")
    print("\n" + "="*70)
    print("Scene ready!")
    print("="*70 + "\n")

    print("Controls:")
    print("  Arrow Keys or WASD: Move the blue circle")
    print("  ESC: Quit")
    print("\nThe circle turns RED when colliding with obstacles!\n")

    # Show collision system info
    print("Collision Detection System:")
    print("  ✓ Built-in collision detection enabled")
    print("  ✓ AABB Tree broad-phase")
    print("  ✓ SAT narrow-phase")
    print("  ✓ Layer-based filtering (PLAYER vs ENVIRONMENT)")
    print("  ✓ Collision callbacks working!")
    print()

    # Game loop
    dt = 1.0 / 60.0  # Fixed timestep

    while engine.poll_events():
        if engine.input.action_pressed("escape"):
            break

        # Update player
        player.update(engine, engine.delta_time)

        # Draw UI
        engine.clear_draw_commands()

        # Instructions
        engine.draw_text(
            "Move with Arrow Keys or WASD",
            24.0, 24.0,
            pyg.Color.WHITE,
            font_size=20.0,
            draw_order=10.0,
        )

        # Collision status
        status_text = "COLLIDING!" if player.is_colliding else "Clear"
        status_color = pyg.Color.rgb(255, 100, 100) if player.is_colliding else pyg.Color.rgb(100, 255, 100)

        engine.draw_text(
            f"Status: {status_text}",
            24.0, 56.0,
            status_color,
            font_size=18.0,
            draw_order=10.0,
        )

        # Position info
        engine.draw_text(
            f"Position: ({player.obj.position.x:.2f}, {player.obj.position.y:.2f})",
            24.0, 88.0,
            pyg.Color.rgb(200, 200, 200),
            font_size=16.0,
            draw_order=10.0,
        )

        # Collision system info
        engine.draw_text(
            "Built-in Collision System: Active with Python Callbacks!",
            24.0, 120.0,
            pyg.Color.rgb(100, 255, 100),
            font_size=14.0,
            draw_order=10.0,
        )

        # ESC hint
        engine.draw_text(
            "ESC to quit",
            24.0, 680.0,
            pyg.Color.rgb(120, 120, 120),
            font_size=14.0,
            draw_order=10.0,
        )

        # Update and render
        engine.update()
        engine.render()

    engine.log_info("Collision demo finished.")
    print("\nDemo complete!\n")


if __name__ == "__main__":
    main()
