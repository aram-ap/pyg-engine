"""
Sound Component - Audio playback component for GameObjects

Features:
    - Play sound effects on GameObjects
    - Automatic loading and caching
    - Volume and pitch control
    - Spatial audio support (distance-based volume)
    - Play on events (collision, trigger, etc.)
"""

from ..components.component import Component
from .audio_manager import audio_manager


class Sound(Component):
    """
    Sound component for playing audio on GameObjects.
    Can be triggered by events or called directly from scripts.
    """
    
    def __init__(self, game_object, sound_name=None, file_path=None, 
                 volume=1.0, loops=0, play_on_start=False, auto_load=True):
        """
        Initialize a Sound component.
        
        Args:
            game_object: The GameObject this sound is attached to
            sound_name: Unique identifier for this sound
            file_path: Path to the audio file
            volume: Playback volume (0.0 to 1.0)
            loops: Number of times to loop (0 = play once, -1 = infinite)
            play_on_start: Whether to play the sound when the component starts
            auto_load: Automatically load the sound if not already loaded
        """
        super().__init__(game_object)
        
        self.sound_name = sound_name or f"sound_{id(self)}"
        self.file_path = file_path
        self.volume = volume
        self.loops = loops
        self.play_on_start = play_on_start
        
        # Playback state
        self.channel = None
        self.is_playing = False
        
        # Auto-load the sound if path is provided
        if auto_load and file_path:
            self.load()
    
    def load(self):
        """Load the sound into the AudioManager."""
        if not self.file_path:
            print(f"Warning: No file path set for sound '{self.sound_name}'")
            return False
        
        sound = audio_manager.load_sound(self.sound_name, self.file_path)
        return sound is not None
    
    def play(self, volume=None, loops=None):
        """
        Play the sound.
        
        Args:
            volume: Override volume (uses self.volume if None)
            loops: Override loop count (uses self.loops if None)
            
        Returns:
            pygame.mixer.Channel object or None
        """
        play_volume = volume if volume is not None else self.volume
        play_loops = loops if loops is not None else self.loops
        
        self.channel = audio_manager.play_sound(self.sound_name, play_volume, play_loops)
        self.is_playing = True
        
        return self.channel
    
    def stop(self):
        """Stop the sound if it's currently playing."""
        audio_manager.stop_sound(self.sound_name)
        self.is_playing = False
        self.channel = None
    
    def set_volume(self, volume):
        """
        Set the playback volume.
        
        Args:
            volume: Volume level (0.0 to 1.0)
        """
        self.volume = max(0.0, min(1.0, volume))
    
    def set_loops(self, loops):
        """
        Set the loop count.
        
        Args:
            loops: Number of times to loop (0 = play once, -1 = infinite)
        """
        self.loops = loops
    
    def check_playing(self):
        """
        Check if the sound is still playing.
        Updates the is_playing flag.
        
        Returns:
            True if sound is playing
        """
        if self.channel:
            self.is_playing = self.channel.get_busy()
        else:
            self.is_playing = False
        return self.is_playing
    
    def start(self):
        """Called when the component starts."""
        if self.play_on_start:
            self.play()
    
    def update(self, engine):
        """Update the sound component (called every frame)."""
        # Update playing status
        self.check_playing()
    
    def on_destroy(self):
        """Clean up when the component is destroyed."""
        self.stop()


class SoundOneShot:
    """
    Utility class for playing one-shot sounds without a component.
    Useful for UI sounds, quick effects, etc.
    """
    
    @staticmethod
    def play(file_path, volume=1.0):
        """
        Play a one-shot sound effect.
        Automatically loads and plays the sound.
        
        Args:
            file_path: Path to the audio file
            volume: Playback volume (0.0 to 1.0)
        """
        # Generate a unique name
        sound_name = f"oneshot_{file_path}_{id(file_path)}"
        
        # Check if already loaded, if not load it
        if sound_name not in audio_manager.sounds:
            audio_manager.load_sound(sound_name, file_path)
        
        # Play the sound
        audio_manager.play_sound(sound_name, volume)

