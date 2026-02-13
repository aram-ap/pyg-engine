#!/usr/bin/env python3
"""
Interactive Collision Detection Demo

Controls:
- Arrow Keys / WASD: Move the player circle
- ESC: Quit

The player circle changes color when colliding with obstacles.
Demonstrates:
- Collision detection between moving objects
- Layer-based collision filtering
- Visual feedback on collision
- User input for movement
"""

import pyg_engine as pyg
import math


class CollisionDetector:
    """Helper to detect circle collisions using simple overlap test"""

    @staticmethod
    def circles_overlap(pos1, radius1, pos2, radius2):
        """Check if two circles overlap"""
        dx = pos2.x - pos1.x
        dy = pos2.y - pos1.y
        distance_sq = dx * dx + dy * dy
        radius_sum = radius1 + radius2
        return distance_sq < (radius_sum * radius_sum)

    @staticmethod
    def circle_box_overlap(circle_pos, circle_radius, box_pos, box_width, box_height):
        """Check if a circle overlaps with an axis-aligned box"""
        # Find the closest point on the box to the circle center
        half_width = box_width / 2
        half_height = box_height / 2

        # Clamp circle center to box bounds
        closest_x = max(box_pos.x - half_width,
                       min(circle_pos.x, box_pos.x + half_width))
        closest_y = max(box_pos.y - half_height,
                       min(circle_pos.y, box_pos.y + half_height))

        # Check if the closest point is within the circle
        dx = circle_pos.x - closest_x
        dy = circle_pos.y - closest_y
        distance_sq = dx * dx + dy * dy

        return distance_sq < (circle_radius * circle_radius)


class Player:
    """Player-controlled circle"""

    def __init__(self, x, y, radius):
        self.obj = pyg.GameObject("Player")
        self.obj.position = pyg.Vec2(x, y)
        self.obj.scale = pyg.Vec2(radius * 2, radius * 2)
        self.radius = radius
        self.speed = 1.0

        # Create mesh
        mesh = pyg.MeshComponent("PlayerMesh")
        mesh.set_geometry_circle(0.5, segments=32)
        mesh.set_fill_color(pyg.Color.rgb(100, 200, 255))  # Blue when not colliding
        mesh.draw_order = 2.0
        self.obj.set_mesh_component(mesh)

        # Collision state
        self.is_colliding = False

        # Create collider (API structure - not yet integrated)
        self.collider = pyg.Collider("PlayerCollider")
        self.collider.set_shape(pyg.ColliderShape.circle(0.5))
        self.collider.set_layer(pyg.PhysicsLayers.PLAYER)
        self.collider.set_collision_mask(pyg.PhysicsLayers.all())

        print(f"  Created player at ({x}, {y})")

    def update(self, engine, dt, obstacles):
        """Update player position based on input"""
        # Get input using axis (Horizontal and Vertical are pre-configured)
        dx = engine.input.axis("Horizontal")
        dy = engine.input.axis("Vertical") 

        # Move player
        if dx != 0.0 or dy != 0.0:
            self.obj.position = self.obj.position + pyg.Vec2(dx * self.speed * dt, dy * self.speed * dt)

        # Check collisions with obstacles
        self.is_colliding = False
        for obstacle in obstacles:
            if obstacle.check_collision(self.obj.position, self.radius):
                self.is_colliding = True
                break

        # Update color based on collision state
        mesh = self.obj.mesh_component()
        if mesh:
            if self.is_colliding:
                mesh.set_fill_color(pyg.Color.rgb(255, 100, 100))  # Red when colliding
            else:
                mesh.set_fill_color(pyg.Color.rgb(100, 200, 255))  # Blue when not colliding


class CircleObstacle:
    """Static circular obstacle"""

    def __init__(self, x, y, radius, color):
        self.obj = pyg.GameObject(f"Circle_{id(self)}")
        self.obj.position = pyg.Vec2(x, y)
        self.obj.scale = pyg.Vec2(radius * 2, radius * 2)
        self.radius = radius

        # Create mesh
        mesh = pyg.MeshComponent("ObstacleMesh")
        mesh.set_geometry_circle(0.5, segments=32)
        mesh.set_fill_color(color)
        mesh.draw_order = 1.0
        self.obj.set_mesh_component(mesh)

        # Create collider
        self.collider = pyg.Collider("ObstacleCollider")
        self.collider.set_shape(pyg.ColliderShape.circle(0.5))
        self.collider.set_layer(pyg.PhysicsLayers.ENVIRONMENT)

    def check_collision(self, player_pos, player_radius):
        """Check if player collides with this obstacle"""
        return CollisionDetector.circles_overlap(
            player_pos, player_radius,
            self.obj.position, self.radius
        )


class BoxObstacle:
    """Static box obstacle"""

    def __init__(self, x, y, width, height, color):
        self.obj = pyg.GameObject(f"Box_{id(self)}")
        self.obj.position = pyg.Vec2(x, y)
        self.obj.scale = pyg.Vec2(width, height)
        self.width = width
        self.height = height

        # Create mesh
        mesh = pyg.MeshComponent("BoxMesh")
        mesh.set_geometry_rectangle(1.0, 1.0)
        mesh.set_fill_color(color)
        mesh.draw_order = 1.0
        self.obj.set_mesh_component(mesh)

        # Create collider
        self.collider = pyg.Collider("BoxCollider")
        self.collider.set_shape(pyg.ColliderShape.box_shape(0.5, 0.5))
        self.collider.set_layer(pyg.PhysicsLayers.ENVIRONMENT)

    def check_collision(self, player_pos, player_radius):
        """Check if player collides with this obstacle"""
        return CollisionDetector.circle_box_overlap(
            player_pos, player_radius,
            self.obj.position, self.width, self.height
        )


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
    player = Player(0.0, 0.0, 0.4)
    if engine.add_game_object(player.obj) is None:
        raise RuntimeError("Failed to add player")

    # Create obstacles
    obstacles = []

    # Circle obstacles
    obstacles.append(CircleObstacle(-3.0, 2.0, 0.6, pyg.Color.rgb(200, 150, 100)))
    obstacles.append(CircleObstacle(3.0, -2.0, 0.5, pyg.Color.rgb(150, 200, 100)))
    obstacles.append(CircleObstacle(-4.0, -3.0, 0.7, pyg.Color.rgb(200, 100, 150)))
    obstacles.append(CircleObstacle(4.5, 2.5, 0.8, pyg.Color.rgb(100, 150, 200)))

    # Box obstacles
    obstacles.append(BoxObstacle(0.0, 3.0, 3.0, 0.6, pyg.Color.rgb(180, 180, 100)))
    obstacles.append(BoxObstacle(0.0, -3.5, 4.0, 0.8, pyg.Color.rgb(100, 180, 180)))
    obstacles.append(BoxObstacle(-5.5, 0.0, 1.0, 4.0, pyg.Color.rgb(180, 100, 180)))
    obstacles.append(BoxObstacle(5.5, 0.0, 1.0, 4.0, pyg.Color.rgb(150, 150, 180)))

    # Add obstacles to engine
    for obstacle in obstacles:
        if engine.add_game_object(obstacle.obj) is None:
            raise RuntimeError(f"Failed to add obstacle")

    print(f"  Created {len(obstacles)} obstacles")
    print("\n" + "="*70)
    print("Scene ready!")
    print("="*70 + "\n")

    print("Controls:")
    print("  Arrow Keys or WASD: Move the blue circle")
    print("  ESC: Quit")
    print("\nThe circle turns RED when colliding with obstacles!\n")

    # Show collider API structure
    print("Collision Detection System:")
    print("  • Player collider: Circle (radius 0.4)")
    print("  • Player layer: PLAYER")
    print("  • Obstacles layer: ENVIRONMENT")
    print("  • Detection: Broad-phase + SAT (simulated)")
    print()

    # Game loop
    dt = 1.0 / 60.0  # Fixed timestep

    while engine.poll_events():
        if engine.input.action_pressed("escape"):
            break

        # Update player
        player.update(engine, dt, obstacles)

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
            "Collision Detection: Shape-based overlap testing",
            24.0, 120.0,
            pyg.Color.rgb(150, 150, 150),
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
