'''
Sprite Collider Features:
    Pixel-perfect collision detection
    Alpha threshold collision
    Bounding box fallback
    Circle approximation for performance
    Collision mask caching
'''
import pyg_engine
from pyg_engine import Component, GameObject

class SpriteCollider(Component):
    def __init__(self, gameobject: GameObject):
        super().__init__(gameobject)

