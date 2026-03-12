#!/usr/bin/env python3
"""
Fixed, uncapped performance benchmark for pygame.
"""

from __future__ import annotations

import argparse
import time

import pygame

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
        description="Run fixed-scene uncapped benchmark in pygame."
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
    logger = BenchmarkLogger(engine_name="pygame", config=config)

    print("[pygame] starting fixed benchmark")
    print(
        f"[pygame] resolution={config.width}x{config.height} duration={config.duration_s:.1f}s "
        f"objects={scene.total_objects} (rect={config.rect_count}, circle={config.circle_count}, "
        f"line={config.line_count}, polygon={config.polygon_count})"
    )
    print(f"[pygame] logs: {logger.csv_path} and {logger.summary_path}")

    pygame.init()
    window = pygame.display.set_mode((config.width, config.height))
    pygame.display.set_caption("pygame Fixed Benchmark (Uncapped)")

    start = time.perf_counter()
    last_frame = start
    next_report = start + config.log_interval_s
    frame = 0
    running = True

    while running:
        now = time.perf_counter()
        elapsed = now - start
        if elapsed >= config.duration_s:
            break

        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
            elif event.type == pygame.KEYDOWN and event.key == pygame.K_ESCAPE:
                running = False
        if not running:
            break

        dt = now - last_frame
        dt = min(dt, 0.05)
        last_frame = now

        update_start = time.perf_counter()
        update_scene(scene, dt, config.width, config.height)
        update_ms = (time.perf_counter() - update_start) * 1000.0

        draw_start = time.perf_counter()
        window.fill((8, 8, 12))

        for obj in scene.rects:
            pygame.draw.rect(window, obj.color, (obj.x, obj.y, obj.w, obj.h))

        for obj in scene.circles:
            pygame.draw.circle(window, obj.color, (int(obj.x), int(obj.y)), int(obj.radius))

        for obj in scene.lines:
            x1, y1, x2, y2 = line_endpoints(obj)
            pygame.draw.line(
                window,
                obj.color,
                (x1, y1),
                (x2, y2),
                obj.thickness,
            )

        for obj in scene.polygons:
            pygame.draw.polygon(window, obj.color, polygon_points(obj))

        draw_submit_ms = (time.perf_counter() - draw_start) * 1000.0

        present_start = time.perf_counter()
        pygame.display.flip()
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
                f"[pygame] t={elapsed:6.2f}s frame={frame:6d} fps={fps:8.2f} "
                f"frame_ms={frame_ms:7.3f} update_ms={update_ms:7.3f} "
                f"draw_ms={draw_submit_ms:7.3f} present_ms={present_ms:7.3f}"
            )
            next_report += config.log_interval_s

        frame += 1

    total_elapsed = time.perf_counter() - start
    summary = logger.build_summary(total_elapsed_s=total_elapsed, total_frames=frame)
    logger.close(summary)

    pygame.quit()

    print(
        "[pygame] complete "
        f"frames={frame} elapsed={total_elapsed:.3f}s avg_fps={summary['avg_fps']:.2f} "
        f"p95_ms={summary['frame_ms_p95']:.3f}"
    )
    print(f"[pygame] summary json: {logger.summary_path}")


if __name__ == "__main__":
    main()
