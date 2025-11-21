"""
Audio System - Sound effects and music management

This module provides:
    - AudioManager: Global audio system for managing sounds and music
    - Sound: Component for playing sound effects on GameObjects
"""

from .audio_manager import AudioManager
from .sound import Sound

__all__ = ['AudioManager', 'Sound']

