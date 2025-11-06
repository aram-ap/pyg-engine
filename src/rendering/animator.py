'''
Frame-based animation with timing
    Sprite sheet support with automatic frame extraction
    Animation state machine for complex behaviors
    Loop and ping-pong animation modes
    Animation events for triggers
    Blend transitions between animations
'''
import pyg_engine
from pyg_engine import Component, GameObject

class Animator(Component):
    def __init__(self, gameobject:GameObject):
        super().__init__(gameobject)

    def start():
        pass

    def update(engine):
        pass

    def on_destroy():
        pass
