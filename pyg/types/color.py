try:
    from pyg import _native
except ImportError:
    # Fallback for direct import or weird paths
    import _native

# Use _native.Math to avoid circular import issues
Math = _native.Math

class Color(_native.Color):
    """Color class with RGBA components (0-255).
    
    Supports arithmetic operations and provides pre-defined color constants.
    Component values are automatically clamped to 0-255 range and converted to integers.
    """
    
    def __init__(self, r: int, g: int, b: int, a: int = 255) -> None:
        """Initialize a Color with RGBA values.
        
        Args:
            r: Red component (0-255)
            g: Green component (0-255)
            b: Blue component (0-255)
            a: Alpha component (0-255), defaults to 255 (opaque)
        """
        super().__init__(
            Math.clamp_int(int(r), 0, 255),
            Math.clamp_int(int(g), 0, 255),
            Math.clamp_int(int(b), 0, 255),
            Math.clamp_int(int(a), 0, 255)
        )
    
    @property
    def r(self) -> int:
        """Get the red component (0-255)."""
        return _native.Color.r.fget(self)
    
    @r.setter
    def r(self, value: float | int) -> None:
        """Set the red component (0-255), automatically clamped and converted to int."""
        _native.Color.r.fset(self, Math.clamp_int(int(value), 0, 255))
    
    @property
    def g(self) -> int:
        """Get the green component (0-255)."""
        return _native.Color.g.fget(self)
    
    @g.setter
    def g(self, value: float | int) -> None:
        """Set the green component (0-255), automatically clamped and converted to int."""
        _native.Color.g.fset(self, Math.clamp_int(int(value), 0, 255))
    
    @property
    def b(self) -> int:
        """Get the blue component (0-255)."""
        return _native.Color.b.fget(self)
    
    @b.setter
    def b(self, value: float | int) -> None:
        """Set the blue component (0-255), automatically clamped and converted to int."""
        _native.Color.b.fset(self, Math.clamp_int(int(value), 0, 255))
    
    @property
    def a(self) -> int:
        """Get the alpha component (0-255)."""
        return _native.Color.a.fget(self)
    
    @a.setter
    def a(self, value: float | int) -> None:
        """Set the alpha component (0-255), automatically clamped and converted to int."""
        _native.Color.a.fset(self, Math.clamp_int(int(value), 0, 255))

Color.WHITE       = Color(255, 255, 255, 255)
Color.BLACK       = Color(0, 0, 0, 255)
Color.RED         = Color(255, 0, 0, 255)
Color.GREEN       = Color(0, 255, 0, 255)
Color.BLUE        = Color(0, 0, 255, 255)
Color.YELLOW      = Color(255, 255, 0, 255)
Color.CYAN        = Color(0, 255, 255, 255)
Color.MAGENTA     = Color(255, 0, 255, 255)
Color.TRANSPARENT = Color(0, 0, 0, 0)
Color.GRAY        = Color(128, 128, 128, 255)
Color.ORANGE      = Color(255, 165, 0, 255)
Color.PURPLE      = Color(128, 0, 128, 255)
Color.BROWN       = Color(165, 42, 42, 255)
Color.PINK        = Color(255, 192, 203, 255)
Color.LIGHT_GRAY  = Color(211, 211, 211, 255)
Color.DARK_GRAY   = Color(169, 169, 169, 255)
Color.LIGHT_RED   = Color(255, 102, 102, 255)
Color.LIGHT_GREEN = Color(102, 255, 102, 255)
Color.LIGHT_BLUE  = Color(102, 102, 255, 255)
Color.DARK_RED    = Color(139, 0, 0, 255)
Color.DARK_GREEN  = Color(0, 100, 0, 255)
Color.DARK_BLUE   = Color(0, 0, 139, 255)
Color.GOLD        = Color(255, 215, 0, 255)
Color.SILVER      = Color(192, 192, 192, 255)
Color.BRONZE      = Color(205, 127, 50, 255)
Color.NAVY        = Color(0, 0, 128, 255)
Color.TEAL        = Color(0, 128, 128, 255)
Color.OLIVE       = Color(128, 128, 0, 255)
Color.MAROON      = Color(128, 0, 0, 255)
Color.LIME        = Color(0, 255, 0, 255)
Color.AQUA        = Color(0, 255, 255, 255)
Color.FUCHSIA     = Color(255, 0, 255, 255)
Color.CORAL       = Color(255, 127, 80, 255)
Color.SALMON      = Color(250, 128, 114, 255)
Color.KHAKI       = Color(240, 230, 140, 255)
Color.VIOLET      = Color(238, 130, 238, 255)
Color.INDIGO      = Color(75, 0, 130, 255)
Color.TURQUOISE   = Color(64, 224, 208, 255)
Color.BEIGE       = Color(245, 245, 220, 255)
