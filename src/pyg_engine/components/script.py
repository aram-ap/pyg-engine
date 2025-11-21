import pygame as pg
from ..utilities.vector2 import Vector2
from ..utilities.color import Color

class Script:
    """Base class for game scripts that can be attached to GameObjects."""

    def __init__(self, game_object, **kwargs):
        self.game_object = game_object  # Reference to the attached GameObject
        self._started = False
        self.enabled = True  # Scripts can be enabled/disabled like components
        self.config = kwargs  # Configuration parameters

        # Apply configuration to script properties
        self._apply_config()

    def _apply_config(self):
        """Override this method in subclasses to handle configuration parameters."""
        pass

    def get_config(self, key, default=None):
        """Get a configuration value with optional default."""
        return self.config.get(key, default)

    def set_config(self, key, value):
        """Set a configuration value at runtime."""
        self.config[key] = value
        self._apply_config()

    # ================ Component Access Helpers ================

    def get_component(self, component_class):
        """Get a component from the attached GameObject."""
        return self.game_object.get_component(component_class)

    def has_component(self, component_class):
        """Check if GameObject has a component."""
        return self.game_object.has_component(component_class)

    def require_component(self, component_class):
        """Get a component and raise an error if it doesn't exist."""
        component = self.get_component(component_class)
        if component is None:
            raise RuntimeError(f"Script {self.__class__.__name__} requires component {component_class.__name__} on {self.game_object.name}")
        return component

    # Convenience methods for common physics components
    def get_rigidbody(self):
        """Get the RigidBody component."""
        from ..physics.rigidbody import RigidBody
        return self.get_component(RigidBody)

    def get_collider(self):
        """Get any Collider component."""
        from ..physics.collider import Collider
        return self.get_component(Collider)

    # ================ Lifecycle Methods ================

    def start(self, engine):
        """Called once when the script is first initialized."""
        if self._started:
            return
        self.__started = True

    def update(self, engine):
        """Called every frame in the game loop. Engine reference provided for input access."""
        if not self._started:
            self.start(engine)
            self._started = True

    def on_destroy(self):
        """Called when the game object is destroyed."""
        pass

