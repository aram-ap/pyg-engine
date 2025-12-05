try:
    from pyg import _native
except ImportError:
    # Fallback for direct import or weird paths
    import _native

LogType = _native.LogType

class Input:
    def __init__(self):
        self._input_manager = _native.InputManager()

    def update(self):
        self._input_manager.update()

    def set_mouse_position(self, x: int, y: int):
        self._input_manager.set_mouse_position(x, y)

    def get_mouse_x(self) -> int:
        return self._input_manager.get_mouse_x()

    def get_mouse_y(self) -> int:
        return self._input_manager.get_mouse_y()

    def is_mouse_button_pressed(self, button: int) -> bool:
        return self._input_manager.is_mouse_button_pressed(button)

    def is_mouse_button_released(self, button: int) -> bool:
        return self._input_manager.is_mouse_button_released(button)

    def is_mouse_button_down(self, button: int) -> bool:
        return self._input_manager.is_mouse_button_down(button)

    def is_mouse_button_up(self, button: int) -> bool:
        return self._input_manager.is_mouse_button_up(button)

    def is_key_pressed(self, key: int) -> bool:
        return self._input_manager.is_key_pressed(key)

    def is_key_released(self, key: int) -> bool:
        return self._input_manager.is_key_released(key)

    def is_key_down(self, key: int) -> bool:
        return self._input_manager.is_key_down(key)

    def is_key_up(self, key: int) -> bool:
        return self._input_manager.is_key_up(key)

    @property
    def mouse_position(self) -> (int, int):
        return self._input_manager.get_mouse_x(), self._input_manager.get_mouse_y()
