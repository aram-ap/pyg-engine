#!/usr/bin/env python3
"""
Python mesh + GameObject demo for pyg_engine.

Demonstrates:
- creating GameObject instances in Python
- adding MeshComponent with fill and optional image
- transform controls (position, rotation, scale)
- draw ordering with layer and z-index

Run from project root after installing the package in editable mode:
    pip install -e .
    python examples/python_mesh_demo.py
    python examples/python_mesh_demo.py --show-fps
    python examples/python_mesh_demo.py --no-texture
"""

import argparse

import pyg_engine as pyg


def create_background() -> pyg.GameObject:
    bg = pyg.GameObject("Background")
    bg.position = pyg.Vec2(0.0, 0.0)
    bg.scale = pyg.Vec2(1.0, 1.0)

    mesh = pyg.MeshComponent("BackgroundMesh")
    mesh.set_geometry_rectangle(1.8, 1.2)
    mesh.set_fill_color(pyg.Color.rgb(20, 24, 32))
    mesh.layer = 0
    mesh.z_index = -0.8
    bg.set_mesh_component(mesh)
    return bg


def create_solid_quad() -> pyg.GameObject:
    quad = pyg.GameObject("SolidQuad")
    quad.position = pyg.Vec2(-0.35, 0.1)
    quad.scale = pyg.Vec2(0.45, 0.45)
    quad.rotation = 0.3

    mesh = pyg.MeshComponent("SolidQuadMesh")
    mesh.set_geometry_rectangle(1.0, 1.0)
    mesh.set_fill_color(pyg.Color.ORANGE)
    mesh.layer = 1
    mesh.z_index = 0.1
    quad.set_mesh_component(mesh)
    return quad


def create_textured_quad(use_texture: bool = True) -> pyg.GameObject:
    quad = pyg.GameObject("TexturedQuad")
    quad.position = pyg.Vec2(0.35, -0.1)
    quad.scale = pyg.Vec2(0.45, 0.45)
    quad.rotation = -0.2

    mesh = pyg.MeshComponent("TexturedQuadMesh")
    mesh.set_geometry_rectangle(1.0, 1.0)
    if use_texture:
        # Loading and decoding image assets can add startup latency.
        mesh.set_fill_color(pyg.Color.WHITE)
        mesh.set_image_path("images/1.png")
    else:
        mesh.set_fill_color(pyg.Color.CYAN)
    mesh.layer = 2
    mesh.z_index = 0.3
    quad.set_mesh_component(mesh)
    return quad


def main() -> None:
    parser = argparse.ArgumentParser(description="PyG Engine Python mesh demo")
    parser.add_argument(
        "--no-texture",
        action="store_true",
        help="Skip textured quad image load for faster startup",
    )
    parser.add_argument(
        "--show-fps",
        action="store_true",
        help="Show FPS in the window title (forces continuous redraw)",
    )
    args = parser.parse_args()

    engine = pyg.Engine(log_level="INFO")

    # NOTE:
    # Mesh transforms are currently interpreted in normalized clip space:
    # X/Y in roughly [-1, 1], where (0,0) is center of screen.
    engine.add_game_object(create_background())
    engine.add_game_object(create_solid_quad())
    engine.add_game_object(create_textured_quad(use_texture=not args.no_texture))

    engine.run(
        title="PyG Engine - Python Mesh Demo",
        width=1280,
        height=720,
        vsync=False,
        background_color=pyg.Color.DARK_GRAY,
        redraw_on_change_only=not args.show_fps,
        show_fps_in_title=args.show_fps,
    )


if __name__ == "__main__":
    main()
