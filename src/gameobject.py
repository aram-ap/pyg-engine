import pygame as pg
from pygame import Color, Vector2
import time
import importlib.util
import sys

from .object_types import BasicShape, Tag
from .script import Script
from .component import Component

class GameObject(pg.sprite.Sprite):
    """Game object with component and script attachment capabilities."""
    
    def __init__(self, name: str, id: int = time.time_ns(), enabled: bool = True, position: Vector2 = Vector2(),
                 size: Vector2 = Vector2(), rotation: float = 0.0, color: Color = Color(255, 255, 255, 255),
                 tag: Tag = Tag.Other, basicShape: BasicShape = BasicShape.Rectangle, script_configs=None,
                 show_rotation_line: bool = False):
        # Initialize pygame sprite
        super().__init__()
        
        self.name = name
        self.id = id
        self.enabled = enabled
        self.position = position
        self.size = size
        self.rotation = rotation
        self.basicShape = basicShape
        self.color = color
        self.tag = tag
        self.show_rotation_line = show_rotation_line

        # Component system
        self.components = {}  # Dictionary to store components by type
        self._components_started = False

        # Script system
        self.scripts = []
        self._scripts_started = False

        # Create the sprite image and rect
        self._create_sprite_surface()
        
        if script_configs:
            self.add_script_configs(script_configs)

    def _create_sprite_surface(self):
        """Create pygame surface and rect for the sprite."""
        # Create surface based on shape and size
        if self.basicShape == BasicShape.Circle:
            radius = max(1, int(max(self.size.x, self.size.y) / 2)) if self.size.x > 0 or self.size.y > 0 else 20
            diameter = radius * 2
            self.image = pg.Surface((diameter, diameter), pg.SRCALPHA)
            pg.draw.circle(self.image, self.color, (radius, radius), radius)
        else:  # Rectangle
            width = max(1, int(self.size.x)) if self.size.x > 0 else 40
            height = max(1, int(self.size.y)) if self.size.y > 0 else 40
            self.image = pg.Surface((width, height), pg.SRCALPHA)
            self.image.fill(self.color)
        
        # Create rect and set position
        self.rect = self.image.get_rect()
        self.rect.center = (int(self.position.x), int(self.position.y))
        
        # Apply rotation if needed
        if abs(self.rotation) > 0.1:
            self._apply_rotation()

    def _apply_rotation(self):
        """Apply rotation to the sprite image."""
        if hasattr(self, 'image') and self.image:
            # Store original image for rotation
            if not hasattr(self, '_original_image'):
                self._original_image = self.image.copy()
            else:
                self.image = self._original_image.copy()
            
            # Rotate the image
            rotated_image = pg.transform.rotate(self.image, self.rotation)
            self.image = rotated_image
            self.rect = self.image.get_rect()
            self.rect.center = (int(self.position.x), int(self.position.y))

    def update_position(self, new_position: Vector2):
        """Update sprite position and rect."""
        self.position = new_position
        if hasattr(self, 'rect'):
            self.rect.center = (int(self.position.x), int(self.position.y))

    def update_size(self, new_size: Vector2):
        """Update sprite size and recreate surface."""
        self.size = new_size
        self._create_sprite_surface()

    def update_rotation(self, new_rotation: float):
        """Update sprite rotation."""
        self.rotation = new_rotation
        self._apply_rotation()

    def update_color(self, new_color: Color):
        """Update sprite color and recreate surface."""
        self.color = new_color
        self._create_sprite_surface()

    # ================ Component System Methods ================

    def add_component(self, component_class, **kwargs):
        """Add a component to this GameObject."""
        if component_class in self.components:
            print(f"Warning: {component_class.__name__} already exists on {self.name}")
            return self.components[component_class]

        try:
            component = component_class(self, **kwargs)
            self.components[component_class] = component
            print(f"Added {component_class.__name__} to {self.name}")
            return component
        except Exception as e:
            print(f"Failed to add component {component_class.__name__} to {self.name}: {e}")
            return None

    def get_component(self, component_class):
        """Get a component of the specified type, supporting inheritance."""
        # First try exact match
        if component_class in self.components:
            return self.components[component_class]

        # Then try inheritance - find any component that is an instance of component_class
        for stored_class, component_instance in self.components.items():
            if isinstance(component_instance, component_class):
                return component_instance

        return None

    def has_component(self, component_class):
        """Check if this GameObject has a component of the specified type, supporting inheritance."""
        return self.get_component(component_class) is not None

    def get_components(self, component_class):
        """Get all components of the specified type (supporting inheritance)."""
        matching_components = []
        for stored_class, component_instance in self.components.items():
            if isinstance(component_instance, component_class):
                matching_components.append(component_instance)
        return matching_components

    def remove_component(self, component_class):
        """Remove a component from this GameObject."""
        if component_class in self.components:
            component = self.components[component_class]
            try:
                component.on_destroy()
            except Exception as e:
                print(f"Error destroying component {component_class.__name__}: {e}")
            del self.components[component_class]
            print(f"Removed {component_class.__name__} from {self.name}")
            return True
        return False

    def get_all_components(self):
        """Get all components attached to this GameObject."""
        return list(self.components.values())

    # ================ Script System Methods ================

    def add_script(self, script_path: str, script_class_name: str = None, **kwargs):
        """Dynamically import and add a script from a file path with optional parameters."""
        try:
            spec = importlib.util.spec_from_file_location("script_module", script_path)
            module = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(module)

            if script_class_name is None:
                for attr_name in dir(module):
                    attr = getattr(module, attr_name)
                    if (isinstance(attr, type) and
                        issubclass(attr, Script) and
                        attr != Script):
                        script_class_name = attr_name
                        break

                if script_class_name is None:
                    raise ValueError(f"No Script subclass found in {script_path}")

            script_class = getattr(module, script_class_name)
            if not issubclass(script_class, Script):
                raise ValueError(f"Class {script_class_name} in {script_path} must inherit from Script")

            script_instance = script_class(self, **kwargs)
            self.scripts.append(script_instance)
            print(f"Added script '{script_class_name}' from {script_path} to {self.name}")
            return script_instance
        except Exception as e:
            print(f"Failed to add script from {script_path}: {e}")
            return None

    def add_script_configs(self, script_configs):
        """Add multiple scripts with their configurations."""
        for config in script_configs:
            if isinstance(config, dict) and 'path' in config:
                path = config.pop('path')
                class_name = config.pop('class_name', None)
                self.add_script(path, class_name, **config)
            elif isinstance(config, str):
                self.add_script(config)

    def get_script(self, script_class_type):
        """Get the first script instance of a specific type."""
        for script in self.scripts:
            if isinstance(script, script_class_type):
                return script
        return None

    def get_scripts(self, script_class_type):
        """Get all script instances of a specific type."""
        return [script for script in self.scripts if isinstance(script, script_class_type)]

    def configure_script(self, script_class_type, **kwargs):
        """Configure a script at runtime."""
        script = self.get_script(script_class_type)
        if script:
            for key, value in kwargs.items():
                script.set_config(key, value)
            return True
        return False

    # ================ Update Methods ================

    def update(self, engine):
        """Update all scripts and components."""
        if not self.enabled:
            return

        # Start components on first update
        if not self._components_started:
            for component in self.components.values():
                if component.enabled:
                    try:
                        component.start()
                    except Exception as e:
                        print(f"Error starting component {component.__class__.__name__} on {self.name}: {e}")
            self._components_started = True

        # Update all enabled components
        for component in self.components.values():
            if component.enabled:
                try:
                    component.update(engine)
                except Exception as e:
                    print(f"Error updating component {component.__class__.__name__} on {self.name}: {e}")

        # Update all scripts
        for script in self.scripts:
            try:
                script.update(engine)
            except Exception as e:
                print(f"Error updating script on {self.name}: {e}")

    # ================ Debug and Utility Methods ================

    def list_components(self):
        """Print all components attached to this GameObject."""
        print(f"Components on {self.name}:")
        for component_class, component in self.components.items():
            print(f"  - {component_class.__name__}: {component}")

    def list_scripts(self):
        """Print all scripts attached to this GameObject."""
        print(f"Scripts on {self.name}:")
        for i, script in enumerate(self.scripts):
            print(f"  - {i}: {script.__class__.__name__}")

    def kill(self):
        """Pygame sprite kill method - calls destroy and removes from groups."""
        self.destroy()
        super().kill()

    def destroy(self):
        """Clean up; call on_destroy for all scripts and components."""
        # Destroy all components
        for component in list(self.components.values()):  # Create a copy to avoid dict change during iteration
            try:
                component.on_destroy()
            except Exception as e:
                print(f"Error destroying component on {self.name}: {e}")

        # Destroy all scripts
        for script in self.scripts:
            try:
                script.on_destroy()
            except Exception as e:
                print(f"Error destroying script on {self.name}: {e}")

        self.components = {}
        self.scripts = []
        print(f"Destroyed {self.name if self.name else self.id}")

    def __repr__(self):
        component_names = [cls.__name__ for cls in self.components.keys()]
        script_count = len(self.scripts)
        return f"GameObject(name='{self.name}', components={component_names}, scripts={script_count})"


# Backward compatibility - alias BasicObject to GameObject
BasicObject = GameObject

