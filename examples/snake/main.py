from pyg import *

def main():
    print("Starting Snake!")

    # Initialize the pyg-engine core
    engine = Engine()

    # Set the window size
    engine.window.size = (800, 600)

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

    # This specifies that our (0,0) point is at the center of the screen.
    # - This can be changed to Screenspace.TopLeft, TopCenter, TopRight, Left,
    # - Right, BottomLeft, BottomCenter, BottomRight depending on your needs.
    # - The Screenspace location type is used only for the Screenspace camera
    # - and/or canvas
    scene.camera.screenspace_anchor = Screenspace.Center

    # CameraType.Worldspace would act as if the camera was a game object-
    # containing its own location (that can be changed), rotation, size, etc.
    # This would be more helpful with things like 2D platformers
    # scene.camera.type = CameraType.Worldspace


    # Start the game mechanics
    engine.begin()




if __name__ == "__main__":
    main()
