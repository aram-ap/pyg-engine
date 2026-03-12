#!/usr/bin/env python3
"""
Shared fixed-scene benchmark utilities for pyg_engine and pygame.
"""

from __future__ import annotations

import csv
import json
import math
import random
import statistics
from dataclasses import asdict, dataclass
from datetime import datetime, timezone
from pathlib import Path


@dataclass
class BenchmarkConfig:
    width: int = 1920
    height: int = 1080
    duration_s: float = 20.0
    seed: int = 1337
    log_interval_s: float = 1.0
    rect_count: int = 2200
    circle_count: int = 1800
    line_count: int = 1000
    polygon_count: int = 600
    output_dir: str = "benchmark_logs"
    benchmark_name: str = "fixed_scene_v1"


@dataclass
class MovingRect:
    x: float
    y: float
    w: float
    h: float
    vx: float
    vy: float
    color: tuple[int, int, int]


@dataclass
class MovingCircle:
    x: float
    y: float
    radius: float
    vx: float
    vy: float
    color: tuple[int, int, int]


@dataclass
class MovingLine:
    x: float
    y: float
    half_len: float
    angle: float
    angular_vel: float
    vx: float
    vy: float
    thickness: int
    color: tuple[int, int, int]


@dataclass
class MovingPolygon:
    x: float
    y: float
    radius: float
    sides: int
    angle: float
    angular_vel: float
    vx: float
    vy: float
    color: tuple[int, int, int]


@dataclass
class SceneState:
    rects: list[MovingRect]
    circles: list[MovingCircle]
    lines: list[MovingLine]
    polygons: list[MovingPolygon]

    @property
    def total_objects(self) -> int:
        return len(self.rects) + len(self.circles) + len(self.lines) + len(self.polygons)


def _random_color(rng: random.Random) -> tuple[int, int, int]:
    return (
        rng.randint(40, 255),
        rng.randint(40, 255),
        rng.randint(40, 255),
    )


def create_scene(config: BenchmarkConfig) -> SceneState:
    rng = random.Random(config.seed)

    rects: list[MovingRect] = []
    for _ in range(config.rect_count):
        w = rng.uniform(8.0, 26.0)
        h = rng.uniform(8.0, 26.0)
        rects.append(
            MovingRect(
                x=rng.uniform(0.0, config.width - w),
                y=rng.uniform(0.0, config.height - h),
                w=w,
                h=h,
                vx=rng.uniform(-240.0, 240.0),
                vy=rng.uniform(-240.0, 240.0),
                color=_random_color(rng),
            )
        )

    circles: list[MovingCircle] = []
    for _ in range(config.circle_count):
        radius = rng.uniform(4.0, 14.0)
        circles.append(
            MovingCircle(
                x=rng.uniform(radius, config.width - radius),
                y=rng.uniform(radius, config.height - radius),
                radius=radius,
                vx=rng.uniform(-260.0, 260.0),
                vy=rng.uniform(-260.0, 260.0),
                color=_random_color(rng),
            )
        )

    lines: list[MovingLine] = []
    for _ in range(config.line_count):
        lines.append(
            MovingLine(
                x=rng.uniform(0.0, config.width),
                y=rng.uniform(0.0, config.height),
                half_len=rng.uniform(8.0, 18.0),
                angle=rng.uniform(0.0, math.tau),
                angular_vel=rng.uniform(-2.6, 2.6),
                vx=rng.uniform(-280.0, 280.0),
                vy=rng.uniform(-280.0, 280.0),
                thickness=rng.randint(1, 3),
                color=_random_color(rng),
            )
        )

    polygons: list[MovingPolygon] = []
    for _ in range(config.polygon_count):
        polygons.append(
            MovingPolygon(
                x=rng.uniform(0.0, config.width),
                y=rng.uniform(0.0, config.height),
                radius=rng.uniform(7.0, 18.0),
                sides=rng.randint(3, 7),
                angle=rng.uniform(0.0, math.tau),
                angular_vel=rng.uniform(-2.0, 2.0),
                vx=rng.uniform(-220.0, 220.0),
                vy=rng.uniform(-220.0, 220.0),
                color=_random_color(rng),
            )
        )

    return SceneState(rects=rects, circles=circles, lines=lines, polygons=polygons)


def _bounce(value: float, velocity: float, lower: float, upper: float) -> tuple[float, float]:
    if value < lower:
        return lower, abs(velocity)
    if value > upper:
        return upper, -abs(velocity)
    return value, velocity


def update_scene(scene: SceneState, dt: float, width: int, height: int) -> None:
    for obj in scene.rects:
        obj.x += obj.vx * dt
        obj.y += obj.vy * dt
        obj.x, obj.vx = _bounce(obj.x, obj.vx, 0.0, width - obj.w)
        obj.y, obj.vy = _bounce(obj.y, obj.vy, 0.0, height - obj.h)

    for obj in scene.circles:
        obj.x += obj.vx * dt
        obj.y += obj.vy * dt
        obj.x, obj.vx = _bounce(obj.x, obj.vx, obj.radius, width - obj.radius)
        obj.y, obj.vy = _bounce(obj.y, obj.vy, obj.radius, height - obj.radius)

    for obj in scene.lines:
        obj.x += obj.vx * dt
        obj.y += obj.vy * dt
        obj.angle += obj.angular_vel * dt
        margin = obj.half_len + 1.0
        obj.x, obj.vx = _bounce(obj.x, obj.vx, margin, width - margin)
        obj.y, obj.vy = _bounce(obj.y, obj.vy, margin, height - margin)

    for obj in scene.polygons:
        obj.x += obj.vx * dt
        obj.y += obj.vy * dt
        obj.angle += obj.angular_vel * dt
        margin = obj.radius + 1.0
        obj.x, obj.vx = _bounce(obj.x, obj.vx, margin, width - margin)
        obj.y, obj.vy = _bounce(obj.y, obj.vy, margin, height - margin)


def polygon_points(obj: MovingPolygon) -> list[tuple[float, float]]:
    points: list[tuple[float, float]] = []
    step = math.tau / obj.sides
    for idx in range(obj.sides):
        theta = obj.angle + (idx * step)
        px = obj.x + math.cos(theta) * obj.radius
        py = obj.y + math.sin(theta) * obj.radius
        points.append((px, py))
    return points


def line_endpoints(obj: MovingLine) -> tuple[float, float, float, float]:
    dx = math.cos(obj.angle) * obj.half_len
    dy = math.sin(obj.angle) * obj.half_len
    return (obj.x - dx, obj.y - dy, obj.x + dx, obj.y + dy)


def _percentile(values: list[float], fraction: float) -> float:
    if not values:
        return 0.0
    if len(values) == 1:
        return values[0]
    ordered = sorted(values)
    pos = fraction * (len(ordered) - 1)
    low = int(math.floor(pos))
    high = int(math.ceil(pos))
    if low == high:
        return ordered[low]
    weight = pos - low
    return ordered[low] * (1.0 - weight) + ordered[high] * weight


class BenchmarkLogger:
    def __init__(
        self,
        *,
        engine_name: str,
        config: BenchmarkConfig,
    ) -> None:
        self.engine_name = engine_name
        self.config = config
        self.timestamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")

        self.base_dir = Path(config.output_dir).resolve() / config.benchmark_name
        self.base_dir.mkdir(parents=True, exist_ok=True)

        prefix = f"{engine_name}_{self.timestamp}"
        self.csv_path = self.base_dir / f"{prefix}.csv"
        self.summary_path = self.base_dir / f"{prefix}.json"

        self._csv_file = self.csv_path.open("w", newline="", encoding="utf-8")
        self._writer = csv.writer(self._csv_file)
        self._writer.writerow(
            [
                "frame",
                "elapsed_s",
                "dt_s",
                "update_ms",
                "draw_submit_ms",
                "present_ms",
                "frame_ms",
                "fps",
                "rect_count",
                "circle_count",
                "line_count",
                "polygon_count",
                "total_objects",
            ]
        )

        self.frame_times_ms: list[float] = []
        self.fps_values: list[float] = []
        self.update_ms_values: list[float] = []
        self.draw_ms_values: list[float] = []
        self.present_ms_values: list[float] = []

    def log_frame(
        self,
        *,
        frame: int,
        elapsed_s: float,
        dt_s: float,
        update_ms: float,
        draw_submit_ms: float,
        present_ms: float,
        frame_ms: float,
        scene: SceneState,
    ) -> None:
        fps = (1000.0 / frame_ms) if frame_ms > 0.0 else 0.0
        self._writer.writerow(
            [
                frame,
                f"{elapsed_s:.6f}",
                f"{dt_s:.6f}",
                f"{update_ms:.6f}",
                f"{draw_submit_ms:.6f}",
                f"{present_ms:.6f}",
                f"{frame_ms:.6f}",
                f"{fps:.3f}",
                len(scene.rects),
                len(scene.circles),
                len(scene.lines),
                len(scene.polygons),
                scene.total_objects,
            ]
        )

        self.frame_times_ms.append(frame_ms)
        self.fps_values.append(fps)
        self.update_ms_values.append(update_ms)
        self.draw_ms_values.append(draw_submit_ms)
        self.present_ms_values.append(present_ms)

        if frame % 300 == 0:
            self._csv_file.flush()

    def build_summary(self, total_elapsed_s: float, total_frames: int) -> dict[str, object]:
        avg_frame_ms = statistics.fmean(self.frame_times_ms) if self.frame_times_ms else 0.0
        avg_fps = statistics.fmean(self.fps_values) if self.fps_values else 0.0

        return {
            "engine": self.engine_name,
            "benchmark_name": self.config.benchmark_name,
            "timestamp_utc": self.timestamp,
            "config": asdict(self.config),
            "total_elapsed_s": total_elapsed_s,
            "total_frames": total_frames,
            "avg_fps": avg_fps,
            "avg_frame_ms": avg_frame_ms,
            "frame_ms_p50": _percentile(self.frame_times_ms, 0.50),
            "frame_ms_p95": _percentile(self.frame_times_ms, 0.95),
            "frame_ms_p99": _percentile(self.frame_times_ms, 0.99),
            "update_ms_avg": statistics.fmean(self.update_ms_values) if self.update_ms_values else 0.0,
            "draw_submit_ms_avg": statistics.fmean(self.draw_ms_values) if self.draw_ms_values else 0.0,
            "present_ms_avg": statistics.fmean(self.present_ms_values) if self.present_ms_values else 0.0,
            "csv_path": str(self.csv_path),
        }

    def close(self, summary: dict[str, object]) -> None:
        self._csv_file.flush()
        self._csv_file.close()
        self.summary_path.write_text(json.dumps(summary, indent=2), encoding="utf-8")
