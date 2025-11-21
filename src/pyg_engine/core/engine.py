import io
import pkgutil
import os
from importlib.resources import files, as_file

import pygame as pg
from pygame import RESIZABLE, Color
from collections import OrderedDict, defaultdict, deque
import weakref
import threading
from dataclasses import dataclass, field
import time
from ..utilities.object_types import Size, BasicShape, Tag
from .gameobject import GameObject
from ..physics.physics_system import PhysicsSystem
from ..rendering.camera import Camera
from .runnable import RunnableSystem, Priority
from ..input.input import Input
from ..physics.rigidbody import RigidBody
from ..events.event_manager import EventManager
from .debug_interface import DebugInterface


def configure_headless_mode():
    """
    Configure the engine to run in headless mode (no display window).
    
    This MUST be called BEFORE creating any Engine instances or importing pygame modules.
    It sets the SDL_VIDEODRIVER environment variable to 'dummy' which tells SDL
    not to create any display windows.
    
    Example:
        from src.core.engine import Engine, configure_headless_mode
        
        # Call this first for headless mode
        configure_headless_mode()
        
        # Now create engine
        engine = Engine(useDisplay=False)
    """
    os.environ['SDL_VIDEODRIVER'] = 'dummy'
    print("Headless mode configured - SDL will use dummy video driver")


class Engine:
    """Core game engine that handles the main loop, rendering, and system coordination."""

    log_debug = False

    def __init__(self, size: Size = Size(w=800, h=600),
                 backgroundColor: Color = Color(0,0,0),
                 running = False,
                 windowName = "PyGame", displaySize = True, fpsCap = 60, useDisplay = True,
                 displayMode = pg.RESIZABLE, icon = None
                 ):
        Engine.__debug_log("Initializing Engine")
        self.isRunning:bool = running
        self.fpsCap:int = fpsCap
        self.__size = size
        self.__dt = 0.0
        self.__useDisplay = useDisplay
        self.time_scale = 1.0  # Time scaling for ML/training (1.0 = normal speed)

        # Global dictionary system
        self.globals = GlobalDictionary()

        # Runnable system
        self.runnable_system = RunnableSystem()

        # Input system
        self.input = Input(self)

        # Window setup
        self.__displaySizeInTitle:bool = displaySize
        self.__windowName:str = "{} {}".format(windowName,
                                               str(self.__size) if displaySize else "")
        self.background_color = backgroundColor

        # Sprite groups for efficient rendering
        self.__gameobjects = pg.sprite.Group()
        self.__all_sprites = pg.sprite.Group()

        # Core systems initialization
        self.physics_system = PhysicsSystem(self)

        # Event system initialization
        self.event_manager = EventManager()

        # Debug interface for developer tools
        self.debug_interface = DebugInterface(self)

        # UI canvases registry for debug tools
        self._ui_canvases = []

        # Display mode (e.g. pygame.FULLSCREEN, RESIZABLE, SCALED, SHOWN, ETC)
        self.displayMode = displayMode

        # Initialize pygame
        pg.init()
        
        # Create clock
        self.clock = pg.time.Clock()

        # Create camera
        self.camera = Camera(self.__size.w, self.__size.h)
        self.camera.scale_mode = "fit"  # Options: "fit", "fill", "stretch", "fixed"

        # Setup display (with or without window)
        if useDisplay:
            # Handle icon setup (only in display mode, after pg.init())
            if icon is None:
                try:
                    data = pkgutil.get_data("pyg_engine", "etc/pyg_logo_transparent.png")
                except Exception as exc:
                    data = None
                    Engine.__debug_log(f"Failed to access default icon resource: {exc}")

                if data:
                    try:
                        icon_surface = pg.image.load(io.BytesIO(data), "pyg_logo_transparent.png")
                        pg.display.set_icon(icon_surface)
                    except Exception as exc:
                        Engine.__debug_log(f"Failed to load default icon surface: {exc}")
                else:
                    Engine.__debug_log("Default icon resource not available; using pygame default icon.")
            else:
                pg.display.set_icon(icon)
            
            # Create display window
            self.screen = pg.display.set_mode((self.__size.w, self.__size.h), displayMode)
        else:
            # Create a dummy surface for headless mode (for compatibility)
            self.screen = pg.Surface((self.__size.w, self.__size.h))
            Engine.__debug_log("Dummy surface created for headless mode")

        if(running):
            self.start()

    @staticmethod
    def __debug_log(msg: str):
        """Output debug messages when debug mode is enabled."""
        if(Engine.log_debug):
            print(msg)

    def getWindowSize(self) -> Size:
        """Return current window dimensions."""
        return Size(self.__size.w, self.__size.h)

    def setWindowSize(self,  size: Size):
        """Resize window to specified dimensions."""
        # Validate input dimensions
        if(size.w <= 0 or size.h <= 0):
            Engine.__debug_log(str(size) + " is an invalid window size argument!")
            return

        if(size.w > 0 and size.h > 0):
            self.__size = size
        if self.__useDisplay:
            self.screen = pg.display.set_mode((self.__size.w, self.__size.h), flags=self.displayMode, vsync=True)
        else:
            # Update dummy surface in headless mode
            self.screen = pg.Surface((self.__size.w, self.__size.h))

    def stop(self):
        """Stop the game engine."""
        self.isRunning = False

        # Stop all gameobjects
        for obj in self.__gameobjects:
            if type(obj) is not GameObject:
                continue

            obj.destroy

        Engine.__debug_log("Engine Stopped")

    def setRunning(self, running:bool):
        """Set running state."""
        if(self.isRunning != running):
            self.isRunning = running
            Engine.__debug_log("Engine set to " + ("running" if running else "NOT running"))

    def running(self)->bool:
        """Check if engine is running."""
        return self.isRunning

    # ================ Event System Integration ====================

    def subscribe(self, event_type: str, listener: callable, priority: Priority = Priority.NORMAL):
        """Subscribe a listener to an event type via the event manager."""
        self.event_manager.subscribe(event_type, listener, priority)

    def unsubscribe(self, event_type: str, listener: callable):
        """Unsubscribe a listener from an event type via the event manager."""
        self.event_manager.unsubscribe(event_type, listener)

    def dispatch_event(self, event_type: str, data: dict = None, immediate: bool = False):
        """Dispatch an event via the event manager."""
        self.event_manager.dispatch(event_type, data, immediate)


    # ================ Runnable System Integration ====================

    def add_runnable(self, func, event_type="update", priority=Priority.NORMAL,
                    max_runs=None, key=None, error_handler=None):
        """Add a runnable function to the engine."""
        self.runnable_system.add_runnable(func, event_type, priority, max_runs, key, error_handler)

    def add_error_handler(self, handler):
        """Add global error handler for runnables."""
        self.runnable_system.add_error_handler(handler)

    def set_debug_mode(self, enabled: bool):
        """Enable/disable debug mode for stricter error handling."""
        self.runnable_system.set_debug_mode(enabled)

    def get_runnable_stats(self):
        """Get statistics about runnable queues."""
        return self.runnable_system.get_queue_stats()

    def clear_runnable_queue(self, event_type: str, key=None):
        """Clear all runnables from a specific queue."""
        self.runnable_system.clear_queue(event_type, key)

    # ================ Game Objects ====================
    def addGameObject(self, gameobj: GameObject):
        """Add game object to the engine."""
        if(gameobj is not None):
            self.__gameobjects.add(gameobj)
            self.__all_sprites.add(gameobj)
            Engine.__debug_log("Added gameobject '{}'".format(gameobj.name if gameobj.name != "" else gameobj.id))

    def removeGameObject(self, gameobj: GameObject):
        """Remove game object from the engine."""
        if gameobj in self.__gameobjects:
            # Remove from physics system first
            self.physics_system.remove_object(gameobj)

            # Remove from engine lists
            self.__gameobjects.remove(gameobj)
            self.__all_sprites.remove(gameobj)
            gameobj.destroy()
            Engine.__debug_log("Removed gameobject '{}'".format(gameobj.name if gameobj.name != "" else gameobj.id))

    def getGameObjects(self):
        """Return all game objects as a list."""
        return list(self.__gameobjects)

    # ================ UI Canvas Management ====================
    def register_ui_canvas(self, canvas):
        """Register a UI canvas for debug tooling."""
        if canvas and canvas not in self._ui_canvases:
            self._ui_canvases.append(canvas)

    def unregister_ui_canvas(self, canvas):
        """Unregister a UI canvas."""
        if canvas in self._ui_canvases:
            self._ui_canvases.remove(canvas)

    def get_ui_canvases(self):
        """Get all registered UI canvases."""
        return list(self._ui_canvases)

    # ================= Physics Stuff ======================
    def dt(self)->float:
        """Return delta time (time since last frame), scaled by time_scale."""
        return self.__dt * self.time_scale
    
    def get_unscaled_dt(self)->float:
        """Return unscaled delta time (real time since last frame)."""
        return self.__dt
    
    def set_time_scale(self, scale: float):
        """
        Set the time scale for the simulation.
        Useful for ML training, slow-motion effects, or fast-forwarding.
        
        Args:
            scale: Time scale multiplier (1.0 = normal speed, 2.0 = double speed, 0.5 = half speed)
                   Must be positive. Higher values allow faster-than-realtime training.
        
        Raises:
            ValueError: If scale is not positive
        """
        if scale <= 0:
            raise ValueError(f"Time scale must be positive, got {scale}")
        self.time_scale = scale
        Engine.__debug_log(f"Time scale set to {scale}x")
    
    def get_time_scale(self) -> float:
        """Get the current time scale multiplier."""
        return self.time_scale

    # ================= Display Stuff ======================
    def setWindowTitle(self, title: str):
        """Set window title."""
        self.__windowName = title
        new_title = title + " (%s,%s)" % (self.__size.w, self.__size.h) if self.__displaySizeInTitle else title
        pg.display.set_caption(new_title)
        Engine.__debug_log("Changed window title to: '{}'".format(new_title))

    def __handleResize(self, event: pg.event.Event):
        """Handle window resize events."""
        self.__size = Size(w=event.w, h=event.h)
        if self.__useDisplay:
            self.screen = pg.display.set_mode((self.__size.w, self.__size.h), self.displayMode)
        else:
            # Update dummy surface in headless mode
            self.screen = pg.Surface((self.__size.w, self.__size.h))
        self.camera.resize(event.w, event.h)
        Engine.__debug_log("Handling Resize to {}".format(self.__size))

    # ================= Game Loop ======================

    def start(self):
        """Start the main game loop."""
        Engine.__debug_log("Starting Engine")
        self.isRunning = True

        # Execute start runnables
        self.runnable_system.execute_runnables('start', engine=self)

        # Process all gameobjects
        for gmo in self.__gameobjects:

            if(type(gmo) is not GameObject):
                continue

            # Start all gameobjects and their scripts
            gmo.start(self)

        # Start Update Loop.
        # TODO: Make this async and add notification system
        while(self.isRunning):
            # Check if we should update (handles pause/step logic)
            if self.debug_interface.should_update():
                self.__update()
            else:
                # When paused, still process events to keep UI responsive
                self.__processEvents()
                # Small sleep to avoid busy-waiting
                import time
                time.sleep(0.01)

    def __processEvents(self):
        """Process pygame events."""
        for event in pg.event.get():
            if event.type == pg.QUIT:
                self.stop()

            if event.type == pg.KEYDOWN:
                # Execute key press runnables
                self.runnable_system.execute_runnables('key_press', event.key, self)
                # Pass to input system for event state tracking
                self.input.process_event(event)
            elif event.type == pg.KEYUP:
                # Pass to input system for event state tracking
                self.input.process_event(event)
            if event.type == pg.MOUSEWHEEL:
                # Handle mouse wheel events - pass to input system
                self.input.process_event(event)
            elif event.type == pg.MOUSEBUTTONDOWN:
                # Handle mouse button down events - pass to input system
                self.input.process_event(event)
            elif event.type == pg.MOUSEBUTTONUP:
                # Handle mouse button up events - pass to input system
                self.input.process_event(event)
            elif event.type == pg.MOUSEMOTION:
                # Handle mouse motion events - pass to input system
                self.input.process_event(event)

            if event.type == pg.VIDEORESIZE:
                # Handle window resize
                self.__handleResize(event)
                self.setWindowTitle(self.__windowName)

    def __renderBackground(self):
        """Render background color."""
        # Always fill the surface (even in headless mode) for compatibility
        self.screen.fill(self.background_color)

    def __renderGameObject(self, gameobj: GameObject):
        """Render a single game object with camera transform."""
        if not gameobj.enabled:
            return

        # Check if object is visible
        visible_rect = self.camera.get_visible_rect()
        obj_bounds = pg.Rect(
            gameobj.position.x - gameobj.size.x,
            gameobj.position.y - gameobj.size.y,
            gameobj.size.x * 2,
            gameobj.size.y * 2
        )

        if not visible_rect.colliderect(obj_bounds):
            return  # Skip rendering if not visible

        # Convert world position to screen position
        screen_pos = self.camera.world_to_screen(gameobj.position)

        # Check for NaN values
        if (isinstance(screen_pos.x, float) and (screen_pos.x != screen_pos.x or screen_pos.x == float('inf') or screen_pos.x == float('-inf'))) or \
           (isinstance(screen_pos.y, float) and (screen_pos.y != screen_pos.y or screen_pos.y == float('inf') or screen_pos.y == float('-inf'))):
            return  # Skip rendering if position is invalid

        pos = (int(screen_pos.x), int(screen_pos.y))

        # Apply zoom to size
        zoom = self.camera.zoom

        # Update sprite position for rendering
        gameobj.rect.center = pos

        # Check for Sprite component and render it if present
        from ..rendering.sprite import Sprite
        sprite_component = gameobj.get_component(Sprite)
        if sprite_component and sprite_component.enabled and sprite_component._current_image:
            # Render using Sprite component
            sprite_component.render(self.screen, self.camera)
            return  # Skip basic shape rendering

        # Render based on shape with zoom (fallback if no sprite)
        if gameobj.basicShape == BasicShape.Circle:
            radius = 40 if gameobj.size.x == 0 else int(max(gameobj.size.x, gameobj.size.y) / 2)
            if self.__useDisplay:
                pg.draw.circle(self.screen, gameobj.color, pos, int(radius * zoom))

            # Draw rotation line only if configured to show it
            if hasattr(gameobj, 'show_rotation_line') and gameobj.show_rotation_line:
                import math
                angle_rad = math.radians(gameobj.rotation)
                end_x = pos[0] + radius * zoom * math.cos(angle_rad)
                end_y = pos[1] + radius * zoom * math.sin(angle_rad)

                # Check for NaN values
                if (isinstance(end_x, float) and (end_x != end_x or end_x == float('inf') or end_x == float('-inf'))) or \
                   (isinstance(end_y, float) and (end_y != end_y or end_y == float('inf') or end_y == float('-inf'))):
                    pass  # Skip drawing rotation line if invalid
                else:
                    if self.__useDisplay:
                        pg.draw.line(self.screen, Color(255, 255, 255), pos, (int(end_x), int(end_y)), 2)

        elif gameobj.basicShape == BasicShape.Rectangle:
            if abs(gameobj.rotation) < 0.1:
                # Non-rotated rectangle
                width = 80 if gameobj.size.x == 0 else int(gameobj.size.x * zoom)
                height = 80 if gameobj.size.y == 0 else int(gameobj.size.y * zoom)
                rect = pg.Rect(pos[0] - width//2, pos[1] - height//2, width, height)
                if self.__useDisplay:
                    pg.draw.rect(self.screen, gameobj.color, rect)
            else:
                # Rotated rectangle
                width = 80 if gameobj.size.x == 0 else int(gameobj.size.x * zoom)
                height = 80 if gameobj.size.y == 0 else int(gameobj.size.y * zoom)

                surf = pg.Surface((width, height), pg.SRCALPHA)
                surf.fill(gameobj.color)

                rotated_surf = pg.transform.rotate(surf, gameobj.rotation)
                rotated_rect = rotated_surf.get_rect()
                rotated_rect.center = pos

                if self.__useDisplay:
                    self.screen.blit(rotated_surf, rotated_rect)

    def __renderBody(self):
        """Render all game objects."""
        for gameobj in self.__gameobjects:
            if gameobj:
                self.__renderGameObject(gameobj)

    def __renderUI(self):
        """Render UI elements like FPS counter."""
        if Engine.log_debug:
            fps = self.clock.get_fps()
            font = pg.font.Font(None, 36)
            fps_text = font.render(f"FPS: {fps:.1f}", True, Color(255, 255, 255))
            if self.__useDisplay:
                self.screen.blit(fps_text, (10, 10))

    def __render(self):
        """Main rendering pipeline."""
        # In headless mode, skip rendering but still process timing
        if self.__useDisplay:
            self.__renderBackground()
            self.__renderBody()

            # Execute render runnables
            self.runnable_system.execute_runnables('render', engine=self)

            self.__renderUI()

            # Update display
            pg.display.flip()

        # Limit FPS and calculate delta time
        # In headless mode with high time_scale, optionally uncap FPS for maximum speed
        if self.fpsCap > 0 and (self.__useDisplay or self.time_scale <= 1.0):
            self.__dt = self.clock.tick(self.fpsCap) / 1000.00
        else:
            # Uncapped FPS for headless mode with time_scale > 1.0
            self.__dt = self.clock.tick() / 1000.00

    def __update(self):
        """Main game loop update."""

        # Execute update runnables
        self.runnable_system.execute_runnables('update', engine=self)

        # Update camera
        self.camera.update(self.__dt)

        # Update input system
        self.input.update()

        # Update all game objects and their components/scripts
        for gameobj in self.__gameobjects:
            if gameobj is not None and gameobj.enabled:
                gameobj.update(self)  # Pass engine reference to scripts

        # Execute physics update runnables
        self.runnable_system.execute_runnables('physics_update', engine=self)

        # Run physics simulation AFTER all updates
        self.physics_system.update(self, self.getGameObjects())

        # Process queued events before rendering
        self.event_manager.process_queue()

        self.__processEvents()
        self.__render()

    def pause_physics(self):
        """Pause the physics simulation."""
        self.physics_system.pause()

    def unpause_physics(self):
        """Unpause the physics simulation."""
        self.physics_system.unpause()

    def toggle_physics(self):
        """Toggle the pause state of the physics simulation."""
        self.physics_system.toggle_pause()

    def __del__(self):
        """Clean up all game objects on destruction."""
        for gameobj in self.__gameobjects:
            if gameobj:
                gameobj.destroy()
        Engine.__debug_log("Engine Destroyed")


class GlobalDictionary:
    """Optimized global variable system for the game engine."""

    def __init__(self):
        self._variables: OrderedDict = OrderedDict()
        self._categories: OrderedDict = OrderedDict()
        self._lock = threading.RLock()
        self._cache = {}
        self._cache_size = 100

    def set(self, key: str, value, category: str = "default"):
        """Set a global variable with category support."""
        with self._lock:
            if category == "default":
                self._variables[key] = value
            else:
                if category not in self._categories:
                    self._categories[category] = OrderedDict()
                self._categories[category][key] = value

            # Update cache
            cache_key = f"{category}:{key}"
            self._cache[cache_key] = value

            # Simple cache eviction
            if len(self._cache) > self._cache_size:
                self._cache.pop(next(iter(self._cache)))

    def get(self, key: str, default=None, category: str = "default"):
        """Get a global variable with caching."""
        cache_key = f"{category}:{key}"

        # Check cache first
        if cache_key in self._cache:
            return self._cache[cache_key]

        with self._lock:
            if category == "default":
                value = self._variables.get(key, default)
            else:
                value = self._categories.get(category, {}).get(key, default)

            # Cache the result
            self._cache[cache_key] = value
            return value

    def has(self, key: str, category: str = "default"):
        """Check if variable exists."""
        with self._lock:
            if category == "default":
                return key in self._variables
            return key in self._categories.get(category, {})

    def remove(self, key: str, category: str = "default"):
        """Remove a variable, returns True if removed."""
        with self._lock:
            cache_key = f"{category}:{key}"
            self._cache.pop(cache_key, None)

            if category == "default":
                if key in self._variables:
                    del self._variables[key]
                    return True
            else:
                if category in self._categories and key in self._categories[category]:
                    del self._categories[category][key]
                    return True
            return False

    def clear_category(self, category: str):
        """Clear all variables in a category."""
        with self._lock:
            if category == "default":
                self._variables.clear()
            else:
                self._categories.pop(category, None)

            # Clear cache entries for this category
            keys_to_remove = [k for k in self._cache.keys() if k.startswith(f"{category}:")]
            for k in keys_to_remove:
                self._cache.pop(k, None)

    def get_all(self, category: str = "default"):
        """Get all variables in a category."""
        with self._lock:
            if category == "default":
                return dict(self._variables)
            return dict(self._categories.get(category, {}))

