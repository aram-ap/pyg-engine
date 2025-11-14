'''
SpriteSheet - Utility for loading and managing sprite sheets

Features:
    - Load sprite sheets as single images
    - Extract individual frames by grid position
    - Extract frames by pixel coordinates
    - Automatic frame animation sequences
    - Support for different sprite sheet layouts
'''

import pygame
from ..utilities.vector2 import Vector2


class SpriteSheet:
    """
    Utility class for loading and extracting frames from sprite sheets.
    """
    
    def __init__(self, image_path, sprite_width=None, sprite_height=None):
        """
        Load a sprite sheet.
        
        Args:
            image_path: Path to the sprite sheet image
            sprite_width: Width of each sprite (if uniform grid)
            sprite_height: Height of each sprite (if uniform grid)
        """
        try:
            # Load and optimize the sprite sheet
            loaded_image = pygame.image.load(image_path)
            if loaded_image.get_alpha() is not None or loaded_image.get_flags() & pygame.SRCALPHA:
                self.sheet = loaded_image.convert_alpha()
            else:
                self.sheet = loaded_image.convert()
            
            self.sprite_width = sprite_width
            self.sprite_height = sprite_height
            
            # Calculate grid dimensions if sprite size is provided
            if sprite_width and sprite_height:
                self.columns = self.sheet.get_width() // sprite_width
                self.rows = self.sheet.get_height() // sprite_height
            else:
                self.columns = 1
                self.rows = 1
                
        except pygame.error as e:
            print(f"Error loading sprite sheet '{image_path}': {e}")
            # Create placeholder
            self.sheet = pygame.Surface((32, 32), pygame.SRCALPHA)
            self.sheet.fill((255, 0, 255))
            self.sprite_width = 32
            self.sprite_height = 32
            self.columns = 1
            self.rows = 1
    
    def get_frame(self, x, y, width=None, height=None):
        """
        Extract a single frame from the sprite sheet by pixel coordinates.
        
        Args:
            x: X pixel coordinate
            y: Y pixel coordinate
            width: Width of the frame (uses sprite_width if None)
            height: Height of the frame (uses sprite_height if None)
            
        Returns:
            pygame.Surface containing the extracted frame
        """
        width = width or self.sprite_width
        height = height or self.sprite_height
        
        if width is None or height is None:
            print("Error: Frame dimensions not specified")
            return pygame.Surface((32, 32), pygame.SRCALPHA)
        
        # Create a new surface for the frame
        frame = pygame.Surface((width, height), pygame.SRCALPHA)
        frame.blit(self.sheet, (0, 0), (x, y, width, height))
        
        # Optimize the frame
        if frame.get_alpha() is not None or frame.get_flags() & pygame.SRCALPHA:
            frame = frame.convert_alpha()
        else:
            frame = frame.convert()
        
        return frame
    
    def get_frame_at_index(self, index):
        """
        Extract a frame by its index in a uniform grid.
        
        Args:
            index: Frame index (0-based, left-to-right, top-to-bottom)
            
        Returns:
            pygame.Surface containing the extracted frame
        """
        if not self.sprite_width or not self.sprite_height:
            print("Error: Sprite dimensions not set for indexed access")
            return pygame.Surface((32, 32), pygame.SRCALPHA)
        
        # Calculate row and column from index
        col = index % self.columns
        row = index // self.columns
        
        return self.get_frame_at_grid(col, row)
    
    def get_frame_at_grid(self, col, row):
        """
        Extract a frame by its grid position.
        
        Args:
            col: Column index (0-based)
            row: Row index (0-based)
            
        Returns:
            pygame.Surface containing the extracted frame
        """
        if not self.sprite_width or not self.sprite_height:
            print("Error: Sprite dimensions not set for grid access")
            return pygame.Surface((32, 32), pygame.SRCALPHA)
        
        x = col * self.sprite_width
        y = row * self.sprite_height
        
        return self.get_frame(x, y, self.sprite_width, self.sprite_height)
    
    def get_frames_range(self, start_index, end_index):
        """
        Extract a range of frames from the sprite sheet.
        
        Args:
            start_index: Starting frame index (inclusive)
            end_index: Ending frame index (inclusive)
            
        Returns:
            List of pygame.Surface objects
        """
        frames = []
        for i in range(start_index, end_index + 1):
            frames.append(self.get_frame_at_index(i))
        return frames
    
    def get_row(self, row_index):
        """
        Extract all frames from a specific row.
        
        Args:
            row_index: Row index (0-based)
            
        Returns:
            List of pygame.Surface objects
        """
        frames = []
        for col in range(self.columns):
            frames.append(self.get_frame_at_grid(col, row_index))
        return frames
    
    def get_column(self, col_index):
        """
        Extract all frames from a specific column.
        
        Args:
            col_index: Column index (0-based)
            
        Returns:
            List of pygame.Surface objects
        """
        frames = []
        for row in range(self.rows):
            frames.append(self.get_frame_at_grid(col_index, row))
        return frames
    
    def get_all_frames(self):
        """
        Extract all frames from the sprite sheet.
        
        Returns:
            List of all frames as pygame.Surface objects
        """
        frames = []
        for row in range(self.rows):
            for col in range(self.columns):
                frames.append(self.get_frame_at_grid(col, row))
        return frames


def load_animation_frames(image_paths):
    """
    Load a list of individual images as animation frames.
    
    Args:
        image_paths: List of paths to image files
        
    Returns:
        List of pygame.Surface objects
    """
    frames = []
    for path in image_paths:
        try:
            loaded_image = pygame.image.load(path)
            if loaded_image.get_alpha() is not None or loaded_image.get_flags() & pygame.SRCALPHA:
                frame = loaded_image.convert_alpha()
            else:
                frame = loaded_image.convert()
            frames.append(frame)
        except pygame.error as e:
            print(f"Error loading animation frame '{path}': {e}")
            # Create placeholder
            placeholder = pygame.Surface((32, 32), pygame.SRCALPHA)
            placeholder.fill((255, 0, 255))
            frames.append(placeholder)
    
    return frames

