import pygame as pg
import random
from pygame import Color
from pyg_engine import Script, RigidBody, MouseHoverComponent, MouseClickComponent, MouseButton
from pyg_engine import Vector2
class PlayerScript(Script):


    # Minimal Header:
    # def __init__(self, gameobject)
    def __init__(self, gameobject, speed=1):
        # Script initialization. Default arguments must be (self, gameobject)

        super().__init__(gameobject) # Initialize the Script parent class
        self.gameobject = gameobject # Not necessary but needed to do any processes with the gameobject itself
        self.speed = speed # Example of a passed in value

        # Should print "------>Player initialized, speed: 4.2"
        print("------>Player initialized, speed: {}".format(speed))

        self.first_update = False


    def update(self, engine):
        # Updates every frame

        if self.first_update is False:
            print("------>First player update")
            self.first_update = True
        else:
            pass

    def start(self, engine):
        # Called upon startup

        print("------>Player is started!")

    def on_destroy(self):
        print("------>Player is destroyed!")

