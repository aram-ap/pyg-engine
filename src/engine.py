import pygame as pg
from pygame import Color
from .object_types import Size, BasicShape, Tag
from .gameobject import GameObject
from .pymunk_physics_system import PymunkPhysicsSystem
from .camera import Camera
from .mouse_input import MouseInputSystem

class Engine:
    """Core game engine that handles the main loop, rendering, and system coordination."""
    
    log_debug = False
    
    def __init__(self, size: Size = Size(w=800, h=600),
                 backgroundColor: Color = Color(0,0,0),
                 running = False, paused = False,
                 windowName = "PyGame", displaySize = True, fpsCap = 60):
        Engine.__debug_log("Initializing Engine")
        self.isRunning:bool = running
        self.isPaused:bool = paused
        self.fpsCap:int = fpsCap
        self.__size = size
        self.__dt = 0.0

        # Window setup
        self.__displaySizeInTitle:bool = displaySize
        self.__windowName:str = "{} {}".format(windowName,
                                               str(self.__size) if displaySize else "")
        self.__backgroundColor = backgroundColor

        # Sprite groups for efficient rendering
        self.__gameobjects = pg.sprite.Group()
        self.__all_sprites = pg.sprite.Group()

        # Core systems initialization
        self.physics_system = PymunkPhysicsSystem()
        self.mouse_input = MouseInputSystem()

        pg.init()
        # Create resizable window
        self.screen = pg.display.set_mode((self.__size.w,self.__size.h), pg.RESIZABLE)
        self.clock = pg.time.Clock()
        self.camera = Camera(self.__size.w, self.__size.h)
        self.camera.scale_mode = "fit"  # Options: "fit", "fill", "stretch", "fixed"

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
        self.screen = pg.display.set_mode((self.__size.w, self.__size.h), pg.RESIZABLE)

    def toggPause(self):
        """Toggle pause state."""
        self.isPaused = not self.isPaused
        Engine.__debug_log("PAUSE: {}".format(self.isPaused))

    def setPause(self, pause:bool):
        """Set pause state explicitly."""
        self.isPaused = pause
        Engine.__debug_log("PAUSE: {}".format(self.isPaused))

    def paused(self)->bool:
        """Check if engine is paused."""
        return self.isPaused

    def stop(self):
        """Stop the game engine."""
        self.isRunning = False
        Engine.__debug_log("Engine Stopped")

    def setRunning(self, running:bool):
        """Set running state."""
        if(self.isRunning != running):
            self.isRunning = running
            Engine.__debug_log("Engine set to " + ("running" if running else "NOT running"))

    def running(self)->bool:
        """Check if engine is running."""
        return self.isRunning

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
        self.screen = pg.display.set_mode((self.__size.w, self.__size.h), pg.RESIZABLE)
        self.camera.resize(event.w, event.h)
        Engine.__debug_log("Handling Resize to {}".format(self.__size))

    # ================= Game Loop ======================

    def start(self):
        """Start the main game loop."""
        Engine.__debug_log("Starting Engine")
        self.isRunning = True
        self.isPaused = False

        while(self.isRunning):
            self.__update()

    def __processEvents(self):
        """Process pygame events."""
        for event in pg.event.get():
            if event.type == pg.QUIT:
                self.stop()

            if event.type == pg.KEYDOWN:
                if event.key == pg.K_ESCAPE:
                    self.toggPause()

            if event.type == pg.VIDEORESIZE:
                # Handle window resize
                self.__handleResize(event)
                self.setWindowTitle(self.__windowName)
            
            # Handle mouse wheel events
            if event.type == pg.MOUSEWHEEL:
                self.mouse_input.current_state.wheel_delta = event.y

    def __renderBackground(self):
        """Render background color."""
        self.screen.fill(self.__backgroundColor)

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
                    pg.draw.line(self.screen, Color(255, 255, 255), pos, (int(end_x), int(end_y)), 2)
                
        elif gameobj.basicShape == BasicShape.Rectangle:
            if abs(gameobj.rotation) < 0.1:
                # Non-rotated rectangle
                width = 80 if gameobj.size.x == 0 else int(gameobj.size.x * zoom)
                height = 80 if gameobj.size.y == 0 else int(gameobj.size.y * zoom)
                rect = pg.Rect(pos[0] - width//2, pos[1] - height//2, width, height)
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
            self.screen.blit(fps_text, (10, 10))

    def __render(self):
        """Main rendering pipeline."""
        self.__renderBackground()
        self.__renderBody()
        self.__renderUI()

        # Update display
        pg.display.flip()

        # Limit FPS and calculate delta time
        if(self.fpsCap > 0):
            self.__dt = self.clock.tick(self.fpsCap) / 1000.00
        else:
            self.__dt = self.clock.tick() / 1000.00

    def __update(self):
        """Main game loop update."""
        if not self.paused():
            # Update camera
            self.camera.update(self.__dt)

            # Update mouse input system
            self.mouse_input.update(self)

            # Update all game objects and their components/scripts
            for gameobj in self.__gameobjects:
                if gameobj is not None and gameobj.enabled:
                    gameobj.update(self)  # Pass engine reference to scripts

            # Run physics simulation AFTER all updates
            self.physics_system.update(self, self.getGameObjects())

        self.__processEvents()
        self.__render()

    def __del__(self):
        """Clean up all game objects on destruction."""
        for gameobj in self.__gameobjects:
            if gameobj:
                gameobj.destroy()
        Engine.__debug_log("Engine Destroyed")

