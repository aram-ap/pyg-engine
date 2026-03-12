from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Iterable, Sequence

from .pyg_engine_native import DrawCommand as _RustDrawCommand
from .pyg_engine_native import MeshGeometry as _RustMeshGeometry


PointLike = Any


def _xy(point: PointLike) -> tuple[float, float]:
    if hasattr(point, "x") and hasattr(point, "y"):
        return float(point.x), float(point.y)
    if isinstance(point, Sequence) and len(point) == 2:
        return float(point[0]), float(point[1])
    raise TypeError(f"Expected a Vec2-like value, got {type(point).__name__}")


@dataclass(slots=True)
class Line:
    start: PointLike
    end: PointLike
    color: Any
    thickness: float = 1.0
    draw_order: float = 0.0

    def to_draw_command(self) -> Any:
        start_x, start_y = _xy(self.start)
        end_x, end_y = _xy(self.end)
        return _RustDrawCommand.line(
            start_x,
            start_y,
            end_x,
            end_y,
            self.color,
            thickness=self.thickness,
            draw_order=self.draw_order,
        )


@dataclass(slots=True)
class Rect:
    position: PointLike
    width: float
    height: float
    color: Any
    filled: bool = True
    thickness: float = 1.0
    draw_order: float = 0.0

    def to_draw_command(self) -> Any:
        x, y = _xy(self.position)
        return _RustDrawCommand.rectangle(
            x,
            y,
            self.width,
            self.height,
            self.color,
            filled=self.filled,
            thickness=self.thickness,
            draw_order=self.draw_order,
        )


@dataclass(slots=True)
class Circle:
    position: PointLike
    radius: float
    color: Any
    filled: bool = True
    thickness: float = 1.0
    segments: int = 32
    draw_order: float = 0.0

    def to_draw_command(self) -> Any:
        x, y = _xy(self.position)
        return _RustDrawCommand.circle(
            x,
            y,
            self.radius,
            self.color,
            filled=self.filled,
            thickness=self.thickness,
            segments=self.segments,
            draw_order=self.draw_order,
        )


@dataclass(slots=True)
class Arc:
    position: PointLike
    radius: float
    start_angle: float
    end_angle: float
    color: Any
    filled: bool = True
    thickness: float = 1.0
    segments: int = 32
    draw_order: float = 0.0

    def to_draw_command(self) -> Any:
        x, y = _xy(self.position)
        return _RustDrawCommand.arc(
            x,
            y,
            self.radius,
            self.start_angle,
            self.end_angle,
            self.color,
            filled=self.filled,
            thickness=self.thickness,
            segments=self.segments,
            draw_order=self.draw_order,
        )


@dataclass(slots=True)
class Polygon:
    points: Sequence[PointLike]
    color: Any
    filled: bool = True
    thickness: float = 1.0
    draw_order: float = 0.0

    def to_draw_command(self) -> Any:
        return _RustDrawCommand.polygon(
            [_xy(point) for point in self.points],
            self.color,
            filled=self.filled,
            thickness=self.thickness,
            draw_order=self.draw_order,
        )


@dataclass(slots=True)
class Mesh:
    vertices: Sequence[PointLike]
    indices: Sequence[int]
    color: Any
    texture_path: str | None = None
    uvs: Sequence[PointLike] | None = None
    draw_order: float = 0.0

    def to_draw_command(self) -> Any:
        return _RustDrawCommand.mesh(
            [_xy(vertex) for vertex in self.vertices],
            list(self.indices),
            self.color,
            texture_path=self.texture_path,
            uvs=None if self.uvs is None else [_xy(uv) for uv in self.uvs],
            draw_order=self.draw_order,
        )

    @staticmethod
    def rect(width: float, height: float) -> Any:
        return _RustMeshGeometry.rectangle(width, height)

    Rect = rect

    @staticmethod
    def circle(radius: float, segments: int = 32) -> Any:
        return _RustMeshGeometry.circle(radius, segments)

    Circle = circle


@dataclass(slots=True)
class Text:
    text: str
    position: PointLike
    color: Any
    font_size: float = 24.0
    font_path: str | None = None
    letter_spacing: float = 0.0
    line_spacing: float = 0.0
    draw_order: float = 0.0

    def to_draw_command(self) -> Any:
        x, y = _xy(self.position)
        return _RustDrawCommand.text(
            self.text,
            x,
            y,
            self.color,
            font_size=self.font_size,
            font_path=self.font_path,
            letter_spacing=self.letter_spacing,
            line_spacing=self.line_spacing,
            draw_order=self.draw_order,
        )


ShapeLike = Any


def to_draw_command(shape: ShapeLike) -> Any:
    if isinstance(shape, _RustDrawCommand):
        return shape
    if hasattr(shape, "to_draw_command"):
        return shape.to_draw_command()
    raise TypeError(
        "Expected a draw shape or DrawCommand instance, "
        f"got {type(shape).__name__}"
    )


def to_draw_commands(drawable: ShapeLike | Iterable[ShapeLike]) -> list[Any]:
    if isinstance(drawable, Iterable) and not isinstance(
        drawable, (str, bytes, _RustDrawCommand)
    ):
        if hasattr(drawable, "to_draw_command"):
            return [to_draw_command(drawable)]
        return [to_draw_command(shape) for shape in drawable]
    return [to_draw_command(drawable)]
