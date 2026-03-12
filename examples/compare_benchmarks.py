#!/usr/bin/env python3
"""
Compare benchmark summary JSON files for pyg_engine vs pygame.
"""

from __future__ import annotations

import argparse
import json
import re
import statistics
from pathlib import Path
from typing import Any


PAIR_RE = re.compile(r"^(pyg_engine|pygame)_(\d{8}T\d{6}Z)\.json$")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Compare benchmark summary files (pyg_engine vs pygame)."
    )
    parser.add_argument(
        "--logs-dir",
        default="benchmark_logs/fixed_scene_v1",
        help="Directory containing benchmark summary JSON files.",
    )
    parser.add_argument(
        "--left",
        default=None,
        help="Path to pyg_engine summary JSON (overrides auto-discovery).",
    )
    parser.add_argument(
        "--right",
        default=None,
        help="Path to pygame summary JSON (overrides auto-discovery).",
    )
    parser.add_argument(
        "--timestamp",
        default=None,
        help="Timestamp to compare (format: YYYYMMDDTHHMMSSZ).",
    )
    parser.add_argument(
        "--all",
        action="store_true",
        help="Aggregate all matched pairs grouped by resolution.",
    )
    return parser.parse_args()


def load_summary(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def percent_improvement(lower_is_better_a: float, lower_is_better_b: float) -> float:
    if lower_is_better_b == 0:
        return 0.0
    return (1.0 - (lower_is_better_a / lower_is_better_b)) * 100.0


def fps_gain_pct(a: float, b: float) -> float:
    if b == 0:
        return 0.0
    return ((a / b) - 1.0) * 100.0


def discover_pairs(logs_dir: Path) -> dict[str, dict[str, Path]]:
    pairs: dict[str, dict[str, Path]] = {}
    for path in logs_dir.glob("*.json"):
        match = PAIR_RE.match(path.name)
        if not match:
            continue
        engine, timestamp = match.groups()
        if timestamp not in pairs:
            pairs[timestamp] = {}
        pairs[timestamp][engine] = path
    return pairs


def compare_pair(pyg_data: dict[str, Any], pygame_data: dict[str, Any]) -> str:
    w = pyg_data["config"]["width"]
    h = pyg_data["config"]["height"]
    duration = pyg_data["config"]["duration_s"]

    pyg_fps = float(pyg_data["avg_fps"])
    pg_fps = float(pygame_data["avg_fps"])
    pyg_frame = float(pyg_data["avg_frame_ms"])
    pg_frame = float(pygame_data["avg_frame_ms"])
    pyg_p95 = float(pyg_data["frame_ms_p95"])
    pg_p95 = float(pygame_data["frame_ms_p95"])
    pyg_draw = float(pyg_data["draw_submit_ms_avg"])
    pg_draw = float(pygame_data["draw_submit_ms_avg"])
    pyg_present = float(pyg_data["present_ms_avg"])
    pg_present = float(pygame_data["present_ms_avg"])

    lines = [
        f"Resolution: {w}x{h}, duration: {duration:.1f}s",
        f"avg_fps:        pyg_engine={pyg_fps:8.2f} | pygame={pg_fps:8.2f} | "
        f"delta={fps_gain_pct(pyg_fps, pg_fps):7.2f}%",
        f"avg_frame_ms:   pyg_engine={pyg_frame:8.3f} | pygame={pg_frame:8.3f} | "
        f"improvement={percent_improvement(pyg_frame, pg_frame):7.2f}%",
        f"p95_frame_ms:   pyg_engine={pyg_p95:8.3f} | pygame={pg_p95:8.3f} | "
        f"improvement={percent_improvement(pyg_p95, pg_p95):7.2f}%",
        f"draw_submit_ms: pyg_engine={pyg_draw:8.3f} | pygame={pg_draw:8.3f} | "
        f"improvement={percent_improvement(pyg_draw, pg_draw):7.2f}%",
        f"present_ms:     pyg_engine={pyg_present:8.3f} | pygame={pg_present:8.3f} | "
        f"improvement={percent_improvement(pyg_present, pg_present):7.2f}%",
    ]
    return "\n".join(lines)


def aggregate_all(pairs: dict[str, dict[str, Path]]) -> str:
    grouped: dict[tuple[int, int], list[tuple[dict[str, Any], dict[str, Any]]]] = {}
    for timestamp, engines in pairs.items():
        if "pyg_engine" not in engines or "pygame" not in engines:
            continue
        pyg_data = load_summary(engines["pyg_engine"])
        pygame_data = load_summary(engines["pygame"])
        key = (int(pyg_data["config"]["width"]), int(pyg_data["config"]["height"]))
        grouped.setdefault(key, []).append((pyg_data, pygame_data))

    if not grouped:
        return "No matched pyg_engine/pygame summary pairs found."

    out: list[str] = []
    for (w, h), rows in sorted(grouped.items()):
        pyg_fps = statistics.fmean(float(r[0]["avg_fps"]) for r in rows)
        pg_fps = statistics.fmean(float(r[1]["avg_fps"]) for r in rows)
        pyg_ms = statistics.fmean(float(r[0]["avg_frame_ms"]) for r in rows)
        pg_ms = statistics.fmean(float(r[1]["avg_frame_ms"]) for r in rows)
        pyg_p95 = statistics.fmean(float(r[0]["frame_ms_p95"]) for r in rows)
        pg_p95 = statistics.fmean(float(r[1]["frame_ms_p95"]) for r in rows)
        out.append(
            f"{w}x{h} (n={len(rows)}): "
            f"fps +{fps_gain_pct(pyg_fps, pg_fps):.2f}% | "
            f"avg_frame_ms {percent_improvement(pyg_ms, pg_ms):.2f}% better | "
            f"p95 {percent_improvement(pyg_p95, pg_p95):.2f}% better"
        )
    return "\n".join(out)


def main() -> None:
    args = parse_args()

    if args.left and args.right:
        pyg_path = Path(args.left).resolve()
        pygame_path = Path(args.right).resolve()
        pyg_data = load_summary(pyg_path)
        pygame_data = load_summary(pygame_path)
        print(compare_pair(pyg_data, pygame_data))
        return

    logs_dir = Path(args.logs_dir).resolve()
    pairs = discover_pairs(logs_dir)

    if args.all:
        print(aggregate_all(pairs))
        return

    matched_timestamps = sorted(
        ts for ts, p in pairs.items() if "pyg_engine" in p and "pygame" in p
    )
    if not matched_timestamps:
        raise SystemExit(f"No complete benchmark pairs found in {logs_dir}")

    target_timestamp = args.timestamp or matched_timestamps[-1]
    if target_timestamp not in pairs:
        raise SystemExit(f"Timestamp {target_timestamp} not found in {logs_dir}")
    if "pyg_engine" not in pairs[target_timestamp] or "pygame" not in pairs[target_timestamp]:
        raise SystemExit(f"Timestamp {target_timestamp} does not have both engine summaries.")

    pyg_data = load_summary(pairs[target_timestamp]["pyg_engine"])
    pygame_data = load_summary(pairs[target_timestamp]["pygame"])

    print(f"timestamp: {target_timestamp}")
    print(compare_pair(pyg_data, pygame_data))


if __name__ == "__main__":
    main()
