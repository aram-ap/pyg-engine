import time  # For timestamp
from dataclasses import dataclass, field  # Import dataclasses here

@dataclass(frozen=True)
class Event:
    """
    Immutable data class representing an event in the game engine.
    
    Events are used for communication between different components of the engine.
    The frozen=True ensures thread safety and prevents accidental modification.
    """
    type: str          # Event type identifier (e.g., 'collision', 'input', 'spawn')
    data: dict = None  # Optional payload containing event-specific data
    timestamp: float = field(default_factory=lambda: time.time())  # Creation time for event ordering

