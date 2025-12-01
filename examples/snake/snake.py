#from pyg import GameObject, DeltaTime, Sprite,

class Snake(GameObject):

    __init__(self, engine: Engine):
        # Snake initialization function
        pyg.log("Created Snake Object")


    def start(self):
        pyg.log("Started snake object")


    def update(self, deltatime: DeltaTime):
        # This function is called every frame
        pass

    def fixed_update(self):
        # This function is called at a fixed rate (i.e., 60 times a second)
        # Helpful if you wanted to create your own physics implementation
        pass

    def on_destroyed(self):
        # This function is called when the game object is destroyed
        pyg.log("Destroyed snake object")


