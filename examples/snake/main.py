import pyg
from pyg import *
import numpy as np
from enum import Enum

class Direction(Enum):
    UP = 1
    DOWN = 2
    LEFT = 3
    RIGHT = 4

class Snake(GameObject):
    def __init__(self, engine: Engine):
        # Snake initialization function
        pyg.log("Created Snake Object")

        # Main game loop:
        self.rows = 15
        self.columns = 15
        self.speed = 1.0
        self.num_points = 0
        self.map = np.zeros((self.rows, self.columns))
        self.direction = Direction.UP
        self.head_color = Color("#00BB00")
        self.body_color = Color("#FFFF00")
        self.fruit_color = Color("#FF00FF")
        self.head = (int(self.rows/2), int(self.columns/2))
        self.tail = self.head

    def add_fruit(self):
        pass

    def move_snake(self, direction: Direction):

        pass

    def start(self):
        pyg.log("Started snake object")


    def update(self, deltatime: DeltaTime):
        # This function is called every frame

        canvas = self.engine.window.canvas
        # We can draw onto the window's canvas

        # Here, we do all rendering

        # With pyg-engine, there's two modes of drawing,
        # Either you can draw directly to the canvas using basic shapes / images
        # Or, you can use sprites and the camera system

        # Drawing directly may be easier for programs displaying simple shapes
        # (i.e., snake or tetris). Think PyGame

        # Using the sprite rendering option would be better for anything using
        # the gameobject class such as side scrollers or other more complex
        # programs / games where you have multiple moving components,
        # visible game objects, etc. Think Unity Engine

        # Clears everything on the screen (Sets to the camera background color)
        canvas.clear()


        # Draw every element in our snake map
        for r in range(self.rows):
            for c in range(self.columns):
                # Check to see if a snake part is inside here
                if self.map[r,c] == 0:
                    continue

                color = self.body_color

                # Check to see if the current index is our head
                if r == self.head[0] and c == self.head[1]:
                    # Set our color to the head color
                    color = self.head_color
                # Check to see if there's a fruit
                elif self.map[r,c] == 2:
                    color = self.fruit_color

                # Draw the square of the snake

                # Width / Height of the square
                width = self.engine.window.size.x / self.columns
                height = self.engine.window.size.y / self.rows

                # Top left position of the square
                pos_x = width * c
                pos_y = height * r

                # draw_rect(x, y, width, height, color)
                canvas.draw_rect(pos_x, pos_y, width, height, color)

                # Optionally:
                # draw_rect(Vec2(pos_x, pos_y), Vec2(width, height), color)
        pass

    def fixed_update(self):
        # This function is called at a fixed rate (i.e., 60 times a second)
        # Helpful if you wanted to create your own physics implementation
        pass

    def on_destroyed(self):
        # This function is called when the game object is destroyed
        pyg.log("Destroyed snake object")


def main():
    pyg.log("Starting Snake!")

    # Initialize the pyg-engine core
    engine = pyg.Engine()

    # Set the window size
    engine.window.size = (700, 700)

    # Disable the resize function
    # (disabled by default but if you wanted to change it, here it is)
    engine.window.resize = False

    # Set FPS cap, default is uncapped (any value < 0)
    engine.fps = 60

    # Create our scene, this is where all game objects will exist
    scene = Scene()

    # The scene object has a camera object we can change to fit our needs
    # - CameraType.Screenspace (default) indicates that the window serves as
    # - the canvas for everything. (0,0) will always be at the center of the
    # - worldspace
    scene.camera.type = CameraType.Screenspace

    # Screenspace.Center (default) specifies that our (0,0) point is at the
    # - center of the screen. This can be changed to Screenspace.TopLeft,
    # - TopCenter, TopRight, Left, Right, BottomLeft, BottomCenter, BottomRight
    # - depending on your needs. The Screenspace location type is used only for
    # - the Screenspace camera and/or canvas
    scene.camera.screenspace_anchor = Screenspace.Center

    # - CameraType.Worldspace would act as if the camera was a game object-
    # - containing its own location (that can be changed), rotation, size, etc.
    # - This would be more helpful with things like 2D platformers
    # - scene.camera.type = CameraType.Worldspace

    scene.camera.background = Color("#101010")

    snake = Snake()
    scene.add_gameobject(snake)


    # The scene contains all our game information, so we need to add this to
    # the engine. The scene object lets us easily swap out game scenes.
    # I.e., helpful for platformers with multiple levels and others
    engine.set_scene(scene)

    # Start the scene
    engine.begin()




if __name__ == "__main__":
    main()
