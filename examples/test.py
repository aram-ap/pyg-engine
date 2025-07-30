import pyg_engine
from pyg_engine import BasicShape, Engine, GameObject, PymunkBoxCollider, PymunkRigidBody, Size
from pygame import Color, Vector2

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

player.add_component( PymunkRigidBody,
                              mass=1.0,  # Heavy
                              gravity_scale=1.0,  # Normal gravity to reduce oscillation energy
                              drag=0.15,  # Higher drag to dampen oscillations
                              use_gravity=True,  # Make it fall
                              lock_rotation=False )  # Allow natural rolling

# Adding colliders. Use PymunkCircleCollider for circles
player.add_component(
        PymunkBoxCollider,
        width=50,
        height=50,
        material=pyg_engine.Materials.METAL,
        collision_layer="Player"
        )

# Example of adding scripts
player.add_script("scripts/test_script.py",
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
floor.add_component(PymunkRigidBody,
                    is_kinematic=True, # Kinematic necessary for physics simulations
                    use_gravity=False)
floor.add_component(
        PymunkBoxCollider,
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

