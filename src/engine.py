import pygame as pg
from pygame import Color
from collections import OrderedDict, defaultdict, deque
import weakref
import threading
from dataclasses import dataclass, field
import time
from .object_types import Size, BasicShape, Tag
from .gameobject import GameObject
from .physics_system import PhysicsSystem
from .camera import Camera
from .runnable import RunnableSystem, Priority
from .input import Input
from .rigidbody import RigidBody
from .event_manager import EventManager

class Engine:
    """Core game engine that handles the main loop, rendering, and system coordination."""

    log_debug = False

    def __init__(self, size: Size = Size(w=800, h=600),
                 backgroundColor: Color = Color(0,0,0),
                 running = False,
                 windowName = "PyGame", displaySize = True, fpsCap = 60, useDisplay = True):
        Engine.__debug_log("Initializing Engine")
        self.isRunning:bool = running
        self.fpsCap:int = fpsCap
        self.__size = size
        self.__dt = 0.0
        self.__useDisplay = useDisplay

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


        pg.init()
        # Create resizable window
        self.clock = pg.time.Clock()

        self.camera = Camera(self.__size.w, self.__size.h)
        self.camera.scale_mode = "fit"  # Options: "fit", "fill", "stretch", "fixed"

        if useDisplay:
            self.screen = pg.display.set_mode((self.__size.w, self.__size.h), pg.RESIZABLE)

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
            self.screen = pg.display.set_mode((self.__size.w, self.__size.h), pg.RESIZABLE)

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

    # ================= Physics Stuff ======================
    def dt(self)->float:
        """Return delta time (time since last frame)."""
        return self.__dt

    # ================= Display Stuff ======================
    def setWindowTitle(self, title: str):
        """Set window title."""
        self.__windowName = title
        new_title = title + " (%s,%s)" % (self.__size.w, self.__size.h) if self.__displaySizeInTitle else title
        pg.display.set_caption(new_title)
        Engine.__debug_log("Changed window title to: '{}'".format(new_title))

    def __handleResize(self, event: pg.Event):
        """Handle window resize events."""
        self.__size = Size(event.w, event.h)
        if self.__useDisplay:
            self.screen = pg.display.set_mode((self.__size.w, self.__size.h), pg.RESIZABLE)
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
            self.__update()

    def __processEvents(self):
        """Process pygame events."""
        for event in pg.event.get():
            if event.type == pg.QUIT:
                self.stop()

            if event.type == pg.KEYDOWN:
                # Execute key press runnables
                self.runnable_system.execute_runnables('key_press', event.key, self)

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
        if self.__useDisplay:
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

        # Render based on shape with zoom
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
        self.__renderBackground()
        self.__renderBody()

        # Execute render runnables
        self.runnable_system.execute_runnables('render', engine=self)

        self.__renderUI()

        # Update display
        if self.__useDisplay:
            pg.display.flip()

        # Limit FPS and calculate delta time
        if(self.fpsCap > 0):
            self.__dt = self.clock.tick(self.fpsCap) / 1000.00
        else:
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

