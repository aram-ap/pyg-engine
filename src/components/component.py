class Component:
    """Base class for all components that can be attached to GameObjects."""

    def __init__(self, game_object):
        self.game_object = game_object
        self.enabled = True

    def start(self):
        """Called once when the component is first added or the game starts."""
        pass

    def update(self, engine):
        """Called every frame if the component is enabled."""
        pass

    def on_destroy(self):
        """Called when the component or its GameObject is destroyed."""
        pass

    def __repr__(self):
        return f"{self.__class__.__name__}(enabled={self.enabled})"

