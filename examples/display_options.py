import pygame
from pygame import Color
import pyg_engine
from pyg_engine import Engine, Size


engine = Engine(
        size=Size(w=2560, h=1600),
        backgroundColor=Color(50, 50, 50),  # Dark gray background
        windowName="Pyg Engine - Basic Example",
        displayMode=pygame.OPENGL | pygame.FULLSCREEN
        )




