"""
AudioManager - Global audio system for sound effects and music

Features:
    - Global sound effect management
    - Music playback with volume control
    - Sound pooling for performance
    - Volume control (master, music, sfx)
    - Fade in/out support
"""

import pygame
from pathlib import Path


class AudioManager:
    """
    Global audio manager singleton for handling all sound effects and music.
    This class manages the pygame mixer and provides convenient methods for audio playback.
    """
    
    _instance = None
    
    def __new__(cls):
        """Implement singleton pattern."""
        if cls._instance is None:
            cls._instance = super(AudioManager, cls).__new__(cls)
            cls._instance._initialized = False
        return cls._instance
    
    def __init__(self):
        """Initialize the audio manager."""
        if self._initialized:
            return
        
        # Initialize pygame mixer
        pygame.mixer.init(frequency=44100, size=-16, channels=2, buffer=512)
        
        # Volume settings (0.0 to 1.0)
        self.master_volume = 1.0
        self.music_volume = 1.0
        self.sfx_volume = 1.0
        
        # Sound effect library
        self.sounds = {}  # name -> pygame.mixer.Sound
        
        # Music state
        self.current_music = None
        self.music_paused = False
        
        self._initialized = True
        print("AudioManager initialized")
    
    # ==================== Sound Effect Methods ====================
    
    def load_sound(self, name, file_path):
        """
        Load a sound effect into memory.
        
        Args:
            name: Unique identifier for the sound
            file_path: Path to the audio file (.wav, .ogg, etc.)
            
        Returns:
            pygame.mixer.Sound object or None on error
        """
        try:
            sound = pygame.mixer.Sound(file_path)
            self.sounds[name] = sound
            print(f"Loaded sound '{name}' from '{file_path}'")
            return sound
        except pygame.error as e:
            print(f"Error loading sound '{name}' from '{file_path}': {e}")
            return None
    
    def play_sound(self, name, volume=1.0, loops=0):
        """
        Play a loaded sound effect.
        
        Args:
            name: Name of the sound to play
            volume: Volume multiplier (0.0 to 1.0)
            loops: Number of times to loop (-1 for infinite)
            
        Returns:
            pygame.mixer.Channel object or None
        """
        if name not in self.sounds:
            print(f"Warning: Sound '{name}' not loaded")
            return None
        
        sound = self.sounds[name]
        
        # Set volume (consider master and sfx volume)
        final_volume = volume * self.sfx_volume * self.master_volume
        sound.set_volume(final_volume)
        
        # Play the sound
        channel = sound.play(loops=loops)
        return channel
    
    def stop_sound(self, name):
        """
        Stop a playing sound effect.
        
        Args:
            name: Name of the sound to stop
        """
        if name in self.sounds:
            self.sounds[name].stop()
    
    def unload_sound(self, name):
        """
        Unload a sound effect from memory.
        
        Args:
            name: Name of the sound to unload
        """
        if name in self.sounds:
            self.sounds[name].stop()
            del self.sounds[name]
            print(f"Unloaded sound '{name}'")
    
    # ==================== Music Methods ====================
    
    def load_music(self, file_path):
        """
        Load a music file for playback.
        Note: Only one music track can be loaded at a time.
        
        Args:
            file_path: Path to the music file (.mp3, .ogg, etc.)
        """
        try:
            pygame.mixer.music.load(file_path)
            self.current_music = file_path
            print(f"Loaded music from '{file_path}'")
        except pygame.error as e:
            print(f"Error loading music from '{file_path}': {e}")
    
    def play_music(self, file_path=None, volume=1.0, loops=-1, fade_ms=0):
        """
        Play music.
        
        Args:
            file_path: Path to music file (uses currently loaded if None)
            volume: Volume level (0.0 to 1.0)
            loops: Number of times to loop (-1 for infinite)
            fade_ms: Fade in time in milliseconds
        """
        if file_path:
            self.load_music(file_path)
        
        if self.current_music is None:
            print("Warning: No music loaded")
            return
        
        # Set volume
        final_volume = volume * self.music_volume * self.master_volume
        pygame.mixer.music.set_volume(final_volume)
        
        # Play music
        if fade_ms > 0:
            pygame.mixer.music.play(loops=loops, fade_ms=fade_ms)
        else:
            pygame.mixer.music.play(loops=loops)
        
        self.music_paused = False
    
    def stop_music(self, fade_ms=0):
        """
        Stop the currently playing music.
        
        Args:
            fade_ms: Fade out time in milliseconds
        """
        if fade_ms > 0:
            pygame.mixer.music.fadeout(fade_ms)
        else:
            pygame.mixer.music.stop()
        self.music_paused = False
    
    def pause_music(self):
        """Pause the currently playing music."""
        pygame.mixer.music.pause()
        self.music_paused = True
    
    def unpause_music(self):
        """Resume paused music."""
        pygame.mixer.music.unpause()
        self.music_paused = False
    
    def is_music_playing(self):
        """Check if music is currently playing."""
        return pygame.mixer.music.get_busy() and not self.music_paused
    
    # ==================== Volume Control Methods ====================
    
    def set_master_volume(self, volume):
        """
        Set the master volume level.
        
        Args:
            volume: Volume level (0.0 to 1.0)
        """
        self.master_volume = max(0.0, min(1.0, volume))
        
        # Update music volume
        if self.current_music:
            final_volume = self.music_volume * self.master_volume
            pygame.mixer.music.set_volume(final_volume)
    
    def set_music_volume(self, volume):
        """
        Set the music volume level.
        
        Args:
            volume: Volume level (0.0 to 1.0)
        """
        self.music_volume = max(0.0, min(1.0, volume))
        
        # Update current music volume
        if self.current_music:
            final_volume = self.music_volume * self.master_volume
            pygame.mixer.music.set_volume(final_volume)
    
    def set_sfx_volume(self, volume):
        """
        Set the sound effects volume level.
        
        Args:
            volume: Volume level (0.0 to 1.0)
        """
        self.sfx_volume = max(0.0, min(1.0, volume))
    
    def get_master_volume(self):
        """Get the master volume level."""
        return self.master_volume
    
    def get_music_volume(self):
        """Get the music volume level."""
        return self.music_volume
    
    def get_sfx_volume(self):
        """Get the sound effects volume level."""
        return self.sfx_volume
    
    # ==================== Utility Methods ====================
    
    def stop_all(self):
        """Stop all sounds and music."""
        pygame.mixer.stop()
        pygame.mixer.music.stop()
    
    def cleanup(self):
        """Clean up and quit the audio system."""
        self.stop_all()
        self.sounds.clear()
        pygame.mixer.quit()
        print("AudioManager cleaned up")


# Create a global instance
audio_manager = AudioManager()

