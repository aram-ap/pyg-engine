'''
Sprite Component Features:
    Scaling and offset support
    Image loading from file paths
    Rotation with proper image handling
    Color tinting and alpha blending
    Flip horizontal/vertical
    Culling for performance
'''
'''
Example usage:
    # Create sprite-based game object
    player = GameObject(name="Player", position=Vector2(100, 100))
    player.add_component(Sprite, image_path="assets/player.png", scale=Vector2(2, 2))
    player.add_component(SpriteCollider, collision_type="pixel_perfect")
    player.add_component(Animator)

# Set up animations
    animator = player.get_component(Animator)
    animator.add_animation("idle", idle_frames, frame_duration=0.2)
    animator.add_animation("walk", walk_frames, frame_duration=0.1)
    animator.play("idle")
'''
import pygame
from pygame import sprite, Vector2
from pyg_engine import Component


class SpriteRenderer(Component):

    def __init__(self, gameobject:GameObject, image_path:str="",
                 scale: Vector2 | tuple[float,float] = (0,0)):
        super().__init__(gameobject)
        self.image_path = image_path
        self.scale = scale

       try:
          image = pygame.image.load






