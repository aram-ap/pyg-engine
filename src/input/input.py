"""
The Input module manages all input devices. Axis integrations allow for interop between joysticks
and keyboard. Easily access mouse+keyboard direct input. Easily assign keys to aliases.
"""

import pygame
from pygame.locals import *
from enum import Enum, auto
import time


class Input:
    class Keybind(Enum):
        # Letter keys
        A=pygame.K_a
        B=pygame.K_b
        C=pygame.K_c
        D=pygame.K_d
        E=pygame.K_e
        F=pygame.K_f
        G=pygame.K_g
        H=pygame.K_h
        I=pygame.K_i
        J=pygame.K_j
        K=pygame.K_k
        L=pygame.K_l
        M=pygame.K_m
        N=pygame.K_n
        O=pygame.K_o
        P=pygame.K_p
        Q=pygame.K_q
        R=pygame.K_r
        S=pygame.K_s
        T=pygame.K_t
        U=pygame.K_u
        V=pygame.K_v
        W=pygame.K_w
        X=pygame.K_x
        Y=pygame.K_y
        Z=pygame.K_z

        # Number keys
        K_1=pygame.K_1
        K_2=pygame.K_2
        K_3=pygame.K_3
        K_4=pygame.K_4
        K_5=pygame.K_5
        K_6=pygame.K_6
        K_7=pygame.K_7
        K_8=pygame.K_8
        K_9=pygame.K_9
        K_0=pygame.K_0

        # Function keys
        F1=pygame.K_F1
        F2=pygame.K_F2
        F3=pygame.K_F3
        F4=pygame.K_F4
        F5=pygame.K_F5
        F6=pygame.K_F6
        F7=pygame.K_F7
        F8=pygame.K_F8
        F9=pygame.K_F9
        F10=pygame.K_F10
        F11=pygame.K_F11
        F12=pygame.K_F12

        # Special keys
        K_MINUS=pygame.K_MINUS
        K_PLUS=pygame.K_PLUS
        K_EQUALS=pygame.K_EQUALS
        K_BACKSPACE=pygame.K_BACKSPACE
        K_TAB=pygame.K_TAB
        K_CAPS_LOCK=pygame.K_CAPSLOCK
        K_ENTER=pygame.K_RETURN
        K_ESCAPE=pygame.K_ESCAPE
        K_SPACE=pygame.K_SPACE
        K_BACKSLASH=pygame.K_BACKSLASH
        K_SLASH=pygame.K_SLASH
        K_COMMA=pygame.K_COMMA
        K_PERIOD=pygame.K_PERIOD
        K_SEMICOLON=pygame.K_SEMICOLON
        K_QUOTE=pygame.K_QUOTE
        K_BACKQUOTE=pygame.K_BACKQUOTE
        K_BRACKET_LEFT=pygame.K_LEFTBRACKET
        K_BRACKET_RIGHT=pygame.K_RIGHTBRACKET
        K_GRAVE=pygame.K_BACKQUOTE

        # Arrow keys
        K_UP=pygame.K_UP
        K_DOWN=pygame.K_DOWN
        K_LEFT=pygame.K_LEFT
        K_RIGHT=pygame.K_RIGHT

        # Navigation keys
        K_HOME=pygame.K_HOME
        K_END=pygame.K_END
        K_PAGE_UP=pygame.K_PAGEUP
        K_PAGE_DOWN=pygame.K_PAGEDOWN
        K_INSERT=pygame.K_INSERT
        K_DELETE=pygame.K_DELETE

        # Numpad keys
        K_NUMPAD_0=pygame.K_KP0
        K_NUMPAD_1=pygame.K_KP1
        K_NUMPAD_2=pygame.K_KP2
        K_NUMPAD_3=pygame.K_KP3
        K_NUMPAD_4=pygame.K_KP4
        K_NUMPAD_5=pygame.K_KP5
        K_NUMPAD_6=pygame.K_KP6
        K_NUMPAD_7=pygame.K_KP7
        K_NUMPAD_8=pygame.K_KP8
        K_NUMPAD_9=pygame.K_KP9
        K_NUMPAD_PLUS=pygame.K_KP_PLUS
        K_NUMPAD_MINUS=pygame.K_KP_MINUS
        K_NUMPAD_MULTIPLY=pygame.K_KP_MULTIPLY
        K_NUMPAD_DIVIDE=pygame.K_KP_DIVIDE
        K_NUMPAD_ENTER=pygame.K_KP_ENTER
        K_NUMPAD_DECIMAL=pygame.K_KP_PERIOD

        # Modifier keys (M_ prefix)
        M_SHIFT_L=pygame.K_LSHIFT
        M_SHIFT_R=pygame.K_RSHIFT
        M_CTRL_L=pygame.K_LCTRL
        M_CTRL_R=pygame.K_RCTRL
        M_ALT_L=pygame.K_LALT
        M_ALT_R=pygame.K_RALT
        M_SUPER_L=pygame.K_LSUPER  # Windows/Command key
        M_SUPER_R=pygame.K_RSUPER  # Windows/Command key
        M_CAPS_LOCK=pygame.K_CAPSLOCK
        M_NUM_LOCK=pygame.K_NUMLOCK
        M_SCROLL_LOCK=pygame.K_SCROLLLOCK

        # Mouse buttons
        MOUSE_LEFT=0
        MOUSE_RIGHT=2
        MOUSE_MIDDLE=1
        # MOUSE_BUTTON_4=3
        # MOUSE_BUTTON_5=4

    class Axis(Enum):
        # Movement axes
        HORIZONTAL=auto(),
        VERTICAL=auto(),
        MOUSE_X=auto(),
        MOUSE_Y=auto(),
        MOUSE_REL_X=auto(),
        MOUSE_REL_Y=auto(),
        MOUSE_SCROLL=auto(),

        # Gameplay axes
        JUMP=auto(),
        CROUCH=auto(),
        CRAWL=auto(),
        RUN=auto(),
        SPRINT=auto(),
        WALK=auto(),

        # Combat/Interaction axes
        FIRE=auto(),
        FIRE2=auto(),
        FIRE3=auto(),
        RELOAD=auto(),
        INTERACT=auto(),
        USE=auto(),
        DROP=auto(),
        PICKUP=auto(),

        # Camera/View axes
        LOOK_X=auto(),
        LOOK_Y=auto(),
        ZOOM_IN=auto(),
        ZOOM_OUT=auto(),
        CAMERA_LEFT=auto(),
        CAMERA_RIGHT=auto(),
        CAMERA_UP=auto(),
        CAMERA_DOWN=auto(),

        # Menu/UI axes
        MENU_UP=auto(),
        MENU_DOWN=auto(),
        MENU_LEFT=auto(),
        MENU_RIGHT=auto(),
        MENU_SELECT=auto(),
        MENU_BACK=auto(),
        MENU_START=auto(),
        PAUSE=auto(),

        # Vehicle/Transport axes
        ACCELERATE=auto(),
        BRAKE=auto(),
        STEER_LEFT=auto(),
        STEER_RIGHT=auto(),
        HANDBRAKE=auto(),

        # Advanced movement
        STRAFE_LEFT=auto(),
        STRAFE_RIGHT=auto(),
        LEAN_LEFT=auto(),
        LEAN_RIGHT=auto(),
        PRONE=auto(),
        SLIDE=auto(),
        WALL_RUN=auto(),

        # Weapon/Equipment axes
        AIM=auto(),
        SCOPE=auto(),
        SWITCH_WEAPON=auto(),
        SWITCH_WEAPON_NEXT=auto(),
        SWITCH_WEAPON_PREV=auto(),
        THROW_GRENADE=auto(),
        MELEE=auto(),
        BLOCK=auto(),

        # Communication/Social
        VOICE_CHAT=auto(),
        TEXT_CHAT=auto(),
        EMOTE=auto(),
        GESTURE=auto(),

        # Debug/Development
        DEBUG_MENU=auto(),
        CONSOLE=auto(),
        SCREENSHOT=auto(),
        RECORD=auto(),

        # Joystick/Gamepad axes
        JOYSTICK_LEFT_X=auto(),
        JOYSTICK_LEFT_Y=auto(),
        JOYSTICK_RIGHT_X=auto(),
        JOYSTICK_RIGHT_Y=auto(),
        JOYSTICK_LEFT_TRIGGER=auto(),
        JOYSTICK_RIGHT_TRIGGER=auto(),

        # Joystick buttons
        JOYSTICK_A=auto(),
        JOYSTICK_B=auto(),
        JOYSTICK_X=auto(),
        JOYSTICK_Y=auto(),
        JOYSTICK_LB=auto(),
        JOYSTICK_RB=auto(),
        JOYSTICK_BACK=auto(),
        JOYSTICK_START=auto(),
        JOYSTICK_LEFT_STICK=auto(),
        JOYSTICK_RIGHT_STICK=auto(),
        JOYSTICK_DPAD_UP=auto(),
        JOYSTICK_DPAD_DOWN=auto(),
        JOYSTICK_DPAD_LEFT=auto(),
        JOYSTICK_DPAD_RIGHT=auto()

    class InputEvent(Enum):
        """Input event types for consistent event handling."""
        # Mouse events
        MOUSE_SCROLL_UP = auto()
        MOUSE_SCROLL_DOWN = auto()
        MOUSE_BUTTON_DOWN = auto()
        MOUSE_BUTTON_UP = auto()
        MOUSE_MOTION = auto()
        MOUSE_ENTER = auto()
        MOUSE_LEAVE = auto()

        # Keyboard events
        KEY_DOWN = auto()
        KEY_UP = auto()
        KEY_REPEAT = auto()
        KEY_HOLD = auto()

        # Joystick events
        JOYSTICK_BUTTON_DOWN = auto()
        JOYSTICK_BUTTON_UP = auto()
        JOYSTICK_AXIS_MOTION = auto()
        JOYSTICK_HAT_MOTION = auto()
        JOYSTICK_BALL_MOTION = auto()
        JOYSTICK_CONNECTED = auto()
        JOYSTICK_DISCONNECTED = auto()

        # Window events
        WINDOW_FOCUS_GAINED = auto()
        WINDOW_FOCUS_LOST = auto()
        WINDOW_RESIZED = auto()
        WINDOW_MOVED = auto()
        WINDOW_MINIMIZED = auto()
        WINDOW_RESTORED = auto()

        # System events
        QUIT = auto()
        ACTIVEEVENT = auto()
        VIDEORESIZE = auto()
        VIDEOEXPOSE = auto()

        # Custom events
        INPUT_DEVICE_CHANGED = auto()
        INPUT_MAPPING_CHANGED = auto()
        INPUT_PROFILE_LOADED = auto()
        INPUT_PROFILE_SAVED = auto()

    class Mouse:
        """Main mouse interface providing clean access to mouse position, relative movement, and scroll."""

        def __init__(self, input):
            self.input = input
            self._last_pos = (0, 0)
            self._current_pos = (0, 0)
            self._rel_movement = (0, 0)
            self._scroll_delta = (0, 0)
            self._button_states = [False, False, False]  # Left, Middle, Right, Button4, Button5

        def get_pos(self) -> tuple[int, int]:
            """Get the current mouse position as a tuple (x, y)."""
            return pygame.mouse.get_pos()

        def get_rel(self) -> tuple[int, int]:
            """Get the relative mouse movement since last frame as a tuple (x, y)."""
            return pygame.mouse.get_rel()

        def get_scroll(self) -> tuple[int, int]:
            """Get the scroll wheel movement as a tuple (x, y). Returns (0, 0) if no scroll."""
            # Check for scroll events from the input system
            scroll_x = 0
            scroll_y = 0

            # Check for vertical scroll (button 4 = up, button 5 = down)
            if self.input.get_event_state('mouse_scroll_up'):
                scroll_y = 1
            elif self.input.get_event_state('mouse_scroll_down'):
                scroll_y = -1

            return (scroll_x, scroll_y)

        def get_button(self, button: int) -> bool:
            """Get the state of a mouse button. 0=Left, 1=Middle, 2=Right, 3=Button4, 4=Button5."""
            return pygame.mouse.get_pressed()[button]

        def get_button_down(self, button: int) -> bool:
            """Check if a mouse button was pressed this frame."""
            return self.input.get_event_state('mouse_button_down', button)

        def get_button_up(self, button: int) -> bool:
            """Check if a mouse button was released this frame."""
            return self.input.get_event_state('mouse_button_up', button)

        def set_pos(self, pos: tuple[int, int]):
            """Set the mouse position."""
            pygame.mouse.set_pos(pos)

        def set_visible(self, visible: bool):
            """Set mouse cursor visibility."""
            pygame.mouse.set_visible(visible)

        def get_visible(self) -> bool:
            """Get mouse cursor visibility."""
            return pygame.mouse.get_visible()

        def update(self):
            """Update mouse state - called by the input system each frame."""
            self._last_pos = self._current_pos
            self._current_pos = self.get_pos()
            self._rel_movement = self.get_rel()

            # Update button states
            pressed = pygame.mouse.get_pressed()
            for i in range(len(self._button_states)):
                self._button_states[i] = pressed[i]

    def __init__(self, engine):
        self.engine = engine
        self.key_states = {}
        self.mouse_states = {}
        self.key_aliases = {}
        self.axis_bindings = {}

        # Store mouse relative movement for the current frame
        self._mouse_rel_x = 0.0
        self._mouse_rel_y = 0.0

        # Initialize the main Mouse interface
        self.mouse = self.Mouse(self)

        # Enhanced event-based input system
        self.event_states = {
            # Mouse events
            self.InputEvent.MOUSE_SCROLL_UP: False,
            self.InputEvent.MOUSE_SCROLL_DOWN: False,
            self.InputEvent.MOUSE_BUTTON_DOWN: {},  # Track button down events per button
            self.InputEvent.MOUSE_BUTTON_UP: {},  # Track button up events per button
            self.InputEvent.MOUSE_MOTION: False,
            self.InputEvent.MOUSE_ENTER: False,
            self.InputEvent.MOUSE_LEAVE: False,

            # Keyboard events
            self.InputEvent.KEY_DOWN: {},  # Track key down events per key
            self.InputEvent.KEY_UP: {},  # Track key up events per key
            self.InputEvent.KEY_REPEAT: {},  # Track key repeat events per key
            self.InputEvent.KEY_HOLD: {},  # Track key hold events per key

            # Joystick events
            self.InputEvent.JOYSTICK_BUTTON_DOWN: {},  # Track joystick button down events
            self.InputEvent.JOYSTICK_BUTTON_UP: {},  # Track joystick button up events
            self.InputEvent.JOYSTICK_AXIS_MOTION: {},  # Track joystick axis motion events
            self.InputEvent.JOYSTICK_HAT_MOTION: {},  # Track joystick hat motion events
            self.InputEvent.JOYSTICK_BALL_MOTION: {},  # Track joystick ball motion events
            self.InputEvent.JOYSTICK_CONNECTED: False,
            self.InputEvent.JOYSTICK_DISCONNECTED: False,

            # Window events
            self.InputEvent.WINDOW_FOCUS_GAINED: False,
            self.InputEvent.WINDOW_FOCUS_LOST: False,
            self.InputEvent.WINDOW_RESIZED: False,
            self.InputEvent.WINDOW_MOVED: False,
            self.InputEvent.WINDOW_MINIMIZED: False,
            self.InputEvent.WINDOW_RESTORED: False,

            # System events
            self.InputEvent.QUIT: False,
            self.InputEvent.ACTIVEEVENT: False,
            self.InputEvent.VIDEORESIZE: False,
            self.InputEvent.VIDEOEXPOSE: False,

            # Custom events
            self.InputEvent.INPUT_DEVICE_CHANGED: False,
            self.InputEvent.INPUT_MAPPING_CHANGED: False,
            self.InputEvent.INPUT_PROFILE_LOADED: False,
            self.InputEvent.INPUT_PROFILE_SAVED: False,
        }

        # Legacy event states for backward compatibility
        self._legacy_event_states = {
            'mouse_scroll_up': False,
            'mouse_scroll_down': False,
            'mouse_button_up': {},  # Track button up events per button
            'mouse_button_down': {},  # Track button down events per button
            'key_up': {},  # Track key up events per key
            'key_down': {},  # Track key down events per key
            'joystick_button_up': {},  # Track joystick button up events
            'joystick_button_down': {},  # Track joystick button down events
            'joystick_axis_motion': {},  # Track joystick axis motion events
        }

        # Initialize alias dictionary after classes are defined
        self.alias = {

            # Movement axes (-1.0 to 1.0)
            self.Axis.HORIZONTAL: [self.Keybind.A, self.Keybind.D, self.Keybind.K_LEFT, self.Keybind.K_RIGHT, self.Axis.JOYSTICK_LEFT_X],
            self.Axis.VERTICAL: [self.Keybind.W, self.Keybind.S, self.Keybind.K_UP, self.Keybind.K_DOWN, self.Axis.JOYSTICK_LEFT_Y],

            # Vehicle axes (-1.0 to 1.0)
            self.Axis.ACCELERATE: [self.Keybind.W, self.Keybind.K_UP, self.Axis.JOYSTICK_RIGHT_TRIGGER],
            self.Axis.BRAKE: [self.Keybind.S, self.Keybind.K_DOWN, self.Axis.JOYSTICK_LEFT_TRIGGER],
            self.Axis.STEER_LEFT: [self.Keybind.A, self.Keybind.K_LEFT, self.Axis.JOYSTICK_LEFT_X],
            self.Axis.STEER_RIGHT: [self.Keybind.D, self.Keybind.K_RIGHT, self.Axis.JOYSTICK_LEFT_X],

            # Mouse axes
            self.Axis.MOUSE_X: [self.Axis.MOUSE_X],
            self.Axis.MOUSE_Y: [self.Axis.MOUSE_Y],
            self.Axis.MOUSE_SCROLL: [self.Axis.MOUSE_SCROLL],
            self.Axis.MOUSE_REL_X: [self.Axis.MOUSE_REL_X],
            self.Axis.MOUSE_REL_Y: [self.Axis.MOUSE_REL_Y],

            # Gameplay axes
            self.Axis.JUMP: [self.Keybind.K_SPACE, self.Keybind.W, self.Axis.JOYSTICK_A],
            self.Axis.CROUCH: [self.Keybind.C, self.Keybind.K_DOWN, self.Axis.JOYSTICK_B],
            self.Axis.SPRINT: [self.Keybind.M_SHIFT_L, self.Keybind.M_SHIFT_R, self.Axis.JOYSTICK_LEFT_STICK],
            self.Axis.WALK: [self.Keybind.M_ALT_L, self.Keybind.M_ALT_R],

            # Combat axes
            self.Axis.FIRE: [self.Keybind.K_SPACE, self.Keybind.MOUSE_LEFT, self.Axis.JOYSTICK_RIGHT_TRIGGER],
            self.Axis.FIRE2: [self.Keybind.MOUSE_RIGHT, self.Axis.JOYSTICK_LEFT_TRIGGER],
            self.Axis.FIRE3: [self.Keybind.MOUSE_MIDDLE, self.Axis.JOYSTICK_RB],
            self.Axis.RELOAD: [self.Keybind.R, self.Axis.JOYSTICK_X],
            self.Axis.INTERACT: [self.Keybind.E, self.Keybind.F, self.Axis.JOYSTICK_Y],

            # Camera axes
            self.Axis.LOOK_X: [self.Axis.MOUSE_X, self.Axis.JOYSTICK_RIGHT_X],
            self.Axis.LOOK_Y: [self.Axis.MOUSE_Y, self.Axis.JOYSTICK_RIGHT_Y],
            self.Axis.ZOOM_IN: [self.Keybind.K_EQUALS, self.Axis.MOUSE_SCROLL],
            self.Axis.ZOOM_OUT: [self.Keybind.K_MINUS, self.Axis.MOUSE_SCROLL],

            # Menu/UI axes
            self.Axis.MENU_UP: [self.Keybind.K_UP, self.Keybind.W, self.Axis.JOYSTICK_DPAD_UP],
            self.Axis.MENU_DOWN: [self.Keybind.K_DOWN, self.Keybind.S, self.Axis.JOYSTICK_DPAD_DOWN],
            self.Axis.MENU_LEFT: [self.Keybind.K_LEFT, self.Keybind.A, self.Axis.JOYSTICK_DPAD_LEFT],
            self.Axis.MENU_RIGHT: [self.Keybind.K_RIGHT, self.Keybind.D, self.Axis.JOYSTICK_DPAD_RIGHT],
            self.Axis.MENU_SELECT: [self.Keybind.K_ENTER, self.Keybind.K_SPACE, self.Axis.JOYSTICK_A],
            self.Axis.MENU_BACK: [self.Keybind.K_ESCAPE, self.Keybind.K_BACKSPACE, self.Axis.JOYSTICK_B],
            self.Axis.PAUSE: [self.Keybind.K_ESCAPE, self.Axis.JOYSTICK_START],

            # Advanced movement
            self.Axis.STRAFE_LEFT: [self.Keybind.Q, self.Keybind.A],
            self.Axis.STRAFE_RIGHT: [self.Keybind.E, self.Keybind.D],
            self.Axis.LEAN_LEFT: [self.Keybind.Q],
            self.Axis.LEAN_RIGHT: [self.Keybind.E],
            self.Axis.PRONE: [self.Keybind.Z],
            self.Axis.SLIDE: [self.Keybind.C],

            # Weapon/Equipment
            self.Axis.AIM: [self.Keybind.MOUSE_RIGHT, self.Axis.JOYSTICK_LEFT_TRIGGER],
            self.Axis.SCOPE: [self.Keybind.R, self.Axis.JOYSTICK_RIGHT_STICK],
            self.Axis.SWITCH_WEAPON_NEXT: [self.Keybind.K_TAB, self.Keybind.K_EQUALS, self.Axis.JOYSTICK_RB],
            self.Axis.SWITCH_WEAPON_PREV: [self.Keybind.K_MINUS, self.Axis.JOYSTICK_LB],
            self.Axis.THROW_GRENADE: [self.Keybind.G, self.Axis.JOYSTICK_Y],
            self.Axis.MELEE: [self.Keybind.F, self.Axis.JOYSTICK_X],

            # Communication
            self.Axis.VOICE_CHAT: [self.Keybind.V],
            self.Axis.TEXT_CHAT: [self.Keybind.T],

            # Debug/Development
            self.Axis.DEBUG_MENU: [self.Keybind.F1],
            self.Axis.CONSOLE: [self.Keybind.K_BACKQUOTE],
            self.Axis.SCREENSHOT: [self.Keybind.F12],

            # Key aliases (legacy support)
            "left": self.Keybind.K_LEFT,
            "right": self.Keybind.K_RIGHT,
            "up": self.Keybind.K_UP,
            "down": self.Keybind.K_DOWN,
            "space": self.Keybind.K_SPACE,
            "enter": self.Keybind.K_ENTER,
            "escape": self.Keybind.K_ESCAPE,
        }

        self.last_joystick_check = 0
        self.joystick_init_time_interval = 2000
        self.joystick_init_max_tries = 20
        self.__joystick_init_tries = 0


    def get(self, input: 'Input.Keybind | Input.Axis | str') -> bool | float:
        if isinstance(input, self.Keybind):
            mouse_buttons = [
                        self.Keybind.MOUSE_LEFT, self.Keybind.MOUSE_RIGHT, self.Keybind.MOUSE_MIDDLE,
            ]

            if input in mouse_buttons:
                return pygame.mouse.get_pressed()[input.value]

            return pygame.key.get_pressed()[input.value]
        elif isinstance(input, self.Axis):
            # Check if it's a joystick button (returns bool) or axis (returns float)
            joystick_buttons = [
                self.Axis.JOYSTICK_A, self.Axis.JOYSTICK_B, self.Axis.JOYSTICK_X, self.Axis.JOYSTICK_Y,
                self.Axis.JOYSTICK_LB, self.Axis.JOYSTICK_RB, self.Axis.JOYSTICK_BACK, self.Axis.JOYSTICK_START,
                self.Axis.JOYSTICK_LEFT_STICK, self.Axis.JOYSTICK_RIGHT_STICK,
                self.Axis.JOYSTICK_DPAD_UP, self.Axis.JOYSTICK_DPAD_DOWN, self.Axis.JOYSTICK_DPAD_LEFT, self.Axis.JOYSTICK_DPAD_RIGHT
            ]

            if input in joystick_buttons:
                return self._get_joystick_button_value(input)
            else:
                return self.get_axis(input)
        elif isinstance(input, str):
            # Handle string aliases
            if input in self.alias:
                return self.get(self.alias[input])
            return False
        else:
            raise ValueError(f"Invalid input type: {type(input)}")


    def try_joystick_init(self) -> bool:
        if self.joystick_init_max_tries > 0 and self.__joystick_init_tries > self.joystick_init_max_tries:
            return False
        elif time.time()-self.last_joystick_check < self.joystick_init_time_interval:
            return False

        pygame.joystick.init()
        self.__joystick_init_tries += 1
        self.last_joystick_check = time.time()

        return pygame.joystick.get_init()


    def get_axis(self, axis: 'Input.Axis') -> float:
        """Get the current value of an input axis."""
        if axis in self.alias:
            # Check all bound inputs for this axis
            bound_inputs = self.alias[axis]
            total_value = 0.0

            for bound_input in bound_inputs:
                if isinstance(bound_input, self.Keybind):
                    if pygame.key.get_pressed()[bound_input.value]:
                        # Determine if positive or negative based on key
                        if bound_input in [self.Keybind.A, self.Keybind.K_LEFT, self.Keybind.S, self.Keybind.K_DOWN]:
                            total_value -= 1.0
                        else:
                            total_value += 1.0
                elif isinstance(bound_input, self.Axis):
                    # Handle joystick axes
                    if bound_input in [self.Axis.JOYSTICK_LEFT_X, self.Axis.JOYSTICK_LEFT_Y,
                                     self.Axis.JOYSTICK_RIGHT_X, self.Axis.JOYSTICK_RIGHT_Y,
                                     self.Axis.JOYSTICK_LEFT_TRIGGER, self.Axis.JOYSTICK_RIGHT_TRIGGER]:
                        joystick_value = self._get_joystick_axis_value(bound_input)
                        if joystick_value != 0.0:
                            total_value += joystick_value
                    elif bound_input in [self.Axis.MOUSE_REL_X, self.Axis.MOUSE_REL_Y, self.Axis.MOUSE_SCROLL]:
                        mouse_value = self._get_mouse_axis_value(bound_input)
                        if mouse_value != 0.0:
                            total_value += mouse_value/100.0
                    # Handle mouse axes
                    elif bound_input in [self.Axis.MOUSE_X, self.Axis.MOUSE_Y]:
                        mouse_value = self._get_mouse_axis_value(bound_input)
                        if mouse_value != 0.0:
                            total_value += mouse_value

            return max(-1.0, min(1.0, total_value))  # Clamp between -1 and 1

        return 0.0

    def get_raw_axis(self, axis: 'Input.Axis') -> float:
        """Get the raw value of an input axis without clamping."""
        if axis in self.alias:
            # Check all bound inputs for this axis
            bound_inputs = self.alias[axis]
            total_value = 0.0

            for bound_input in bound_inputs:
                if isinstance(bound_input, self.Keybind):
                    if pygame.key.get_pressed()[bound_input.value]:
                        # Determine if positive or negative based on key
                        if bound_input in [self.Keybind.A, self.Keybind.K_LEFT, self.Keybind.S, self.Keybind.K_DOWN]:
                            total_value -= 1.0
                        else:
                            total_value += 1.0
                elif isinstance(bound_input, self.Axis):
                    # Handle joystick axes
                    if bound_input in [self.Axis.JOYSTICK_LEFT_X, self.Axis.JOYSTICK_LEFT_Y,
                                     self.Axis.JOYSTICK_RIGHT_X, self.Axis.JOYSTICK_RIGHT_Y,
                                     self.Axis.JOYSTICK_LEFT_TRIGGER, self.Axis.JOYSTICK_RIGHT_TRIGGER]:
                        joystick_value = self._get_joystick_axis_value(bound_input)
                        if joystick_value != 0.0:
                            total_value += joystick_value
                    elif bound_input in [self.Axis.MOUSE_REL_X, self.Axis.MOUSE_REL_Y, self.Axis.MOUSE_SCROLL]:
                        mouse_value = self._get_mouse_axis_value(bound_input)
                        if mouse_value != 0.0:
                            total_value += mouse_value
                    # Handle mouse axes
                    elif bound_input in [self.Axis.MOUSE_X, self.Axis.MOUSE_Y]:
                        mouse_value = self._get_mouse_axis_value(bound_input)
                        if mouse_value != 0.0:
                            total_value += mouse_value

            return total_value  # No clamping

        return 0.0

    def _get_joystick_axis_value(self, axis: 'Input.Axis') -> float:
        """Get the value of a specific joystick axis."""
        # Skip if no joysticks detected
        if pygame.joystick.get_count() <= 0:
            return 0.0

        # Check for any uninitialized joysticks
        if not pygame.joystick.get_init():
            if not self.try_joystick_init():
                return 0.0

        try:
            joystick = pygame.joystick.Joystick(0)
            if not joystick.get_init():
                joystick.init()

            # Map axis enum to joystick axis index
            axis_mapping = {
                self.Axis.JOYSTICK_LEFT_X: 0,
                self.Axis.JOYSTICK_LEFT_Y: 1,
                self.Axis.JOYSTICK_RIGHT_X: 2,
                self.Axis.JOYSTICK_RIGHT_Y: 3,
                self.Axis.JOYSTICK_LEFT_TRIGGER: 4,
                self.Axis.JOYSTICK_RIGHT_TRIGGER: 5
            }

            if axis in axis_mapping:
                axis_index = axis_mapping[axis]
                if axis_index < joystick.get_numaxes():
                    value = joystick.get_axis(axis_index)
                    # Apply deadzone to prevent drift
                    if abs(value) < 0.1:
                        return 0.0
                    return value

        except Exception as e:
            print(f"Joystick error: {e}")
            return 0.0

        return 0.0

    def _get_joystick_button_value(self, button: 'Input.Axis') -> bool:
        """Get the value of a specific joystick button."""
        # Skip if no joysticks detected
        if pygame.joystick.get_count() <= 0:
            return False

        # Check for any uninitialized joysticks
        if not pygame.joystick.get_init():
            if not self.try_joystick_init():
                return False

        try:
            joystick = pygame.joystick.Joystick(0)
            if not joystick.get_init():
                joystick.init()

            # Map button enum to joystick button index
            button_mapping = {
                self.Axis.JOYSTICK_A: 0,
                self.Axis.JOYSTICK_B: 1,
                self.Axis.JOYSTICK_X: 2,
                self.Axis.JOYSTICK_Y: 3,
                self.Axis.JOYSTICK_LB: 4,
                self.Axis.JOYSTICK_RB: 5,
                self.Axis.JOYSTICK_BACK: 6,
                self.Axis.JOYSTICK_START: 7,
                self.Axis.JOYSTICK_LEFT_STICK: 8,
                self.Axis.JOYSTICK_RIGHT_STICK: 9,
                self.Axis.JOYSTICK_DPAD_UP: 10,
                self.Axis.JOYSTICK_DPAD_DOWN: 11,
                self.Axis.JOYSTICK_DPAD_LEFT: 12,
                self.Axis.JOYSTICK_DPAD_RIGHT: 13
            }

            if button in button_mapping:
                button_index = button_mapping[button]
                if button_index < joystick.get_numbuttons():
                    return joystick.get_button(button_index) == 1

        except Exception as e:
            print(f"Joystick button error: {e}")
            return False

        return False

    def _get_mouse_axis_value(self, axis: 'Input.Axis') -> float:
        """Get the value of a specific mouse axis."""
        try:
            if axis == self.Axis.MOUSE_X:
                # Get absolute mouse position
                return pygame.mouse.get_pos()[0]
            elif axis == self.Axis.MOUSE_Y:
                # Get absolute mouse position
                return pygame.mouse.get_pos()[1]
            elif axis == self.Axis.MOUSE_REL_X:
                # Get raw relative mouse movement (not scaled)
                return pygame.mouse.get_rel()[0]
            elif axis == self.Axis.MOUSE_REL_Y:
                # Get raw relative mouse movement (not scaled)
                return pygame.mouse.get_rel()[1]
            elif axis == self.Axis.MOUSE_SCROLL:
                # Get mouse wheel scroll from event system
                scroll_value = 0.0
                if self.event_states[self.InputEvent.MOUSE_SCROLL_UP]:
                    scroll_value += 1.0
                if self.event_states[self.InputEvent.MOUSE_SCROLL_DOWN]:
                    scroll_value -= 1.0
                return scroll_value
        except Exception as e:
            print(f"Mouse input error: {e}")
            return 0.0

        return 0.0

    def process_event(self, event):
        """Process pygame events and store them in event_states."""
        # Update both new enum-based and legacy event states
        if event.type == pygame.MOUSEBUTTONDOWN:
            if event.button == 1:  # Left mouse button
                self.event_states[self.InputEvent.MOUSE_BUTTON_DOWN][0] = True
                self._legacy_event_states['mouse_button_down'][0] = True
            elif event.button == 2:  # Middle mouse button
                self.event_states[self.InputEvent.MOUSE_BUTTON_DOWN][1] = True
                self._legacy_event_states['mouse_button_down'][1] = True
            elif event.button == 3:  # Right mouse button
                self.event_states[self.InputEvent.MOUSE_BUTTON_DOWN][2] = True
                self._legacy_event_states['mouse_button_down'][2] = True
            elif event.button == 4:  # Mouse wheel up
                self.event_states[self.InputEvent.MOUSE_SCROLL_UP] = True
                self._legacy_event_states['mouse_scroll_up'] = True
            elif event.button == 5:  # Mouse wheel down
                self.event_states[self.InputEvent.MOUSE_SCROLL_DOWN] = True
                self._legacy_event_states['mouse_scroll_down'] = True

        elif event.type == pygame.MOUSEBUTTONUP:
            if event.button == 1:  # Left mouse button
                self.event_states[self.InputEvent.MOUSE_BUTTON_UP][0] = True
                self.event_states[self.InputEvent.MOUSE_BUTTON_DOWN][0] = False
                self._legacy_event_states['mouse_button_up'][0] = True
                self._legacy_event_states['mouse_button_down'][0] = False
            elif event.button == 2:  # Middle mouse button
                self.event_states[self.InputEvent.MOUSE_BUTTON_UP][1] = True
                self.event_states[self.InputEvent.MOUSE_BUTTON_DOWN][1] = False
                self._legacy_event_states['mouse_button_up'][1] = True
                self._legacy_event_states['mouse_button_down'][1] = False
            elif event.button == 3:  # Right mouse button
                self.event_states[self.InputEvent.MOUSE_BUTTON_UP][2] = True
                self.event_states[self.InputEvent.MOUSE_BUTTON_DOWN][2] = False
                self._legacy_event_states['mouse_button_up'][2] = True
                self._legacy_event_states['mouse_button_down'][2] = False

        elif event.type == pygame.MOUSEMOTION:
            self.event_states[self.InputEvent.MOUSE_MOTION] = True

        elif event.type == pygame.KEYDOWN:
            self.event_states[self.InputEvent.KEY_DOWN][event.key] = True
            self._legacy_event_states['key_down'][event.key] = True

        elif event.type == pygame.KEYUP:
            self.event_states[self.InputEvent.KEY_UP][event.key] = True
            self.event_states[self.InputEvent.KEY_DOWN][event.key] = False
            self._legacy_event_states['key_up'][event.key] = True
            self._legacy_event_states['key_down'][event.key] = False

        elif event.type == pygame.JOYBUTTONDOWN:
            self.event_states[self.InputEvent.JOYSTICK_BUTTON_DOWN][event.button] = True
            self._legacy_event_states['joystick_button_down'][event.button] = True

        elif event.type == pygame.JOYBUTTONUP:
            self.event_states[self.InputEvent.JOYSTICK_BUTTON_UP][event.button] = True
            self.event_states[self.InputEvent.JOYSTICK_BUTTON_DOWN][event.button] = False
            self._legacy_event_states['joystick_button_up'][event.button] = True
            self._legacy_event_states['joystick_button_down'][event.button] = False

        elif event.type == pygame.JOYAXISMOTION:
            self.event_states[self.InputEvent.JOYSTICK_AXIS_MOTION][event.axis] = event.value
            self._legacy_event_states['joystick_axis_motion'][event.axis] = event.value

        elif event.type == pygame.JOYHATMOTION:
            self.event_states[self.InputEvent.JOYSTICK_HAT_MOTION][event.hat] = event.value

        elif event.type == pygame.JOYBALLMOTION:
            self.event_states[self.InputEvent.JOYSTICK_BALL_MOTION][event.ball] = event.value

        elif event.type == pygame.JOYDEVICEADDED:
            self.event_states[self.InputEvent.JOYSTICK_CONNECTED] = True

        elif event.type == pygame.JOYDEVICEREMOVED:
            self.event_states[self.InputEvent.JOYSTICK_DISCONNECTED] = True

        elif event.type == pygame.WINDOWFOCUSGAINED:
            self.event_states[self.InputEvent.WINDOW_FOCUS_GAINED] = True

        elif event.type == pygame.WINDOWFOCUSLOST:
            self.event_states[self.InputEvent.WINDOW_FOCUS_LOST] = True

        elif event.type == pygame.WINDOWSIZECHANGED:
            self.event_states[self.InputEvent.WINDOW_RESIZED] = True

        elif event.type == pygame.WINDOWMOVED:
            self.event_states[self.InputEvent.WINDOW_MOVED] = True

        elif event.type == pygame.WINDOWMINIMIZED:
            self.event_states[self.InputEvent.WINDOW_MINIMIZED] = True

        elif event.type == pygame.WINDOWRESTORED:
            self.event_states[self.InputEvent.WINDOW_RESTORED] = True

        elif event.type == pygame.QUIT:
            self.event_states[self.InputEvent.QUIT] = True

        elif event.type == pygame.ACTIVEEVENT:
            self.event_states[self.InputEvent.ACTIVEEVENT] = True

        elif event.type == pygame.VIDEORESIZE:
            self.event_states[self.InputEvent.VIDEORESIZE] = True

        elif event.type == pygame.VIDEOEXPOSE:
            self.event_states[self.InputEvent.VIDEOEXPOSE] = True

    def get_event_state(self, event_type, key=None):
        """Get the state of an event-based input using InputEvent enum or legacy string."""
        # Handle enum-based event types
        if isinstance(event_type, self.InputEvent):
            if event_type in self.event_states:
                if isinstance(self.event_states[event_type], dict):
                    return self.event_states[event_type].get(key, False)
                else:
                    return self.event_states[event_type]
            return False

        # Handle legacy string-based event types
        if event_type == 'mouse_scroll_up':
            return self._legacy_event_states['mouse_scroll_up']
        elif event_type == 'mouse_scroll_down':
            return self._legacy_event_states['mouse_scroll_down']
        elif event_type == 'mouse_button_up':
            return self._legacy_event_states['mouse_button_up'].get(key, False)
        elif event_type == 'mouse_button_down':
            return self._legacy_event_states['mouse_button_down'].get(key, False)
        elif event_type == 'key_up':
            return self._legacy_event_states['key_up'].get(key, False)
        elif event_type == 'key_down':
            return self._legacy_event_states['key_down'].get(key, False)
        elif event_type == 'joystick_button_up':
            return self._legacy_event_states['joystick_button_up'].get(key, False)
        elif event_type == 'joystick_button_down':
            return self._legacy_event_states['joystick_button_down'].get(key, False)
        elif event_type == 'joystick_axis_motion':
            return self._legacy_event_states['joystick_axis_motion'].get(key, 0.0)
        return False

    def bind_keys(self, key: Keybind, axis: list[Axis]):
        pass

    def update(self):
        """Update input states - called by the engine each frame."""
        # Update the main mouse interface
        self.mouse.update()

        # Capture mouse relative movement once per frame
        rel_x, rel_y = pygame.mouse.get_rel()
        self._mouse_rel_x = rel_x / 10.0  # Scale down for smoother input
        self._mouse_rel_y = rel_y / 10.0  # Scale down for smoother input

        # Clear event states that should only last one frame
        # New enum-based event states
        self.event_states[self.InputEvent.MOUSE_SCROLL_UP] = False
        self.event_states[self.InputEvent.MOUSE_SCROLL_DOWN] = False
        self.event_states[self.InputEvent.MOUSE_MOTION] = False
        self.event_states[self.InputEvent.MOUSE_ENTER] = False
        self.event_states[self.InputEvent.MOUSE_LEAVE] = False
        self.event_states[self.InputEvent.MOUSE_BUTTON_UP].clear()
        self.event_states[self.InputEvent.KEY_UP].clear()
        self.event_states[self.InputEvent.JOYSTICK_BUTTON_UP].clear()
        self.event_states[self.InputEvent.JOYSTICK_CONNECTED] = False
        self.event_states[self.InputEvent.JOYSTICK_DISCONNECTED] = False
        self.event_states[self.InputEvent.WINDOW_FOCUS_GAINED] = False
        self.event_states[self.InputEvent.WINDOW_FOCUS_LOST] = False
        self.event_states[self.InputEvent.WINDOW_RESIZED] = False
        self.event_states[self.InputEvent.WINDOW_MOVED] = False
        self.event_states[self.InputEvent.WINDOW_MINIMIZED] = False
        self.event_states[self.InputEvent.WINDOW_RESTORED] = False
        self.event_states[self.InputEvent.QUIT] = False
        self.event_states[self.InputEvent.ACTIVEEVENT] = False
        self.event_states[self.InputEvent.VIDEORESIZE] = False
        self.event_states[self.InputEvent.VIDEOEXPOSE] = False
        self.event_states[self.InputEvent.INPUT_DEVICE_CHANGED] = False
        self.event_states[self.InputEvent.INPUT_MAPPING_CHANGED] = False
        self.event_states[self.InputEvent.INPUT_PROFILE_LOADED] = False
        self.event_states[self.InputEvent.INPUT_PROFILE_SAVED] = False

        # Legacy event states
        self._legacy_event_states['mouse_scroll_up'] = False
        self._legacy_event_states['mouse_scroll_down'] = False
        self._legacy_event_states['mouse_button_up'].clear()
        self._legacy_event_states['key_up'].clear()
        self._legacy_event_states['joystick_button_up'].clear()

        # Try to initialize joysticks if not already done
        if pygame.joystick.get_count() > 0 and not pygame.joystick.get_init():
            self.try_joystick_init()


