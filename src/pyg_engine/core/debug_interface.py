"""
Debug interface for the game engine.
Provides thread-safe access to engine state for developer tools.
"""

import threading
from typing import List, Optional, Dict, Any
from .gameobject import GameObject


class DebugInterface:
    """Thread-safe interface for accessing engine state from external tools."""
    
    def __init__(self, engine):
        """Initialize debug interface with engine reference."""
        self._engine = engine
        self._lock = threading.RLock()
        self._paused = False
        self._step_requested = False
        
    def get_game_objects(self) -> List[GameObject]:
        """Get a thread-safe copy of all game objects."""
        with self._lock:
            return list(self._engine.getGameObjects())
    
    def get_fps(self) -> float:
        """Get current FPS."""
        with self._lock:
            return self._engine.clock.get_fps()
    
    def get_delta_time(self) -> float:
        """Get current delta time."""
        with self._lock:
            return self._engine.dt()
    
    def get_unscaled_delta_time(self) -> float:
        """Get unscaled delta time."""
        with self._lock:
            return self._engine.get_unscaled_dt()
    
    def get_time_scale(self) -> float:
        """Get current time scale."""
        with self._lock:
            return self._engine.time_scale
    
    def get_game_object_count(self) -> int:
        """Get count of game objects."""
        with self._lock:
            return len(self._engine.getGameObjects())
    
    def get_component_count(self) -> int:
        """Get total count of all components across all game objects."""
        with self._lock:
            count = 0
            for obj in self._engine.getGameObjects():
                count += len(obj.components)
            return count
    
    def get_physics_paused(self) -> bool:
        """Check if physics is paused."""
        with self._lock:
            return self._engine.physics_system.paused
    
    def is_running(self) -> bool:
        """Check if engine is running."""
        with self._lock:
            return self._engine.isRunning
    
    def is_paused(self) -> bool:
        """Check if game loop is paused."""
        with self._lock:
            return self._paused
    
    def pause(self):
        """Pause the game loop."""
        with self._lock:
            self._paused = True
    
    def resume(self):
        """Resume the game loop."""
        with self._lock:
            self._paused = False
            self._step_requested = False
    
    def toggle_pause(self):
        """Toggle pause state."""
        with self._lock:
            self._paused = not self._paused
            if not self._paused:
                self._step_requested = False
    
    def step_frame(self):
        """Request a single frame step (only works when paused)."""
        with self._lock:
            if self._paused:
                self._step_requested = True
    
    def should_update(self) -> bool:
        """Check if engine should update this frame (for pause/step logic)."""
        with self._lock:
            if not self._paused:
                return True
            if self._step_requested:
                self._step_requested = False
                return True
            return False
    
    def get_engine_stats(self) -> Dict[str, Any]:
        """Get comprehensive engine statistics."""
        with self._lock:
            return {
                'fps': self._engine.clock.get_fps(),
                'delta_time': self._engine.dt(),
                'unscaled_delta_time': self._engine.get_unscaled_dt(),
                'time_scale': self._engine.time_scale,
                'game_object_count': len(self._engine.getGameObjects()),
                'component_count': sum(len(obj.components) for obj in self._engine.getGameObjects()),
                'is_running': self._engine.isRunning,
                'is_paused': self._paused,
                'physics_paused': self._engine.physics_system.paused,
                'fps_cap': self._engine.fpsCap,
            }
    
    def find_game_object_by_name(self, name: str) -> Optional[GameObject]:
        """Find a game object by name (thread-safe)."""
        with self._lock:
            for obj in self._engine.getGameObjects():
                if obj.name == name:
                    return obj
            return None
    
    def find_game_object_by_id(self, obj_id: int) -> Optional[GameObject]:
        """Find a game object by ID (thread-safe)."""
        with self._lock:
            for obj in self._engine.getGameObjects():
                if obj.id == obj_id:
                    return obj
            return None
    
    def find_game_objects_by_tag(self, tag) -> List[GameObject]:
        """Find all game objects with a specific tag."""
        with self._lock:
            return [obj for obj in self._engine.getGameObjects() if obj.tag == tag]

    def get_ui_elements_info(self) -> List[Dict[str, Any]]:
        """
        Get information about registered UI canvases and elements.
        
        Returns a list of canvas descriptors:
        [
            {
                'index': int,
                'name': str,
                'canvas_ref': UICanvas,
                'element_count': int,
                'elements': [
                    {
                        'type': str,
                        'visible': bool,
                        'enabled': bool,
                        'layer': int,
                        'anchor': str,
                        'offset': Vector2,
                        'size': Vector2,
                        'details': str,
                        'ref': UIElement,
                    },
                    ...
                ]
            },
            ...
        ]
        """
        with self._lock:
            canvases_info: List[Dict[str, Any]] = []
            get_canvases = getattr(self._engine, 'get_ui_canvases', None)
            if callable(get_canvases):
                canvases = get_canvases()
                for index, canvas in enumerate(canvases, start=1):
                    elements_info: List[Dict[str, Any]] = []
                    for element in getattr(canvas, 'elements', []):
                        try:
                            info: Dict[str, Any] = {
                                'type': element.__class__.__name__,
                                'visible': getattr(element, 'visible', True),
                                'enabled': getattr(element, 'enabled', True),
                                'layer': getattr(element, 'layer', 0),
                                'anchor': getattr(getattr(element, 'anchor', None), 'name', str(getattr(element, 'anchor', ''))),
                                'offset': getattr(element, 'offset', None),
                                'size': getattr(element, 'size', None),
                                'details': getattr(element, 'text', None) or getattr(element, 'name', None) or "",
                                'ref': element,
                            }
                            elements_info.append(info)
                        except Exception:
                            # Skip problematic elements but continue collecting others
                            continue
                    canvases_info.append({
                        'index': index,
                        'name': getattr(canvas, 'name', None) or f"Canvas {index}",
                        'canvas_ref': canvas,
                        'element_count': len(elements_info),
                        'elements': elements_info,
                    })
            return canvases_info

