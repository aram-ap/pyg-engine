use crate::core::physics::*;
use crate::types::vector::Vec2;
use pyo3::prelude::*;

// ========== Collision Detection Bindings ==========

/// Physics layer constants for collision filtering.
///
/// Layers allow you to organize objects into groups and control which groups can collide
/// with each other. Each object has one layer (0-31) and a collision mask that determines
/// which layers it can collide with.
///
/// # Built-in Layers
/// - `DEFAULT` - General-purpose objects (layer 0)
/// - `PLAYER` - Player-controlled objects (layer 1)
/// - `ENEMY` - Enemy characters (layer 2)
/// - `PROJECTILE` - Bullets, missiles, etc. (layer 3)
/// - `ENVIRONMENT` - Walls, floors, static objects (layer 4)
/// - `TRIGGER` - Invisible trigger zones (layer 5)
/// - `UI` - User interface elements (layer 6)
/// - `PICKUP` - Collectible items (layer 7)
///
/// # How Layer Filtering Works
/// 1. Each collider has a **layer** (which group it belongs to)
/// 2. Each collider has a **collision mask** (which groups it can detect)
/// 3. Collision occurs if: `(layer_A & mask_B) != 0 AND (layer_B & mask_A) != 0`
///
/// # Example: Basic Layer Assignment
/// ```python
/// import pyg_engine as pyg
///
/// # Create player collider
/// player_collider = pyg.Collider("Player")
/// player_collider.set_layer(pyg.PhysicsLayers.PLAYER)
/// player_collider.set_collision_mask(
///     pyg.PhysicsLayers.create_mask([
///         pyg.PhysicsLayers.ENEMY,
///         pyg.PhysicsLayers.ENVIRONMENT
///     ])
/// )  # Player only collides with enemies and walls
///
/// # Create enemy collider
/// enemy_collider = pyg.Collider("Enemy")
/// enemy_collider.set_layer(pyg.PhysicsLayers.ENEMY)
/// enemy_collider.set_collision_mask(
///     pyg.PhysicsLayers.create_mask([
///         pyg.PhysicsLayers.PLAYER,
///         pyg.PhysicsLayers.PROJECTILE
///     ])
/// )  # Enemy collides with player and projectiles
/// ```
///
/// # Example: Selective Collision
/// ```python
/// import pyg_engine as pyg
///
/// # Projectile that hits enemies but passes through pickups
/// projectile = pyg.Collider("Bullet")
/// projectile.set_layer(pyg.PhysicsLayers.PROJECTILE)
/// projectile.set_collision_mask(
///     pyg.PhysicsLayers.create_mask([
///         pyg.PhysicsLayers.ENEMY,
///         pyg.PhysicsLayers.ENVIRONMENT
///     ])
/// )  # Only hits enemies and walls, ignores pickups
///
/// # Pickup that only detects player
/// pickup = pyg.Collider("Coin")
/// pickup.set_layer(pyg.PhysicsLayers.PICKUP)
/// pickup.set_collision_mask(
///     pyg.PhysicsLayers.create_mask([pyg.PhysicsLayers.PLAYER])
/// )  # Only player can collect it
/// ```
///
/// # Example: Collide With Everything
/// ```python
/// import pyg_engine as pyg
///
/// # Object that collides with all layers
/// wall = pyg.Collider("Wall")
/// wall.set_layer(pyg.PhysicsLayers.ENVIRONMENT)
/// wall.set_collision_mask(pyg.PhysicsLayers.all())  # Collides with everything
/// ```
///
/// # See Also
/// - `create_mask()` - Create collision mask from layer list
/// - `all()` - Mask that collides with all layers
/// - `none()` - Mask that collides with no layers
/// - `Collider.set_layer()` - Set object's layer
/// - `Collider.set_collision_mask()` - Set which layers to collide with
#[pyclass(name = "PhysicsLayers")]
#[derive(Clone)]
pub struct PyPhysicsLayers;

#[pymethods]
impl PyPhysicsLayers {
    #[classattr]
    const DEFAULT: u32 = PhysicsLayers::DEFAULT;

    #[classattr]
    const PLAYER: u32 = PhysicsLayers::PLAYER;

    #[classattr]
    const ENEMY: u32 = PhysicsLayers::ENEMY;

    #[classattr]
    const PROJECTILE: u32 = PhysicsLayers::PROJECTILE;

    #[classattr]
    const ENVIRONMENT: u32 = PhysicsLayers::ENVIRONMENT;

    #[classattr]
    const TRIGGER: u32 = PhysicsLayers::TRIGGER;

    #[classattr]
    const UI: u32 = PhysicsLayers::UI;

    #[classattr]
    const PICKUP: u32 = PhysicsLayers::PICKUP;

    /// Create a collision mask from a list of layers.
    ///
    /// Converts a list of layer IDs into a bitmask that can be used for collision filtering.
    /// The resulting mask allows collision with all specified layers.
    ///
    /// # Arguments
    /// * `layers` - List of layer constants to include in the mask
    ///
    /// # Returns
    /// A 32-bit bitmask representing the specified layers
    ///
    /// # Example: Basic Mask Creation
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Create mask for player + enemy collision
    /// mask = pyg.PhysicsLayers.create_mask([
    ///     pyg.PhysicsLayers.PLAYER,
    ///     pyg.PhysicsLayers.ENEMY
    /// ])
    ///
    /// collider = pyg.Collider("Object")
    /// collider.set_collision_mask(mask)
    /// ```
    ///
    /// # Example: Complex Filtering
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Player collides with enemies, walls, and pickups
    /// player_mask = pyg.PhysicsLayers.create_mask([
    ///     pyg.PhysicsLayers.ENEMY,
    ///     pyg.PhysicsLayers.ENVIRONMENT,
    ///     pyg.PhysicsLayers.PICKUP
    /// ])
    ///
    /// # Bullet collides only with enemies
    /// bullet_mask = pyg.PhysicsLayers.create_mask([
    ///     pyg.PhysicsLayers.ENEMY
    /// ])
    ///
    /// player_collider = pyg.Collider("Player")
    /// player_collider.set_collision_mask(player_mask)
    ///
    /// bullet_collider = pyg.Collider("Bullet")
    /// bullet_collider.set_collision_mask(bullet_mask)
    /// ```
    ///
    /// # See Also
    /// - `all()` - Mask for all layers
    /// - `none()` - Empty mask (no collisions)
    /// - `Collider.set_collision_mask()` - Apply mask to collider
    #[staticmethod]
    fn create_mask(layers: Vec<u32>) -> u32 {
        layers::create_mask(&layers)
    }

    /// Create a mask that collides with all layers.
    ///
    /// Returns a bitmask with all bits set, allowing collision with every layer.
    /// Useful for objects that should interact with everything (like walls or boundaries).
    ///
    /// # Returns
    /// A mask representing all 32 layers (0xFFFFFFFF)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Wall that blocks everything
    /// wall = pyg.Collider("Wall")
    /// wall.set_layer(pyg.PhysicsLayers.ENVIRONMENT)
    /// wall.set_collision_mask(pyg.PhysicsLayers.all())
    /// ```
    ///
    /// # See Also
    /// - `create_mask()` - Create selective mask
    /// - `none()` - Empty mask
    #[staticmethod]
    fn all() -> u32 {
        layers::all()
    }

    /// Create a mask that collides with no layers.
    ///
    /// Returns an empty bitmask (all bits cleared). Objects with this mask won't
    /// trigger collision events. Useful for disabled colliders or special cases.
    ///
    /// # Returns
    /// An empty mask (0x00000000)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Temporarily disable collision
    /// collider = pyg.Collider("Object")
    /// collider.set_collision_mask(pyg.PhysicsLayers.none())
    ///
    /// # Re-enable later
    /// collider.set_collision_mask(pyg.PhysicsLayers.all())
    /// ```
    ///
    /// # See Also
    /// - `all()` - Mask for all layers
    /// - `create_mask()` - Create custom mask
    #[staticmethod]
    fn none() -> u32 {
        layers::none()
    }
}

/// Collider shape builder for creating collision shapes.
///
/// Provides static methods to create different types of collision shapes:
/// circles, boxes, oriented boxes, and custom polygons. Shapes define the
/// physical boundaries used for collision detection.
///
/// # Available Shapes
/// - **Circle**: Defined by radius, efficient for round objects
/// - **Box (AABB)**: Axis-aligned rectangle, fast collision checks
/// - **OBB**: Oriented (rotated) rectangle, follows object rotation
/// - **Polygon**: Custom convex shape with arbitrary vertices
///
/// # Coordinate System
/// All shapes are defined in **local space** relative to the GameObject's center:
/// - (0, 0) is the object's center
/// - Positive X is right, positive Y is up
/// - Shapes scale with GameObject.scale
/// - OBB and Polygon shapes rotate with GameObject.rotation
///
/// # Example: Circle Shape
/// ```python
/// import pyg_engine as pyg
///
/// # Create a circle collider with radius 0.5
/// shape = pyg.ColliderShape.circle(0.5)
///
/// obj = pyg.GameObject("Ball")
/// obj.scale = pyg.Vec2(2.0, 2.0)  # Actual radius becomes 1.0
/// obj.set_mesh_geometry_circle(0.5, 32)
///
/// collider = pyg.Collider("BallCollider")
/// collider.set_shape(shape)
/// obj.add_component(collider)
/// ```
///
/// # Example: Box Shape
/// ```python
/// import pyg_engine as pyg
///
/// # Create a box with half-extents (half width, half height)
/// shape = pyg.ColliderShape.box_shape(0.5, 0.3)  # 1.0 wide, 0.6 tall
///
/// obj = pyg.GameObject("Platform")
/// obj.set_mesh_geometry_rectangle(1.0, 0.6)
///
/// collider = pyg.Collider("PlatformCollider")
/// collider.set_shape(shape)
/// obj.add_component(collider)
/// ```
///
/// # Example: Polygon Shape
/// ```python
/// import pyg_engine as pyg
///
/// # Create a triangle
/// shape = pyg.ColliderShape.polygon([
///     (0.0, 0.5),   # Top vertex
///     (-0.5, -0.5), # Bottom left
///     (0.5, -0.5)   # Bottom right
/// ])
///
/// obj = pyg.GameObject("Triangle")
/// collider = pyg.Collider("TriangleCollider")
/// collider.set_shape(shape)
/// obj.add_component(collider)
/// ```
///
/// # See Also
/// - `Collider.set_shape()` - Apply shape to collider
/// - `Collider.set_offset()` - Offset shape from center
/// - `GameObject.scale` - Scales the collision shape
#[pyclass(name = "ColliderShape")]
#[derive(Clone)]
pub struct PyColliderShape {
    pub(crate) shape: ColliderShape,
}

#[pymethods]
impl PyColliderShape {
    /// Create a circular collision shape.
    ///
    /// Creates a circle defined by its radius in local space. Circles are the most
    /// efficient shape for collision detection and are ideal for balls, enemies,
    /// pickups, and other round objects.
    ///
    /// # Arguments
    /// * `radius` - Circle radius in local units (before scaling)
    ///
    /// # Scaling Behavior
    /// - The actual collision radius = `radius * GameObject.scale`
    /// - Non-uniform scale (different X/Y) uses the average: `(scale.x + scale.y) / 2`
    /// - Use uniform scale for predictable results
    ///
    /// # Example: Basic Circle
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    /// engine.start_manual(width=800, height=600)
    ///
    /// # Create ball with circle collider
    /// ball = pyg.GameObject("Ball")
    /// ball.position = pyg.Vec2(0, 0)
    /// ball.scale = pyg.Vec2(1.0, 1.0)
    /// ball.set_mesh_geometry_circle(0.5, 32)
    /// ball.set_mesh_fill_color(pyg.Color.BLUE)
    ///
    /// # Circle shape with radius matching mesh
    /// shape = pyg.ColliderShape.circle(0.5)
    /// collider = pyg.Collider("BallCollider")
    /// collider.set_shape(shape)
    /// collider.set_layer(pyg.PhysicsLayers.PLAYER)
    /// collider.set_trigger(True)
    ///
    /// ball.add_component(collider)
    /// engine.add_game_object(ball)
    /// ```
    ///
    /// # Example: With Callbacks
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// def on_hit(other_id, nx, ny, penetration):
    ///     print(f"Hit object {other_id}!")
    ///
    /// shape = pyg.ColliderShape.circle(0.4)
    /// collider = pyg.Collider("Player")
    /// collider.set_shape(shape)
    /// collider.set_on_collision_enter(on_hit)
    /// ```
    ///
    /// # See Also
    /// - `box_shape()` - Create rectangle collider
    /// - `polygon()` - Create custom shape
    /// - `Collider.set_shape()` - Apply to collider
    #[staticmethod]
    fn circle(radius: f32) -> Self {
        Self {
            shape: ColliderShape::circle(radius),
        }
    }

    /// Create a box (rectangle) collision shape.
    ///
    /// Creates an axis-aligned bounding box (AABB) defined by half-extents (half width
    /// and half height from center). Box colliders are efficient and ideal for walls,
    /// platforms, and rectangular objects.
    ///
    /// **Note:** This creates an **axis-aligned** box that doesn't rotate with the object.
    /// For rotating boxes, use `obb()` instead.
    ///
    /// # Arguments
    /// * `half_width` - Half the box width (distance from center to edge)
    /// * `half_height` - Half the box height (distance from center to edge)
    ///
    /// # Sizing
    /// - Full width = `half_width * 2 * GameObject.scale.x`
    /// - Full height = `half_height * 2 * GameObject.scale.y`
    /// - For a 2x1 box: `box_shape(1.0, 0.5)`
    ///
    /// # Example: Platform Collider
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// engine = pyg.Engine()
    ///
    /// # Create 4x1 platform
    /// platform = pyg.GameObject("Platform")
    /// platform.position = pyg.Vec2(0, -3)
    /// platform.scale = pyg.Vec2(4.0, 1.0)
    /// platform.set_mesh_geometry_rectangle(1.0, 1.0)
    /// platform.set_mesh_fill_color(pyg.Color.rgb(100, 100, 100))
    ///
    /// # Box with half-extents matching 1x1 rectangle
    /// shape = pyg.ColliderShape.box_shape(0.5, 0.5)
    /// collider = pyg.Collider("PlatformCollider")
    /// collider.set_shape(shape)
    /// collider.set_layer(pyg.PhysicsLayers.ENVIRONMENT)
    ///
    /// platform.add_component(collider)
    /// engine.add_game_object(platform)
    /// ```
    ///
    /// # Example: Wall Boundaries
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// def create_wall(x, y, width, height):
    ///     wall = pyg.GameObject(f"Wall_{x}_{y}")
    ///     wall.position = pyg.Vec2(x, y)
    ///     wall.scale = pyg.Vec2(width, height)
    ///
    ///     shape = pyg.ColliderShape.box_shape(0.5, 0.5)
    ///     collider = pyg.Collider("WallCollider")
    ///     collider.set_shape(shape)
    ///     collider.set_layer(pyg.PhysicsLayers.ENVIRONMENT)
    ///     collider.set_collision_mask(pyg.PhysicsLayers.all())
    ///
    ///     wall.add_component(collider)
    ///     return wall
    ///
    /// # Create level boundaries
    /// left_wall = create_wall(-8, 0, 0.5, 10)
    /// right_wall = create_wall(8, 0, 0.5, 10)
    /// ```
    ///
    /// # See Also
    /// - `obb()` - Oriented box that rotates
    /// - `circle()` - Circle shape
    /// - `Collider.set_shape()` - Apply to collider
    #[staticmethod]
    fn box_shape(half_width: f32, half_height: f32) -> Self {
        Self {
            shape: ColliderShape::box_shape(Vec2::new(half_width, half_height)),
        }
    }

    /// Create an oriented box (OBB) collision shape.
    ///
    /// Creates a rectangular box that can be rotated in local space. Unlike regular
    /// `box_shape()`, OBBs follow the GameObject's rotation, making them ideal for
    /// rotating objects like spaceships, crates, or vehicles.
    ///
    /// # Arguments
    /// * `half_width` - Half the box width
    /// * `half_height` - Half the box height
    /// * `local_rotation` - Additional rotation in radians (adds to GameObject.rotation)
    ///
    /// # Rotation Behavior
    /// - Final rotation = `GameObject.rotation + local_rotation`
    /// - Use `local_rotation = 0.0` to match object rotation exactly
    /// - Use non-zero `local_rotation` for tilted colliders
    ///
    /// # Example: Rotating Spaceship
    /// ```python
    /// import pyg_engine as pyg
    /// import math
    ///
    /// engine = pyg.Engine()
    ///
    /// # Create spaceship with rotating collider
    /// ship = pyg.GameObject("Spaceship")
    /// ship.position = pyg.Vec2(0, 0)
    /// ship.rotation = 0.0
    /// ship.set_mesh_geometry_rectangle(1.0, 0.5)
    ///
    /// # OBB that rotates with the ship
    /// shape = pyg.ColliderShape.obb(0.5, 0.25, 0.0)
    /// collider = pyg.Collider("ShipCollider")
    /// collider.set_shape(shape)
    ///
    /// ship.add_component(collider)
    /// engine.add_game_object(ship)
    ///
    /// # Rotate ship in game loop
    /// while engine.poll_events():
    ///     ship.rotation += math.radians(45) * engine.delta_time
    ///     engine.update()
    ///     engine.render()
    /// ```
    ///
    /// # Example: Tilted Collider
    /// ```python
    /// import pyg_engine as pyg
    /// import math
    ///
    /// # Diamond-shaped collider (rotated 45°)
    /// shape = pyg.ColliderShape.obb(0.5, 0.5, math.radians(45))
    /// collider = pyg.Collider("Diamond")
    /// collider.set_shape(shape)
    /// ```
    ///
    /// # See Also
    /// - `box_shape()` - Non-rotating axis-aligned box
    /// - `polygon()` - Custom rotatable shape
    /// - `GameObject.rotation` - Object rotation
    #[staticmethod]
    fn obb(half_width: f32, half_height: f32, local_rotation: f32) -> Self {
        Self {
            shape: ColliderShape::obb(Vec2::new(half_width, half_height), local_rotation),
        }
    }

    /// Create a custom polygon collision shape.
    ///
    /// Creates a convex polygon from a list of vertices in local space. Polygons allow
    /// you to create custom shapes like triangles, hexagons, or irregular objects.
    ///
    /// **IMPORTANT:** Vertices must form a **convex** polygon (all interior angles < 180°).
    /// Concave polygons may cause incorrect collision detection.
    ///
    /// # Arguments
    /// * `vertices` - List of (x, y) tuples defining polygon vertices in local space
    ///
    /// # Vertex Requirements
    /// - Must have at least 3 vertices (triangle)
    /// - Should be in counter-clockwise order
    /// - Must form a convex shape (no indentations)
    /// - Centered around (0, 0) for best results
    ///
    /// # Example: Triangle Collider
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Upward-pointing triangle
    /// shape = pyg.ColliderShape.polygon([
    ///     (0.0, 0.5),    # Top
    ///     (-0.5, -0.5),  # Bottom left
    ///     (0.5, -0.5)    # Bottom right
    /// ])
    ///
    /// triangle = pyg.GameObject("Triangle")
    /// collider = pyg.Collider("TriCollider")
    /// collider.set_shape(shape)
    /// triangle.add_component(collider)
    /// ```
    ///
    /// # Example: Hexagon
    /// ```python
    /// import pyg_engine as pyg
    /// import math
    ///
    /// # Create regular hexagon
    /// radius = 0.5
    /// vertices = []
    /// for i in range(6):
    ///     angle = i * math.pi / 3  # 60° increments
    ///     x = radius * math.cos(angle)
    ///     y = radius * math.sin(angle)
    ///     vertices.append((x, y))
    ///
    /// shape = pyg.ColliderShape.polygon(vertices)
    /// collider = pyg.Collider("Hexagon")
    /// collider.set_shape(shape)
    /// ```
    ///
    /// # Example: Spaceship Shape
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Arrow-like spaceship
    /// shape = pyg.ColliderShape.polygon([
    ///     (0.6, 0.0),    # Nose
    ///     (0.0, 0.3),    # Top left
    ///     (-0.4, 0.3),   # Wing top left
    ///     (-0.4, -0.3),  # Wing bottom left
    ///     (0.0, -0.3)    # Bottom left
    /// ])
    ///
    /// ship = pyg.GameObject("Ship")
    /// collider = pyg.Collider("ShipCollider")
    /// collider.set_shape(shape)
    /// ship.add_component(collider)
    /// ```
    ///
    /// # See Also
    /// - `circle()` - Simple round shape
    /// - `box_shape()` - Rectangle
    /// - `obb()` - Rotating rectangle
    #[staticmethod]
    fn polygon(vertices: Vec<(f32, f32)>) -> Self {
        let verts: Vec<Vec2> = vertices.iter().map(|&(x, y)| Vec2::new(x, y)).collect();
        Self {
            shape: ColliderShape::polygon(verts),
        }
    }
}

/// Collider component for collision detection.
///
/// The Collider component adds collision detection capabilities to GameObjects. It defines
/// the shape, layer, and callbacks for detecting when objects overlap. Colliders work with
/// the built-in physics system to automatically detect and report collisions.
///
/// # Features
/// - **Multiple shapes**: Circle, box, oriented box, polygon
/// - **Layer-based filtering**: Control which objects collide
/// - **Trigger mode**: Detect overlaps without physics response
/// - **Callbacks**: on_collision_enter, on_collision_stay, on_collision_exit
/// - **Efficient detection**: AABB tree broad-phase + SAT narrow-phase
///
/// # Workflow
/// 1. Create collider: `collider = pyg.Collider("name")`
/// 2. Set shape: `collider.set_shape(pyg.ColliderShape.circle(0.5))`
/// 3. Configure layers: `collider.set_layer()` and `set_collision_mask()`
/// 4. Set callbacks: `collider.set_on_collision_enter(callback)`
/// 5. Add to object: `obj.add_component(collider)`
/// 6. Add to engine: `engine.add_game_object(obj)`
///
/// # Example: Basic Player Collider
/// ```python
/// import pyg_engine as pyg
///
/// engine = pyg.Engine()
/// engine.start_manual(width=800, height=600)
/// engine.set_camera_viewport_size(10.0, 10.0)
///
/// # Create player
/// player = pyg.GameObject("Player")
/// player.position = pyg.Vec2(0, 0)
/// player.set_mesh_geometry_circle(0.4, 32)
/// player.set_mesh_fill_color(pyg.Color.BLUE)
///
/// # Add collider
/// collider = pyg.Collider("PlayerCollider")
/// collider.set_shape(pyg.ColliderShape.circle(0.4))
/// collider.set_layer(pyg.PhysicsLayers.PLAYER)
/// collider.set_collision_mask(pyg.PhysicsLayers.all())
/// collider.set_trigger(True)
///
/// player.add_component(collider)
/// engine.add_game_object(player)
/// ```
///
/// # Example: Collision Callbacks
/// ```python
/// import pyg_engine as pyg
///
/// class Player:
///     def __init__(self):
///         self.obj = pyg.GameObject("Player")
///         self.obj.position = pyg.Vec2(0, 0)
///         self.obj.set_mesh_geometry_circle(0.4, 32)
///         self.obj.set_mesh_fill_color(pyg.Color.BLUE)
///
///         collider = pyg.Collider("PlayerCollider")
///         collider.set_shape(pyg.ColliderShape.circle(0.4))
///         collider.set_layer(pyg.PhysicsLayers.PLAYER)
///         collider.set_collision_mask(pyg.PhysicsLayers.all())
///         collider.set_trigger(True)
///
///         # Set up collision callbacks
///         collider.set_on_collision_enter(self.on_hit)
///         collider.set_on_collision_exit(self.on_exit)
///
///         self.obj.add_component(collider)
///
///     def on_hit(self, other_id, nx, ny, penetration):
///         print(f"Hit object {other_id}!")
///         self.obj.set_mesh_fill_color(pyg.Color.RED)
///
///     def on_exit(self, other_id):
///         print(f"Stopped touching {other_id}")
///         self.obj.set_mesh_fill_color(pyg.Color.BLUE)
///
/// engine = pyg.Engine()
/// player = Player()
/// engine.add_game_object(player.obj)
/// ```
///
/// # Example: Selective Collision
/// ```python
/// import pyg_engine as pyg
///
/// # Bullet that only hits enemies
/// bullet = pyg.GameObject("Bullet")
/// bullet_collider = pyg.Collider("BulletCollider")
/// bullet_collider.set_shape(pyg.ColliderShape.circle(0.1))
/// bullet_collider.set_layer(pyg.PhysicsLayers.PROJECTILE)
/// bullet_collider.set_collision_mask(
///     pyg.PhysicsLayers.create_mask([pyg.PhysicsLayers.ENEMY])
/// )  # Only collides with enemies
/// bullet_collider.set_trigger(True)
/// bullet.add_component(bullet_collider)
///
/// # Enemy that detects player and projectiles
/// enemy = pyg.GameObject("Enemy")
/// enemy_collider = pyg.Collider("EnemyCollider")
/// enemy_collider.set_shape(pyg.ColliderShape.circle(0.5))
/// enemy_collider.set_layer(pyg.PhysicsLayers.ENEMY)
/// enemy_collider.set_collision_mask(
///     pyg.PhysicsLayers.create_mask([
///         pyg.PhysicsLayers.PLAYER,
///         pyg.PhysicsLayers.PROJECTILE
///     ])
/// )
/// enemy.add_component(enemy_collider)
/// ```
///
/// # See Also
/// - `ColliderShape` - Create collision shapes
/// - `PhysicsLayers` - Layer constants and masks
/// - `GameObject.add_component()` - Attach collider to object
#[pyclass(name = "Collider")]
pub struct PyCollider {
    pub(crate) component: ColliderComponent,
}

#[pymethods]
impl PyCollider {
    /// Create a new collider component.
    ///
    /// Creates an empty collider that needs to be configured with a shape, layer, and
    /// collision mask before use. The collider won't detect collisions until added to a
    /// GameObject and the object is added to the engine.
    ///
    /// # Arguments
    /// * `name` - Identifier for debugging (e.g., "PlayerCollider")
    ///
    /// # Default Values
    /// - Shape: Circle with radius 0.5
    /// - Layer: `PhysicsLayers.DEFAULT` (0)
    /// - Collision mask: All layers (`0xFFFFFFFF`)
    /// - Trigger: `False`
    /// - Offset: (0, 0)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Create and configure collider
    /// collider = pyg.Collider("MyCollider")
    /// collider.set_shape(pyg.ColliderShape.circle(0.5))
    /// collider.set_layer(pyg.PhysicsLayers.PLAYER)
    /// collider.set_trigger(True)
    ///
    /// # Add to GameObject
    /// obj = pyg.GameObject("Object")
    /// obj.add_component(collider)
    /// ```
    ///
    /// # See Also
    /// - `set_shape()` - Configure collision shape
    /// - `set_layer()` - Set physics layer
    /// - `GameObject.add_component()` - Attach to object
    #[new]
    fn new(name: String) -> Self {
        Self {
            component: ColliderComponent::new(name),
        }
    }

    /// Set the collision shape.
    ///
    /// Defines the shape used for collision detection. The shape is in local space
    /// and will be transformed by the GameObject's position, rotation, and scale.
    ///
    /// # Arguments
    /// * `shape` - ColliderShape created with `ColliderShape.circle()`, `box_shape()`, etc.
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// collider = pyg.Collider("Collider")
    ///
    /// # Set circle shape
    /// collider.set_shape(pyg.ColliderShape.circle(0.5))
    ///
    /// # Or box shape
    /// collider.set_shape(pyg.ColliderShape.box_shape(0.5, 0.3))
    /// ```
    ///
    /// # See Also
    /// - `ColliderShape.circle()` - Create circle
    /// - `ColliderShape.box_shape()` - Create box
    /// - `ColliderShape.polygon()` - Create polygon
    fn set_shape(&mut self, shape: PyColliderShape) {
        self.component.set_shape(shape.shape);
    }

    /// Set the collider's offset from the GameObject center.
    ///
    /// Moves the collision shape relative to the GameObject's position. Useful when
    /// the visual center doesn't match the collision center, or for creating multiple
    /// colliders on one object.
    ///
    /// # Arguments
    /// * `x` - Horizontal offset in local space
    /// * `y` - Vertical offset in local space
    ///
    /// # Offset Behavior
    /// - Final position = `GameObject.position + offset`
    /// - Offset rotates with the GameObject
    /// - Offset scales with the GameObject
    ///
    /// # Example: Offset Collider
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Create character with head collider
    /// character = pyg.GameObject("Character")
    /// character.position = pyg.Vec2(0, 0)
    ///
    /// # Head collider offset upward
    /// head_collider = pyg.Collider("Head")
    /// head_collider.set_shape(pyg.ColliderShape.circle(0.3))
    /// head_collider.set_offset(0.0, 0.5)  # Move up
    /// character.add_component(head_collider)
    ///
    /// # Body collider centered
    /// body_collider = pyg.Collider("Body")
    /// body_collider.set_shape(pyg.ColliderShape.circle(0.4))
    /// body_collider.set_offset(0.0, 0.0)  # Centered
    /// character.add_component(body_collider)
    /// ```
    ///
    /// # See Also
    /// - `GameObject.position` - Object's world position
    fn set_offset(&mut self, x: f32, y: f32) {
        self.component.set_offset(Vec2::new(x, y));
    }

    /// Set the physics layer (0-31).
    ///
    /// Assigns this collider to a physics layer group. Each collider belongs to exactly
    /// one layer. Layers are used with collision masks to filter which objects can collide.
    ///
    /// # Arguments
    /// * `layer` - Layer ID (0-31), use `PhysicsLayers` constants
    ///
    /// # Built-in Layers
    /// - `PhysicsLayers.DEFAULT` (0)
    /// - `PhysicsLayers.PLAYER` (1)
    /// - `PhysicsLayers.ENEMY` (2)
    /// - `PhysicsLayers.PROJECTILE` (3)
    /// - `PhysicsLayers.ENVIRONMENT` (4)
    /// - `PhysicsLayers.TRIGGER` (5)
    /// - `PhysicsLayers.UI` (6)
    /// - `PhysicsLayers.PICKUP` (7)
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Assign to player layer
    /// collider = pyg.Collider("Player")
    /// collider.set_layer(pyg.PhysicsLayers.PLAYER)
    ///
    /// # Assign to environment layer
    /// wall_collider = pyg.Collider("Wall")
    /// wall_collider.set_layer(pyg.PhysicsLayers.ENVIRONMENT)
    /// ```
    ///
    /// # See Also
    /// - `set_collision_mask()` - Set which layers to collide with
    /// - `PhysicsLayers` - Layer constants
    /// - `layer` (property) - Get current layer
    fn set_layer(&mut self, layer: u32) {
        self.component.set_layer(layer);
    }

    /// Set the collision mask.
    ///
    /// Defines which layers this collider can collide with. The mask is a bitfield where
    /// each bit represents a layer. Collision occurs if both objects' masks include each
    /// other's layers.
    ///
    /// # Arguments
    /// * `mask` - 32-bit bitmask, use `PhysicsLayers.create_mask()` to build
    ///
    /// # Example: Selective Collision
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Player collides with enemies and environment
    /// player = pyg.Collider("Player")
    /// player.set_layer(pyg.PhysicsLayers.PLAYER)
    /// player.set_collision_mask(pyg.PhysicsLayers.create_mask([
    ///     pyg.PhysicsLayers.ENEMY,
    ///     pyg.PhysicsLayers.ENVIRONMENT
    /// ]))
    ///
    /// # Bullet only collides with enemies
    /// bullet = pyg.Collider("Bullet")
    /// bullet.set_layer(pyg.PhysicsLayers.PROJECTILE)
    /// bullet.set_collision_mask(pyg.PhysicsLayers.create_mask([
    ///     pyg.PhysicsLayers.ENEMY
    /// ]))
    /// ```
    ///
    /// # Example: Collide With All
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Wall that blocks everything
    /// wall = pyg.Collider("Wall")
    /// wall.set_collision_mask(pyg.PhysicsLayers.all())
    /// ```
    ///
    /// # See Also
    /// - `set_layer()` - Set this object's layer
    /// - `PhysicsLayers.create_mask()` - Build mask from layers
    /// - `collision_mask` (property) - Get current mask
    fn set_collision_mask(&mut self, mask: u32) {
        self.component.set_collision_mask(mask);
    }

    /// Set whether this is a trigger collider.
    ///
    /// Trigger colliders detect overlaps and fire callbacks but don't produce physical
    /// responses (no bouncing or blocking). Use triggers for pickup zones, damage areas,
    /// level triggers, and other non-solid detectors.
    ///
    /// # Arguments
    /// * `is_trigger` - `True` for trigger (no physics), `False` for solid (future physics)
    ///
    /// # Trigger vs. Solid
    /// - **Trigger (True)**: Detects collisions, fires callbacks, objects pass through
    /// - **Solid (False)**: Reserved for future physics simulation (currently behaves like trigger)
    ///
    /// # Example: Pickup Item
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// class Coin:
    ///     def __init__(self, x, y):
    ///         self.obj = pyg.GameObject("Coin")
    ///         self.obj.position = pyg.Vec2(x, y)
    ///         self.obj.set_mesh_geometry_circle(0.2, 32)
    ///         self.obj.set_mesh_fill_color(pyg.Color.rgb(255, 215, 0))
    ///
    ///         collider = pyg.Collider("CoinCollider")
    ///         collider.set_shape(pyg.ColliderShape.circle(0.2))
    ///         collider.set_layer(pyg.PhysicsLayers.PICKUP)
    ///         collider.set_trigger(True)  # Pass through
    ///         collider.set_on_collision_enter(self.on_pickup)
    ///
    ///         self.obj.add_component(collider)
    ///
    ///     def on_pickup(self, other_id, nx, ny, penetration):
    ///         print("Coin collected!")
    ///         # Remove coin (in real game)
    /// ```
    ///
    /// # Example: Damage Zone
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// # Lava pit that damages player
    /// lava = pyg.GameObject("Lava")
    /// lava.position = pyg.Vec2(0, -5)
    /// lava.scale = pyg.Vec2(4, 1)
    ///
    /// collider = pyg.Collider("LavaCollider")
    /// collider.set_shape(pyg.ColliderShape.box_shape(0.5, 0.5))
    /// collider.set_layer(pyg.PhysicsLayers.TRIGGER)
    /// collider.set_trigger(True)  # Player can enter
    /// collider.set_on_collision_stay(lambda oid, nx, ny, pen: print("Taking damage!"))
    ///
    /// lava.add_component(collider)
    /// ```
    ///
    /// # See Also
    /// - `is_trigger` (property) - Check if trigger
    /// - `set_on_collision_enter()` - Detect entry
    fn set_trigger(&mut self, is_trigger: bool) {
        self.component.set_trigger(is_trigger);
    }

    /// Get the physics layer.
    ///
    /// Returns the layer ID this collider belongs to (0-31).
    ///
    /// # Returns
    /// Layer ID as unsigned 32-bit integer
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// collider = pyg.Collider("Player")
    /// collider.set_layer(pyg.PhysicsLayers.PLAYER)
    /// print(collider.layer)  # Output: 1
    /// ```
    ///
    /// # See Also
    /// - `set_layer()` - Set the layer
    #[getter]
    fn layer(&self) -> u32 {
        self.component.layer()
    }

    /// Get the collision mask.
    ///
    /// Returns the bitmask defining which layers this collider can detect.
    ///
    /// # Returns
    /// 32-bit bitmask
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// collider = pyg.Collider("Player")
    /// collider.set_collision_mask(pyg.PhysicsLayers.all())
    /// print(hex(collider.collision_mask))  # Output: 0xffffffff
    /// ```
    ///
    /// # See Also
    /// - `set_collision_mask()` - Set the mask
    #[getter]
    fn collision_mask(&self) -> u32 {
        self.component.collision_mask()
    }

    /// Check if this is a trigger collider.
    ///
    /// Returns whether this collider is set to trigger mode (no physics response).
    ///
    /// # Returns
    /// `True` if trigger, `False` if solid
    ///
    /// # Example
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// collider = pyg.Collider("Pickup")
    /// collider.set_trigger(True)
    /// print(collider.is_trigger)  # Output: True
    /// ```
    ///
    /// # See Also
    /// - `set_trigger()` - Set trigger mode
    #[getter]
    fn is_trigger(&self) -> bool {
        self.component.is_trigger()
    }

    /// Set callback fired when collision starts.
    ///
    /// Registers a Python function to be called once when this collider first overlaps
    /// with another collider. The callback fires on the first frame of contact.
    ///
    /// # Arguments
    /// * `callback` - Function with signature: `fn(other_id, normal_x, normal_y, penetration)`
    ///   - `other_id` (int): GameObject ID of the other collider
    ///   - `normal_x` (float): X component of collision normal (points away from other)
    ///   - `normal_y` (float): Y component of collision normal
    ///   - `penetration` (float): Overlap depth (how far objects penetrate)
    ///
    /// # Callback Timing
    /// - Fires **once** when collision begins
    /// - Won't fire again until objects separate and re-collide
    /// - Called during the engine's fixed update (60 Hz)
    ///
    /// # Example: Simple Detection
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// def on_hit(other_id, nx, ny, penetration):
    ///     print(f"Hit object {other_id}!")
    ///     print(f"Normal: ({nx:.2f}, {ny:.2f})")
    ///     print(f"Penetration: {penetration:.2f}")
    ///
    /// collider = pyg.Collider("Player")
    /// collider.set_shape(pyg.ColliderShape.circle(0.5))
    /// collider.set_on_collision_enter(on_hit)
    /// ```
    ///
    /// # Example: Color Change on Hit
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// class Player:
    ///     def __init__(self):
    ///         self.obj = pyg.GameObject("Player")
    ///         self.obj.set_mesh_geometry_circle(0.4, 32)
    ///         self.obj.set_mesh_fill_color(pyg.Color.BLUE)
    ///
    ///         collider = pyg.Collider("PlayerCollider")
    ///         collider.set_shape(pyg.ColliderShape.circle(0.4))
    ///         collider.set_on_collision_enter(self.on_enter)
    ///         collider.set_on_collision_exit(self.on_exit)
    ///
    ///         self.obj.add_component(collider)
    ///
    ///     def on_enter(self, other_id, nx, ny, penetration):
    ///         self.obj.set_mesh_fill_color(pyg.Color.RED)
    ///
    ///     def on_exit(self, other_id):
    ///         self.obj.set_mesh_fill_color(pyg.Color.BLUE)
    /// ```
    ///
    /// # Example: Track Multiple Collisions
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// class Player:
    ///     def __init__(self):
    ///         self.collision_count = 0
    ///         self.obj = pyg.GameObject("Player")
    ///
    ///         collider = pyg.Collider("PlayerCollider")
    ///         collider.set_shape(pyg.ColliderShape.circle(0.4))
    ///         collider.set_on_collision_enter(self.on_enter)
    ///         collider.set_on_collision_exit(self.on_exit)
    ///         self.obj.add_component(collider)
    ///
    ///     def on_enter(self, other_id, nx, ny, penetration):
    ///         self.collision_count += 1
    ///         print(f"Colliding with {self.collision_count} objects")
    ///
    ///     def on_exit(self, other_id):
    ///         self.collision_count -= 1
    ///         print(f"Colliding with {self.collision_count} objects")
    /// ```
    ///
    /// # See Also
    /// - `set_on_collision_stay()` - Fires every frame during collision
    /// - `set_on_collision_exit()` - Fires when collision ends
    fn set_on_collision_enter(&mut self, callback: Py<PyAny>) {
        self.component.set_on_collision_enter(move |other_id, normal, penetration| {
            Python::with_gil(|py| {
                let _ = callback.call1(
                    py,
                    (other_id, normal.x(), normal.y(), penetration)
                );
            });
        });
    }

    /// Set callback fired every frame during collision.
    ///
    /// Registers a Python function to be called continuously while this collider overlaps
    /// with another collider. Fires every fixed update frame (60 Hz) during contact.
    ///
    /// # Arguments
    /// * `callback` - Function with signature: `fn(other_id, normal_x, normal_y, penetration)`
    ///   - `other_id` (int): GameObject ID of the other collider
    ///   - `normal_x` (float): X component of collision normal
    ///   - `normal_y` (float): Y component of collision normal
    ///   - `penetration` (float): Overlap depth
    ///
    /// # Callback Timing
    /// - Fires **every frame** while colliding (after initial on_enter)
    /// - Called at 60 Hz during fixed update
    /// - Use for continuous effects like damage over time
    ///
    /// # Performance Note
    /// `on_collision_stay` can fire hundreds of times per second during long collisions.
    /// Keep callbacks lightweight or use timers/counters for expensive operations.
    ///
    /// # Example: Damage Over Time
    /// ```python
    /// import pyg_engine as pyg
    /// import time
    ///
    /// class Player:
    ///     def __init__(self):
    ///         self.obj = pyg.GameObject("Player")
    ///         self.health = 100
    ///         self.last_damage_time = 0
    ///         self.damage_cooldown = 0.5  # 0.5 seconds between damage
    ///
    ///         collider = pyg.Collider("PlayerCollider")
    ///         collider.set_shape(pyg.ColliderShape.circle(0.4))
    ///         collider.set_on_collision_stay(self.on_stay_in_lava)
    ///         self.obj.add_component(collider)
    ///
    ///     def on_stay_in_lava(self, other_id, nx, ny, penetration):
    ///         current_time = time.time()
    ///         if current_time - self.last_damage_time >= self.damage_cooldown:
    ///             self.health -= 10
    ///             self.last_damage_time = current_time
    ///             print(f"Damage! Health: {self.health}")
    /// ```
    ///
    /// # Example: Continuous Force
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// class Magnet:
    ///     def __init__(self):
    ///         self.obj = pyg.GameObject("Magnet")
    ///         self.pull_strength = 2.0
    ///
    ///         collider = pyg.Collider("MagnetField")
    ///         collider.set_shape(pyg.ColliderShape.circle(3.0))  # Large radius
    ///         collider.set_trigger(True)
    ///         collider.set_on_collision_stay(self.pull_object)
    ///         self.obj.add_component(collider)
    ///
    ///     def pull_object(self, other_id, nx, ny, penetration):
    ///         # Pull towards magnet (reverse normal direction)
    ///         # In real game, would apply force to other object
    ///         pull_dir_x = -nx * self.pull_strength
    ///         pull_dir_y = -ny * self.pull_strength
    ///         print(f"Pulling object {other_id}: ({pull_dir_x:.2f}, {pull_dir_y:.2f})")
    /// ```
    ///
    /// # See Also
    /// - `set_on_collision_enter()` - Fires once when collision starts
    /// - `set_on_collision_exit()` - Fires once when collision ends
    fn set_on_collision_stay(&mut self, callback: Py<PyAny>) {
        self.component.set_on_collision_stay(move |other_id, normal, penetration| {
            Python::with_gil(|py| {
                let _ = callback.call1(
                    py,
                    (other_id, normal.x(), normal.y(), penetration)
                );
            });
        });
    }

    /// Set callback fired when collision ends.
    ///
    /// Registers a Python function to be called once when this collider stops overlapping
    /// with another collider. Fires on the first frame after separation.
    ///
    /// # Arguments
    /// * `callback` - Function with signature: `fn(other_id)`
    ///   - `other_id` (int): GameObject ID of the other collider that was exited
    ///
    /// # Callback Timing
    /// - Fires **once** when objects separate
    /// - Won't fire again until objects re-collide and separate again
    /// - Called during the engine's fixed update (60 Hz)
    ///
    /// # Example: Reset on Exit
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// class Player:
    ///     def __init__(self):
    ///         self.obj = pyg.GameObject("Player")
    ///         self.obj.set_mesh_geometry_circle(0.4, 32)
    ///         self.obj.set_mesh_fill_color(pyg.Color.BLUE)
    ///
    ///         collider = pyg.Collider("PlayerCollider")
    ///         collider.set_shape(pyg.ColliderShape.circle(0.4))
    ///         collider.set_on_collision_enter(self.on_enter)
    ///         collider.set_on_collision_exit(self.on_exit)
    ///         self.obj.add_component(collider)
    ///
    ///     def on_enter(self, other_id, nx, ny, penetration):
    ///         self.obj.set_mesh_fill_color(pyg.Color.RED)
    ///         print(f"Entered collision with {other_id}")
    ///
    ///     def on_exit(self, other_id):
    ///         self.obj.set_mesh_fill_color(pyg.Color.BLUE)
    ///         print(f"Exited collision with {other_id}")
    /// ```
    ///
    /// # Example: Track Active Collisions
    /// ```python
    /// import pyg_engine as pyg
    ///
    /// class Sensor:
    ///     def __init__(self):
    ///         self.obj = pyg.GameObject("Sensor")
    ///         self.detected_objects = set()
    ///
    ///         collider = pyg.Collider("SensorCollider")
    ///         collider.set_shape(pyg.ColliderShape.circle(2.0))  # Detection radius
    ///         collider.set_trigger(True)
    ///         collider.set_on_collision_enter(self.on_enter)
    ///         collider.set_on_collision_exit(self.on_exit)
    ///         self.obj.add_component(collider)
    ///
    ///     def on_enter(self, other_id, nx, ny, penetration):
    ///         self.detected_objects.add(other_id)
    ///         print(f"Detecting {len(self.detected_objects)} objects")
    ///
    ///     def on_exit(self, other_id):
    ///         self.detected_objects.discard(other_id)
    ///         print(f"Detecting {len(self.detected_objects)} objects")
    /// ```
    ///
    /// # See Also
    /// - `set_on_collision_enter()` - Fires when collision starts
    /// - `set_on_collision_stay()` - Fires every frame during collision
    fn set_on_collision_exit(&mut self, callback: Py<PyAny>) {
        self.component.set_on_collision_exit(move |other_id| {
            Python::with_gil(|py| {
                let _ = callback.call1(py, (other_id,));
            });
        });
    }
}

/// Register collision detection bindings with Python
pub fn register_physics_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPhysicsLayers>()?;
    m.add_class::<PyColliderShape>()?;
    m.add_class::<PyCollider>()?;
    Ok(())
}
