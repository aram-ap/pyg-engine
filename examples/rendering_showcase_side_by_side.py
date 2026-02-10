#!/usr/bin/env python3
"""
Launch the engine showcase and pygame clone together for comparison.

This starts:
1) `python_rendering_showcase_demo.py` (pyg_engine)
2) `pygame_rendering_showcase_demo.py` (pygame clone)
"""

from __future__ import annotations

import argparse
import subprocess
import sys
import time
from pathlib import Path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Launch side-by-side rendering comparison")
    parser.add_argument(
        "--pygame-window-x",
        type=int,
        default=1420,
        help="X position for pygame window",
    )
    parser.add_argument(
        "--pygame-window-y",
        type=int,
        default=40,
        help="Y position for pygame window",
    )
    parser.add_argument(
        "--pygame-fps",
        type=float,
        default=60.0,
        help="FPS cap for pygame window",
    )
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    examples_dir = Path(__file__).resolve().parent
    pyg_showcase = examples_dir / "python_rendering_showcase_demo.py"
    pygame_showcase = examples_dir / "pygame_rendering_showcase_demo.py"

    python_exe = sys.executable
    pyg_proc = subprocess.Popen([python_exe, str(pyg_showcase)])
    pygame_proc = subprocess.Popen(
        [
            python_exe,
            str(pygame_showcase),
            "--window-x",
            str(args.pygame_window_x),
            "--window-y",
            str(args.pygame_window_y),
            "--fps",
            str(args.pygame_fps),
        ]
    )

    processes = [pyg_proc, pygame_proc]
    try:
        while True:
            time.sleep(0.25)
            if any(proc.poll() is not None for proc in processes):
                break
    except KeyboardInterrupt:
        pass
    finally:
        for proc in processes:
            if proc.poll() is None:
                proc.terminate()


if __name__ == "__main__":
    main()
