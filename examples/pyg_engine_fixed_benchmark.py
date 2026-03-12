#!/usr/bin/env python3
"""
Fixed, uncapped performance benchmark for pyg_engine.
"""

from __future__ import annotations

import argparse
import time

import pyg_engine as pyg

from fixed_benchmark_common import (
    BenchmarkConfig,
    BenchmarkLogger,
    create_scene,
    line_endpoints,
    polygon_points,
    update_scene,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Run fixed-scene uncapped benchmark in pyg_engine."
    )
    parser.add_argument("--width", type=int, default=1920)
    parser.add_argument("--height", type=int, default=1080)
    parser.add_argument("--duration", type=float, default=20.0, help="Benchmark duration in seconds.")
    parser.add_argument("--seed", type=int, default=1337)
    parser.add_argument("--rects", type=int, default=2200)
    parser.add_argument("--circles", type=int, default=1800)
    parser.add_argument("--lines", type=int, default=1000)
    parser.add_argument("--polygons", type=int, default=600)
    parser.add_argument("--log-interval", type=float, default=1.0, help="Console report cadence in seconds.")
    parser.add_argument("--output-dir", default="benchmark_logs")
    parser.add_argument("--name", default="fixed_scene_v1", help="Benchmark run group name.")
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    config = BenchmarkConfig(
        width=args.width,
        height=args.height,
        duration_s=args.duration,
        seed=args.seed,
        log_interval_s=args.log_interval,
        rect_count=args.rects,
        circle_count=args.circles,
        line_count=args.lines,
        polygon_count=args.polygons,
        output_dir=args.output_dir,
        benchmark_name=args.name,
    )

    scene = create_scene(config)
    logger = BenchmarkLogger(engine_name="pyg_engine", config=config)

    print("[pyg_engine] starting fixed benchmark")
    print(
        f"[pyg_engine] resolution={config.width}x{config.height} duration={config.duration_s:.1f}s "
        f"objects={scene.total_objects} (rect={config.rect_count}, circle={config.circle_count}, "
        f"line={config.line_count}, polygon={config.polygon_count})"
    )
    print(f"[pyg_engine] logs: {logger.csv_path} and {logger.summary_path}")

    engine = pyg.Engine(log_level="INFO")
    engine.start_manual(
        title="pyg_engine Fixed Benchmark (Uncapped)",
        width=config.width,
        height=config.height,
        resizable=False,
        vsync=False,
        redraw_on_change_only=False,
        show_fps_in_title=True,
    )

    start = time.perf_counter()
    last_frame = start
    next_report = start + config.log_interval_s
    frame = 0
    color_cache: dict[tuple[int, int, int], pyg.Color] = {}

    def get_color(rgb: tuple[int, int, int]) -> pyg.Color:
        cached = color_cache.get(rgb)
        if cached is None:
            cached = pyg.Color.rgb(*rgb)
            color_cache[rgb] = cached
        return cached

    while engine.poll_events():
        now = time.perf_counter()
        elapsed = now - start
        if elapsed >= config.duration_s:
            break

        dt = now - last_frame
        dt = min(dt, 0.05)
        last_frame = now

        update_start = time.perf_counter()
        update_scene(scene, dt, config.width, config.height)
        update_ms = (time.perf_counter() - update_start) * 1000.0

        draw_start = time.perf_counter()
        engine.clear_draw_commands()

        # Rectangles
        for obj in scene.rects:
            engine.draw_rectangle(obj.x, obj.y, obj.w, obj.h, get_color(obj.color), filled=True)

        # Circles
        for obj in scene.circles:
            engine.draw_circle(obj.x, obj.y, obj.radius, get_color(obj.color), filled=True, segments=18)

        # Lines
        for obj in scene.lines:
            x1, y1, x2, y2 = line_endpoints(obj)
            engine.draw_line(
                x1,
                y1,
                x2,
                y2,
                get_color(obj.color),
                thickness=float(obj.thickness),
            )

        # Polygons
        for obj in scene.polygons:
            points = [pyg.Vec2(px, py) for px, py in polygon_points(obj)]
            engine.draw(pyg.Polygon(points=points, color=get_color(obj.color)))

        draw_submit_ms = (time.perf_counter() - draw_start) * 1000.0

        present_start = time.perf_counter()
        engine.update()
        engine.render()
        present_ms = (time.perf_counter() - present_start) * 1000.0

        frame_ms = (time.perf_counter() - now) * 1000.0
        logger.log_frame(
            frame=frame,
            elapsed_s=elapsed,
            dt_s=dt,
            update_ms=update_ms,
            draw_submit_ms=draw_submit_ms,
            present_ms=present_ms,
            frame_ms=frame_ms,
            scene=scene,
        )

        if now >= next_report:
            fps = 1000.0 / frame_ms if frame_ms > 0.0 else 0.0
            print(
                f"[pyg_engine] t={elapsed:6.2f}s frame={frame:6d} fps={fps:8.2f} "
                f"frame_ms={frame_ms:7.3f} update_ms={update_ms:7.3f} "
                f"draw_ms={draw_submit_ms:7.3f} present_ms={present_ms:7.3f}"
            )
            next_report += config.log_interval_s

        frame += 1

    total_elapsed = time.perf_counter() - start
    summary = logger.build_summary(total_elapsed_s=total_elapsed, total_frames=frame)
    logger.close(summary)

    print(
        "[pyg_engine] complete "
        f"frames={frame} elapsed={total_elapsed:.3f}s avg_fps={summary['avg_fps']:.2f} "
        f"p95_ms={summary['frame_ms_p95']:.3f}"
    )
    print(f"[pyg_engine] summary json: {logger.summary_path}")


if __name__ == "__main__":
    main()
