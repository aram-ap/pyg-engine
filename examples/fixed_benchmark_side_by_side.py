#!/usr/bin/env python3
"""
Launch pygame and pyg_engine fixed benchmarks together.
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Launch pyg_engine and pygame fixed benchmarks with identical settings."
    )
    parser.add_argument("--width", type=int, default=1920)
    parser.add_argument("--height", type=int, default=1080)
    parser.add_argument("--duration", type=float, default=20.0)
    parser.add_argument("--seed", type=int, default=1337)
    parser.add_argument("--rects", type=int, default=2200)
    parser.add_argument("--circles", type=int, default=1800)
    parser.add_argument("--lines", type=int, default=1000)
    parser.add_argument("--polygons", type=int, default=600)
    parser.add_argument("--log-interval", type=float, default=1.0)
    parser.add_argument("--output-dir", default="benchmark_logs")
    parser.add_argument("--name", default="fixed_scene_v1")
    return parser.parse_args()


def build_common_args(args: argparse.Namespace) -> list[str]:
    return [
        "--width",
        str(args.width),
        "--height",
        str(args.height),
        "--duration",
        str(args.duration),
        "--seed",
        str(args.seed),
        "--rects",
        str(args.rects),
        "--circles",
        str(args.circles),
        "--lines",
        str(args.lines),
        "--polygons",
        str(args.polygons),
        "--log-interval",
        str(args.log_interval),
        "--output-dir",
        args.output_dir,
        "--name",
        args.name,
    ]


def main() -> None:
    args = parse_args()
    root = Path(__file__).resolve().parent
    pyg_script = root / "pyg_engine_fixed_benchmark.py"
    pygame_script = root / "pygame_fixed_benchmark.py"
    common = build_common_args(args)

    cmd_pyg = [sys.executable, str(pyg_script), *common]
    cmd_pygame = [sys.executable, str(pygame_script), *common]

    print("Launching side-by-side benchmark processes.")
    print("Arrange windows side-by-side on your desktop for visual parity.")
    print(f"pyg_engine cmd: {' '.join(cmd_pyg)}")
    print(f"pygame cmd: {' '.join(cmd_pygame)}")

    proc_pyg = subprocess.Popen(cmd_pyg, cwd=root.parent)
    proc_pygame = subprocess.Popen(cmd_pygame, cwd=root.parent)

    pyg_return = proc_pyg.wait()
    pygame_return = proc_pygame.wait()

    if pyg_return != 0 or pygame_return != 0:
        raise SystemExit(
            f"Benchmark process failed (pyg_engine={pyg_return}, pygame={pygame_return})."
        )

    print("Both benchmarks completed successfully.")
    print(f"Logs were written under: {Path(args.output_dir).resolve() / args.name}")


if __name__ == "__main__":
    main()
