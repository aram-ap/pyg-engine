try:
    from pyg import _native
except ImportError:
    # Fallback for direct import or weird paths
    import _native

from pyg import Math

class Color(_native.Color):
    def __init__(self, r: int, g: int, b: int, a: int = 255):
        super().__init__(Math.clamp_int(r, 0, 255), Math.clamp_int(g, 0, 255), Math.clamp_int(b, 0, 255), Math.clamp_int(a, 0, 255))

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
