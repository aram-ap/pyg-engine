import pyg_engine
from pathlib import Path
from pyg_engine import BasicShape, Engine, GameObject, BoxCollider, RigidBody, Size
from pygame import Color, Vector2

examples_dir = Path(__file__).parent
scripts_dir = examples_dir / "scripts"
test_script_path = scripts_dir / "test_script.py"

# Create the engine
engine = Engine(
    size=Size(w=800, h=600),
    backgroundColor=Color(0, 0, 0),
    windowName="My Game"
)

engine.physics_system.collision_layers = {
    "Player": ["Player", "Environment"],  # Players collide with other players and environment
    "Environment": ["Player"],  # Environment collides with players
    "NoCollision": []  # Objects that don't collide
}


# Create a game object
player = GameObject(
    name="Player",
    basicShape=BasicShape.Rectangle,
    position=(200, 200),
    size=(50, 50),
    color=Color(255, 0, 0),
    rotation=15
)

player.add_component( RigidBody,
                              mass=1.0,  # Heavy
                              gravity_scale=1.0,  # Normal gravity to reduce oscillation energy
                              drag=0.15,  # Higher drag to dampen oscillations
                              use_gravity=True,  # Make it fall
                              lock_rotation=False )  # Allow natural rolling

# Adding colliders. Use CircleCollider for circles
player.add_component(
        BoxCollider,
        width=50,
        height=50,
        material=pyg_engine.Materials.METAL,
        collision_layer="Player"
        )

# Example of adding scripts
player.add_script(test_script_path,
                  speed=4.2
                  )

# Lets add a floor!
floor  = GameObject(
        name="floor",
        basicShape=BasicShape.Rectangle,
        position=(0,-100),
        size=(800,20),
        color=Color("Yellow"),
        )

# Every interactable gameobject needs a rigidbody and collider
floor.add_component(RigidBody,
                    is_kinematic=True, # Kinematic necessary for physics simulations
                    use_gravity=False)
floor.add_component(
        BoxCollider,
        width=800,
        height=20,
        material=pyg_engine.Materials.DEFAULT,
        collision_layer = "Environment"
        )


# Add to engine
engine.addGameObject(player)
engine.addGameObject(floor)

# Start the game loop
engine.start()

