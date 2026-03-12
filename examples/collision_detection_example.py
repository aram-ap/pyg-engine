"""
Collision Detection System Example

Demonstrates the collision detection system:
- Creating colliders with different shapes
- Using collision layers for filtering
- Creating trigger zones
- Collision event callbacks

Note: This is a collision detection system only.
Physics simulation (forces, gravity, etc.) can be added in the future.
"""

import pyg_engine as pyg


def demonstrate_collider_shapes():
    """Demonstrate different collider shapes"""
    print("\n" + "="*60)
    print("COLLIDER SHAPES")
    print("="*60 + "\n")

    # Circle collider
    circle = pyg.ColliderShape.circle(0.5)
    print("✓ Circle collider created (radius: 0.5)")

    # Box collider
    box = pyg.ColliderShape.box_shape(1.0, 0.5)
    print("✓ Box collider created (half_width: 1.0, half_height: 0.5)")

    # Oriented box (OBB)
    obb = pyg.ColliderShape.obb(1.0, 0.5, 0.785)  # 45 degrees
    print("✓ OBB collider created (rotated 45°)")

    # Polygon collider
    triangle_verts = [
        (-0.5, -0.5),
        (0.5, -0.5),
        (0.0, 0.5)
    ]
    polygon = pyg.ColliderShape.polygon(triangle_verts)
    print("✓ Polygon collider created (triangle)")


def demonstrate_collider_component():
    """Demonstrate collider component setup"""
    print("\n" + "="*60)
    print("COLLIDER COMPONENT")
    print("="*60 + "\n")

    # Create a collider
    collider = pyg.Collider("PlayerCollider")
    print("✓ Collider component created")

    # Set shape
    collider.set_shape(pyg.ColliderShape.circle(0.5))
    print("  - Shape: Circle (radius 0.5)")

    # Set offset from GameObject center
    collider.set_offset(0.0, 0.1)
    print("  - Offset: (0, 0.1)")

    # Set physics layer
    collider.set_layer(pyg.PhysicsLayers.PLAYER)
    print(f"  - Layer: {pyg.PhysicsLayers.PLAYER} (PLAYER)")

    # Set collision mask (what it can collide with)
    mask = pyg.PhysicsLayers.create_mask([
        pyg.PhysicsLayers.ENEMY,
        pyg.PhysicsLayers.ENVIRONMENT,
        pyg.PhysicsLayers.PROJECTILE
    ])
    collider.set_collision_mask(mask)
    print(f"  - Collision mask: {mask:032b} (ENEMY, ENVIRONMENT, PROJECTILE)")

    # Make it a trigger (no physical response)
    collider.set_trigger(False)
    print("  - Trigger: False")

    print(f"\n✓ Final layer: {collider.layer}")
    print(f"✓ Final mask: {collider.collision_mask}")
    print(f"✓ Is trigger: {collider.is_trigger}")


def demonstrate_collision_layers():
    """Demonstrate collision layer system"""
    print("\n" + "="*60)
    print("COLLISION LAYERS")
    print("="*60 + "\n")

    # Available layers
    print("Available layers:")
    print(f"  DEFAULT: {pyg.PhysicsLayers.DEFAULT}")
    print(f"  PLAYER: {pyg.PhysicsLayers.PLAYER}")
    print(f"  ENEMY: {pyg.PhysicsLayers.ENEMY}")
    print(f"  PROJECTILE: {pyg.PhysicsLayers.PROJECTILE}")
    print(f"  ENVIRONMENT: {pyg.PhysicsLayers.ENVIRONMENT}")
    print(f"  TRIGGER: {pyg.PhysicsLayers.TRIGGER}")
    print(f"  UI: {pyg.PhysicsLayers.UI}")
    print(f"  PICKUP: {pyg.PhysicsLayers.PICKUP}")

    # Create collision masks
    print("\nCreating collision masks:")

    # Player collides with enemies, environment, and pickups
    player_mask = pyg.PhysicsLayers.create_mask([
        pyg.PhysicsLayers.ENEMY,
        pyg.PhysicsLayers.ENVIRONMENT,
        pyg.PhysicsLayers.PICKUP
    ])
    print(f"  Player mask: {player_mask:032b}")

    # Enemy collides with player, environment, and projectiles
    enemy_mask = pyg.PhysicsLayers.create_mask([
        pyg.PhysicsLayers.PLAYER,
        pyg.PhysicsLayers.ENVIRONMENT,
        pyg.PhysicsLayers.PROJECTILE
    ])
    print(f"  Enemy mask:  {enemy_mask:032b}")

    # Projectile only collides with enemies and environment
    projectile_mask = pyg.PhysicsLayers.create_mask([
        pyg.PhysicsLayers.ENEMY,
        pyg.PhysicsLayers.ENVIRONMENT
    ])
    print(f"  Projectile:  {projectile_mask:032b}")

    # Utility masks
    all_mask = pyg.PhysicsLayers.all()
    none_mask = pyg.PhysicsLayers.none()
    print(f"\n  All layers:  {all_mask:032b}")
    print(f"  No layers:   {none_mask:032b}")


def demonstrate_trigger_setup():
    """Demonstrate trigger collider setup"""
    print("\n" + "="*60)
    print("TRIGGER COLLIDERS")
    print("="*60 + "\n")

    # Create a trigger collider
    trigger = pyg.Collider("PickupTrigger")
    trigger.set_shape(pyg.ColliderShape.circle(0.5))
    trigger.set_layer(pyg.PhysicsLayers.PICKUP)
    trigger.set_collision_mask(pyg.PhysicsLayers.create_mask([
        pyg.PhysicsLayers.PLAYER
    ]))
    trigger.set_trigger(True)  # This makes it a trigger!

    print("✓ Trigger collider created")
    print("  - Shape: Circle")
    print("  - Layer: PICKUP")
    print("  - Detects: PLAYER")
    print("  - Is trigger: True")
    print("\nTriggers detect collisions and fire callbacks,")
    print("but don't prevent objects from passing through.")
    print("Perfect for: Pickups, damage zones, checkpoint triggers")


def demonstrate_collision_detection():
    """Show how collision detection works"""
    print("\n" + "="*60)
    print("COLLISION DETECTION SYSTEM")
    print("="*60 + "\n")

    print("The collision system uses a two-phase approach:\n")

    print("1. BROAD PHASE (AABB Tree)")
    print("   - Spatial partitioning with dynamic AABB tree")
    print("   - Quickly finds potentially colliding objects")
    print("   - Fattened AABBs reduce tree updates")
    print("   - Scales well to 1000+ objects\n")

    print("2. NARROW PHASE (SAT)")
    print("   - Separating Axis Theorem for precise detection")
    print("   - Returns collision manifold:")
    print("     * Penetration depth")
    print("     * Collision normal")
    print("     * Contact points")
    print("   - Supports all shape combinations\n")

    print("3. EVENT DISPATCH")
    print("   - on_collision_enter(other_id, normal, penetration)")
    print("   - on_collision_stay(other_id, normal, penetration)")
    print("   - on_collision_exit(other_id)")
    print("   - Called on all components of colliding objects\n")


def demonstrate_complete_setup():
    """Show a complete collision object setup"""
    print("\n" + "="*60)
    print("COMPLETE COLLISION OBJECT SETUP")
    print("="*60 + "\n")

    print("Creating a game object with collision detection:\n")

    # Step 1: Create collider
    collider = pyg.Collider("ObjectCollider")
    collider.set_shape(pyg.ColliderShape.circle(0.5))
    collider.set_layer(pyg.PhysicsLayers.DEFAULT)
    collider.set_collision_mask(pyg.PhysicsLayers.all())
    print("1. ✓ Collider configured (circle, radius 0.5)")

    # Step 2: Would attach to GameObject
    print("2. → Attach collider to GameObject")
    print("3. → Add GameObject to engine")
    print("4. → Collision detection runs automatically!")

    print("\nThe system will:")
    print("  • Detect when objects overlap")
    print("  • Filter collisions by layer")
    print("  • Calculate collision information")
    print("  • Fire collision callbacks on components")


def main():
    """Main entry point"""
    print("\n" + "="*80)
    print(" " * 15 + "PyG Engine - Collision Detection System")
    print("="*80)

    print("\nThis example demonstrates the collision detection API.")
    print("Physics simulation (forces, gravity, etc.) can be added in the future.\n")

    # Run demonstrations
    demonstrate_collider_shapes()
    demonstrate_collider_component()
    demonstrate_collision_layers()
    demonstrate_trigger_setup()
    demonstrate_collision_detection()
    demonstrate_complete_setup()

    print("\n" + "="*80)
    print("API demonstration complete!")
    print("="*80 + "\n")

    print("Key Features:")
    print("  ✓ Multiple collision shapes (Circle, Box, OBB, Polygon)")
    print("  ✓ Layer-based collision filtering (32 layers)")
    print("  ✓ Trigger zones for detection-only collisions")
    print("  ✓ Efficient AABB tree for broad-phase")
    print("  ✓ Precise SAT for narrow-phase")
    print("  ✓ Collision event callbacks\n")

    print("Future: Physics Simulation")
    print("  • RigidBody component for physical properties")
    print("  • Gravity and force application")
    print("  • Impulse-based collision response")
    print("  • Constraints and joints\n")


if __name__ == "__main__":
    main()
