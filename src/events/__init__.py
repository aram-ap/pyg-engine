"""
Event management system for handling game events and communication between components.
"""

from .event_manager import EventManager
from .event import Event

__all__ = [
    'EventManager',
    'Event'
] 