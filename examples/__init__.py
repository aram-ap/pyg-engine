"""
Pyg Engine Examples

This directory contains example scripts demonstrating various features of the Pyg Engine.
Run examples using: python examples/<example_name>.py
"""

import os
import sys
from pathlib import Path

# Add the examples directory to the Python path so examples can import from scripts/
examples_dir = Path(__file__).parent
scripts_dir = examples_dir / "scripts"
if scripts_dir.exists():
    sys.path.insert(0, str(scripts_dir))

# Add src to the Python path so examples can import pyg_engine
src_dir = examples_dir.parent / "src"
if src_dir.exists():
    sys.path.insert(0, str(src_dir))

def get_script_path(script_name):
    """Get the absolute path to a script in the examples/scripts directory."""
    return str(scripts_dir / script_name)

def list_examples():
    """List all available examples with descriptions."""
    examples = {
        'basic_example.py': 'Basic engine setup and object creation',
        'test.py': 'Basics of object creation and script usage',
        'main.py': 'Multi-player control demo with different configurations, physics, and mouse+keyboard support',
        'mouse_example.py': 'Mouse input handling and interaction',
        'enhanced_mouse_example.py': 'Advanced mouse interactions with physics',
        'snake_game.py': 'Complete Snake game implementation with scoring, collision detection, and UI',
    }

    print("Available Pyg Engine Examples:")
    print("Run with: python examples/<example_name>.py")
    print()
    for name, description in examples.items():
        print(f"  {name}: {description}")

if __name__ == "__main__":
    list_examples()
