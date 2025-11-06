"""
This is a wrapper for Pygame's Color class, there's no real logic embedded in this.
"""

import pygame
from typing import Union, Tuple, Optional, Iterator, overload

ColorValue = Union[
    "Color",
    pygame.Color,
    Tuple[int, int, int],
    Tuple[int, int, int, int],
    str,
    int,
]


class Color(pygame.Color):
    """
    Complete wrapper around pygame.Color with additional utilities.

    Represents an RGBA color with values from 0-255.
    Supports multiple construction formats:
        - Color(r, g, b, a=255)
        - Color("#RRGGBB") or Color("#RRGGBBAA")
        - Color("red") or other named colors
        - Color(0xRRGGBBAA) as integer
        - Color(another_color)

    Attributes:
        r (int): Red component (0-255)
        g (int): Green component (0-255)
        b (int): Blue component (0-255)
        a (int): Alpha/opacity (0=transparent, 255=opaque)
        hsva: Hue, Saturation, Value, Alpha representation
        hsla: Hue, Saturation, Lightness, Alpha representation
        cmy: Cyan, Magenta, Yellow representation
        i1i2i3: I1, I2, I3 color space representation
    """

    # CONSTRUCTORS & FACTORY METHODS
    def __init__(
        self,
        r: Union[int, str, ColorValue],
        g: Optional[int] = None,
        b: Optional[int] = None,
        a: int = 255,
    ):
        """
        Initialize a Color.

        Args:
            r: Red value (0-255), or color name, or hex string, or Color
            g: Green value (0-255)
            b: Blue value (0-255)
            a: Alpha value (0-255), default 255
        """
        if g is None and b is None:
            # Single argument: name, hex, or color object
            super().__init__(r)
        elif g is not None and b is not None:
            super().__init__(r, g, b, a)
        else:
            super().__init__(r)

    @classmethod
    def from_hex(cls, hex_str: str) -> "Color":
        """
        Create Color from hex string.

        Args:
            hex_str: Hex color like "#FF8800" or "#FF8800AA"

        Returns:
            New Color instance

        Example:
            >>> Color.from_hex("#ff0000")
            Color(255, 0, 0, 255)
        """
        hex_str = hex_str.lstrip("#")

        # Handle short forms: #RGB or #RGBA
        if len(hex_str) in (3, 4):
            hex_str = "".join(ch * 2 for ch in hex_str)

        r = int(hex_str[0:2], 16)
        g = int(hex_str[2:4], 16)
        b = int(hex_str[4:6], 16)
        a = int(hex_str[6:8], 16) if len(hex_str) == 8 else 255

        return cls(r, g, b, a)

    @classmethod
    def from_hsv(cls, h: float, s: float, v: float, a: int = 255) -> "Color":
        """
        Create Color from HSV (Hue, Saturation, Value) values.

        Args:
            h: Hue (0-360)
            s: Saturation (0-100)
            v: Value/Brightness (0-100)
            a: Alpha (0-255)

        Returns:
            New Color instance
        """
        c = cls(0, 0, 0, a)
        c.hsva = (h, s, v, a * 100 / 255)
        return c

    @classmethod
    def from_hsl(cls, h: float, s: float, l: float, a: int = 255) -> "Color":
        """
        Create Color from HSL (Hue, Saturation, Lightness) values.

        Args:
            h: Hue (0-360)
            s: Saturation (0-100)
            l: Lightness (0-100)
            a: Alpha (0-255)

        Returns:
            New Color instance
        """
        c = cls(0, 0, 0, a)
        c.hsla = (h, s, l, a * 100 / 255)
        return c

    @classmethod
    def from_normalized(cls, r: float, g: float, b: float, a: float = 1.0) -> "Color":
        """
        Create Color from normalized float values (0.0-1.0).

        Args:
            r: Red (0.0-1.0)
            g: Green (0.0-1.0)
            b: Blue (0.0-1.0)
            a: Alpha (0.0-1.0)

        Returns:
            New Color instance
        """
        return cls(
            int(r * 255),
            int(g * 255),
            int(b * 255),
            int(a * 255),
        )

    @classmethod
    def from_cmy(cls, c: float, m: float, y: float, a: int = 255) -> "Color":
        """
        Create Color from CMY (Cyan, Magenta, Yellow) values.

        Args:
            c: Cyan (0-100)
            m: Magenta (0-100)
            y: Yellow (0-100)
            a: Alpha (0-255)

        Returns:
            New Color instance
        """
        color = cls(0, 0, 0, a)
        color.cmy = (c, m, y)
        return color

    # CONVERSION METHODS
    def to_hex(self, include_alpha: bool = False) -> str:
        """
        Convert to hex string.

        Args:
            include_alpha: Whether to include alpha channel

        Returns:
            Hex string like "#FF8800" or "#FF8800AA"

        Example:
            >>> Color(255, 136, 0).to_hex()
            '#FF8800'
        """
        if include_alpha:
            return f"#{self.r:02X}{self.g:02X}{self.b:02X}{self.a:02X}"
        return f"#{self.r:02X}{self.g:02X}{self.b:02X}"

    def to_tuple(self, include_alpha: bool = True) -> Union[Tuple[int, int, int], Tuple[int, int, int, int]]:
        """
        Convert to tuple.

        Args:
            include_alpha: Whether to include alpha channel

        Returns:
            (r, g, b) or (r, g, b, a)
        """
        if include_alpha:
            return (self.r, self.g, self.b, self.a)
        return (self.r, self.g, self.b)

    def to_normalized(self, include_alpha: bool = True) -> Union[Tuple[float, float, float], Tuple[float, float, float, float]]:
        """
        Convert to normalized float tuple (0.0-1.0).

        Args:
            include_alpha: Whether to include alpha channel

        Returns:
            Tuple of floats in range 0.0-1.0
        """
        if include_alpha:
            return (self.r / 255, self.g / 255, self.b / 255, self.a / 255)
        return (self.r / 255, self.g / 255, self.b / 255)

    def to_int(self) -> int:
        """
        Convert to 32-bit integer (0xRRGGBBAA).

        Returns:
            Integer representation
        """
        return (self.r << 24) | (self.g << 16) | (self.b << 8) | self.a

    # COLOR MANIPULATION METHODS
    def lerp(self, other: ColorValue, t: float) -> "Color":
        """
        Linear interpolation between this color and another.

        Args:
            other: Target color
            t: Interpolation factor (0.0-1.0)

        Returns:
            New interpolated Color

        Example:
            >>> red = Color(255, 0, 0)
            >>> blue = Color(0, 0, 255)
            >>> red.lerp(blue, 0.5)
            Color(127, 0, 127, 255)
        """
        result = super().lerp(other, t)
        return Color(result.r, result.g, result.b, result.a)

    def grayscale(self) -> "Color":
        """
        Convert to grayscale using luminosity method.

        Returns:
            New grayscale Color
        """
        result = pygame.Color.grayscale(self)
        return Color(result.r, result.g, result.b, result.a)

    def correct_gamma(self, gamma: float) -> "Color":
        """
        Apply gamma correction.

        Args:
            gamma: Gamma value (< 1.0 brightens, > 1.0 darkens)

        Returns:
            New gamma-corrected Color
        """
        result = pygame.Color.correct_gamma(self, gamma)
        return Color(result.r, result.g, result.b, result.a)

    def premul_alpha(self) -> "Color":
        """
        Pre-multiply RGB by alpha channel.

        Returns:
            New Color with pre-multiplied alpha
        """
        result = pygame.Color.premul_alpha(self)
        return Color(result.r, result.g, result.b, result.a)

    def invert(self, invert_alpha: bool = False) -> "Color":
        """
        Invert the color.

        Args:
            invert_alpha: Whether to also invert alpha channel

        Returns:
            New inverted Color
        """
        a = 255 - self.a if invert_alpha else self.a
        return Color(255 - self.r, 255 - self.g, 255 - self.b, a)

    def lighten(self, amount: float) -> "Color":
        """
        Lighten the color.

        Args:
            amount: Amount to lighten (0.0-1.0)

        Returns:
            New lightened Color
        """
        return self.lerp(Color(255, 255, 255, self.a), amount)

    def darken(self, amount: float) -> "Color":
        """
        Darken the color.

        Args:
            amount: Amount to darken (0.0-1.0)

        Returns:
            New darkened Color
        """
        return self.lerp(Color(0, 0, 0, self.a), amount)

    def saturate(self, amount: float) -> "Color":
        """
        Increase saturation.

        Args:
            amount: Amount to increase (0.0-1.0 adds percentage)

        Returns:
            New saturated Color
        """
        h, s, l, a = self.hsla
        new_s = min(100, s + (amount * 100))
        return Color.from_hsl(h, new_s, l, int(a * 255 / 100))

    def desaturate(self, amount: float) -> "Color":
        """
        Decrease saturation.

        Args:
            amount: Amount to decrease (0.0-1.0 removes percentage)

        Returns:
            New desaturated Color
        """
        h, s, l, a = self.hsla
        new_s = max(0, s - (amount * 100))
        return Color.from_hsl(h, new_s, l, int(a * 255 / 100))

    def with_alpha(self, alpha: int) -> "Color":
        """
        Create new Color with different alpha.

        Args:
            alpha: New alpha value (0-255)

        Returns:
            New Color with specified alpha
        """
        return Color(self.r, self.g, self.b, alpha)

    def with_opacity(self, opacity: float) -> "Color":
        """
        Create new Color with opacity percentage.

        Args:
            opacity: Opacity (0.0=transparent, 1.0=opaque)

        Returns:
            New Color with specified opacity
        """
        return Color(self.r, self.g, self.b, int(opacity * 255))

    # ANALYSIS METHODS
    def luminance(self) -> float:
        """
        Calculate relative luminance (0.0-1.0) using ITU-R BT.709.

        Returns:
            Luminance value
        """
        r, g, b = self.to_normalized(include_alpha=False)
        return 0.2126 * r + 0.7152 * g + 0.0722 * b

    def brightness(self) -> float:
        """
        Calculate perceived brightness (0.0-1.0).

        Returns:
            Brightness value
        """
        return (self.r * 299 + self.g * 587 + self.b * 114) / 255000

    def is_light(self, threshold: float = 0.5) -> bool:
        """
        Check if color is light.

        Args:
            threshold: Brightness threshold (0.0-1.0)

        Returns:
            True if color is light
        """
        return self.brightness() > threshold

    def is_dark(self, threshold: float = 0.5) -> bool:
        """
        Check if color is dark.

        Args:
            threshold: Brightness threshold (0.0-1.0)

        Returns:
            True if color is dark
        """
        return self.brightness() <= threshold

    def contrast_ratio(self, other: ColorValue) -> float:
        """
        Calculate WCAG contrast ratio with another color.

        Args:
            other: Color to compare against

        Returns:
            Contrast ratio (1.0-21.0)
        """
        if not isinstance(other, Color):
            other = Color(other)

        l1 = self.luminance()
        l2 = other.luminance()

        lighter = max(l1, l2)
        darker = min(l1, l2)

        return (lighter + 0.05) / (darker + 0.05)

    def distance_to(self, other: ColorValue) -> float:
        """
        Calculate Euclidean distance to another color in RGB space.

        Args:
            other: Color to measure distance to

        Returns:
            Distance value (0-441.67 for RGB)
        """
        if not isinstance(other, Color):
            other = Color(other)

        dr = self.r - other.r
        dg = self.g - other.g
        db = self.b - other.b

        return (dr * dr + dg * dg + db * db) ** 0.5

    # COMPLEMENTARY & HARMONY METHODS
    def complementary(self) -> "Color":
        """
        Get complementary color (opposite on color wheel).

        Returns:
            Complementary Color
        """
        h, s, v, a = self.hsva
        return Color.from_hsv((h + 180) % 360, s, v, int(a * 255 / 100))

    def triadic(self) -> Tuple["Color", "Color"]:
        """
        Get triadic color harmony (120° apart on color wheel).

        Returns:
            Tuple of two complementary Colors
        """
        h, s, v, a = self.hsva
        alpha = int(a * 255 / 100)
        return (
            Color.from_hsv((h + 120) % 360, s, v, alpha),
            Color.from_hsv((h + 240) % 360, s, v, alpha),
        )

    def split_complementary(self) -> Tuple["Color", "Color"]:
        """
        Get split-complementary colors (±150° from complementary).

        Returns:
            Tuple of two Colors
        """
        h, s, v, a = self.hsva
        alpha = int(a * 255 / 100)
        return (
            Color.from_hsv((h + 150) % 360, s, v, alpha),
            Color.from_hsv((h + 210) % 360, s, v, alpha),
        )

    def analogous(self, angle: float = 30) -> Tuple["Color", "Color"]:
        """
        Get analogous colors (adjacent on color wheel).

        Args:
            angle: Degrees offset (default 30)

        Returns:
            Tuple of two analogous Colors
        """
        h, s, v, a = self.hsva
        alpha = int(a * 255 / 100)
        return (
            Color.from_hsv((h - angle) % 360, s, v, alpha),
            Color.from_hsv((h + angle) % 360, s, v, alpha),
        )

    def tetradic(self) -> Tuple["Color", "Color", "Color"]:
        """
        Get tetradic/square color harmony (90° apart).

        Returns:
            Tuple of three Colors
        """
        h, s, v, a = self.hsva
        alpha = int(a * 255 / 100)
        return (
            Color.from_hsv((h + 90) % 360, s, v, alpha),
            Color.from_hsv((h + 180) % 360, s, v, alpha),
            Color.from_hsv((h + 270) % 360, s, v, alpha),
        )

    # UTILITY METHODS
    def copy(self) -> "Color":
        """
        Create a copy of this Color.

        Returns:
            New Color instance with same values
        """
        return Color(self.r, self.g, self.b, self.a)

    def clamp(self) -> "Color":
        """
        Ensure all components are in valid range (0-255).

        Returns:
            New clamped Color
        """
        return Color(
            max(0, min(255, self.r)),
            max(0, min(255, self.g)),
            max(0, min(255, self.b)),
            max(0, min(255, self.a)),
        )

    # Magic cases

    def __repr__(self) -> str:
        """String representation."""
        return f"Color({self.r}, {self.g}, {self.b}, {self.a})"

    def __str__(self) -> str:
        """String conversion."""
        return self.to_hex(include_alpha=True)

    def __iter__(self) -> Iterator[int]:
        """Iterate over RGBA components."""
        return iter((self.r, self.g, self.b, self.a))

    def __getitem__(self, index: int) -> int:
        """Get component by index (0=r, 1=g, 2=b, 3=a)."""
        return super().__getitem__(index)

    def __setitem__(self, index: int, value: int) -> None:
        """Set component by index."""
        super().__setitem__(index, value)

    def __eq__(self, other: object) -> bool:
        """Check equality with another Color."""
        if not isinstance(other, (Color, pygame.Color)):
            return False
        return (self.r, self.g, self.b, self.a) == (other.r, other.g, other.b, other.a)

    def __ne__(self, other: object) -> bool:
        """Check inequality."""
        return not self.__eq__(other)

    def __hash__(self) -> int:
        """Hash for use in sets/dicts."""
        return hash((self.r, self.g, self.b, self.a))

    def __add__(self, other: ColorValue) -> "Color":
        """
        Add two colors component-wise (clamped to 255).

        Example:
            >>> Color(100, 50, 0) + Color(50, 100, 0)
            Color(150, 150, 0, 255)
        """
        if not isinstance(other, (Color, pygame.Color)):
            other = Color(other)
        return Color(
            min(255, self.r + other.r),
            min(255, self.g + other.g),
            min(255, self.b + other.b),
            min(255, self.a + other.a),
        )

    def __sub__(self, other: ColorValue) -> "Color":
        """
        Subtract colors component-wise (clamped to 0).
        """
        if not isinstance(other, (Color, pygame.Color)):
            other = Color(other)
        return Color(
            max(0, self.r - other.r),
            max(0, self.g - other.g),
            max(0, self.b - other.b),
            max(0, self.a - other.a),
        )

    def __mul__(self, scalar: float) -> "Color":
        """
        Multiply color by scalar (clamped to 0-255).

        Example:
            >>> Color(100, 50, 25) * 1.5
            Color(150, 75, 37, 255)
        """
        return Color(
            int(max(0, min(255, self.r * scalar))),
            int(max(0, min(255, self.g * scalar))),
            int(max(0, min(255, self.b * scalar))),
            self.a,
        )

    def __truediv__(self, scalar: float) -> "Color":
        """Divide color by scalar."""
        if scalar == 0:
            raise ValueError("Cannot divide color by zero")
        return self.__mul__(1.0 / scalar)

    def __len__(self) -> int:
        """Length is always 4 (RGBA)."""
        return 4


# PREDEFINED COLORS (Common named colors)
class Colors:
    """Collection of common predefined colors."""

    # Basic colors
    BLACK = Color(0, 0, 0)
    WHITE = Color(255, 255, 255)
    RED = Color(255, 0, 0)
    GREEN = Color(0, 255, 0)
    BLUE = Color(0, 0, 255)
    YELLOW = Color(255, 255, 0)
    CYAN = Color(0, 255, 255)
    MAGENTA = Color(255, 0, 255)

    # Grayscale
    GRAY = Color(128, 128, 128)
    LIGHT_GRAY = Color(192, 192, 192)
    DARK_GRAY = Color(64, 64, 64)

    # Extended colors
    ORANGE = Color(255, 165, 0)
    PURPLE = Color(128, 0, 128)
    PINK = Color(255, 192, 203)
    BROWN = Color(165, 42, 42)

    # Transparent
    TRANSPARENT = Color(0, 0, 0, 0)

    @classmethod
    def random(cls, include_alpha: bool = False, min_alpha: int = 128) -> Color:
        """
        Generate a random color.

        Args:
            include_alpha: Whether to randomize alpha
            min_alpha: Minimum alpha value if randomizing

        Returns:
            Random Color
        """
        import random
        r = random.randint(0, 255)
        g = random.randint(0, 255)
        b = random.randint(0, 255)
        a = random.randint(min_alpha, 255) if include_alpha else 255
        return Color(r, g, b, a)

