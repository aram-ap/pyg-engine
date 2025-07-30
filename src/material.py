class PhysicsMaterial:
    """Defines physical properties for collision behavior."""

    def __init__(self, name="Default", bounce=0.0, friction=0.5, friction_combine="average"):
        self.name = name
        self.bounce = bounce  # Restitution coefficient (0.0 = no bounce, 1.0 = perfect bounce)
        self.friction = friction  # Surface friction (0.0 = ice, 1.0 = rubber)
        self.friction_combine = friction_combine  # Friction combination method: "average", "min", "max"

    def __repr__(self):
        return f"PhysicsMaterial({self.name}, bounce={self.bounce}, friction={self.friction})"

# Predefined materials for common use cases
class Materials:
    """Collection of predefined physics materials."""
    DEFAULT = PhysicsMaterial("Default", bounce=0.0, friction=0.5)
    BOUNCY = PhysicsMaterial("Bouncy", bounce=0.9, friction=0.1)
    ICE = PhysicsMaterial("Ice", bounce=0.1, friction=0.05)
    RUBBER = PhysicsMaterial("Rubber", bounce=0.9, friction=0.8)
    METAL = PhysicsMaterial("Metal", bounce=0.3, friction=0.4)
    WOOD = PhysicsMaterial("Wood", bounce=0.2, friction=0.7)

