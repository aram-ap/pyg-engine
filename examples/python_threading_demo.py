#!/usr/bin/env python3
"""
Python threading demo for pyg_engine.

Demonstrates:
1. Running the main engine loop in the main thread
2. Spawning a background thread to generate game objects
3. Using `engine.get_handle()` to safely send commands from the background thread
"""

import math
import time
import threading
import random
import pyg_engine as pyg


def background_spawner(handle: pyg.EngineHandle, stop_event: threading.Event, counter: list[int]) -> None:
    """
    Background thread that spawns objects safely using the engine handle.
    """
    print("[Thread] Spawner started.")
    
    while not stop_event.is_set():
        # Simulate some work
        time.sleep(0.001)
        
        # Calculate random position
        x = random.randint(100, 1180)
        y = random.randint(100, 620)
        
        # Create object
        # Note: We create PyGameObjects here, but we can't add them directly.
        # We must use the handle.
        obj = pyg.GameObject(name=f"Spawned_{int(time.time()*1000)}")
        # obj.set_position is not available on PyGameObject directly in Python as a method
        # but as a property 'position' setter
        obj.position = pyg.Vec2(float(x), float(y))
        
        # Add a visual representation
        mesh = pyg.MeshComponent()
        mesh.set_geometry_rectangle(20.0, 20.0)
        
        r = random.random()
        g = random.random()
        b = random.random()
        mesh.set_fill_color(pyg.Color(r, g, b, 1.0))
        
        obj.add_mesh_component(mesh)
        
        # Send command to main thread to add this object
        # This is thread-safe!
        handle.add_game_object(obj)
        
        # We can also draw immediate mode debug lines from this thread!
        handle.draw_line(
            640.0, 360.0, 
            float(x), float(y), 
            pyg.Color.WHITE, 
            thickness=1.0,
            layer=0,
            z_index=0.1
        )
        
        # Increment line count
        counter[0] += 1

    print("[Thread] Spawner stopped.")


def main() -> None:
    # Initialize engine
    engine = pyg.Engine(log_level="INFO")
    
    engine.initialize(
        title="PyG Engine - Threading Demo",
        width=1280,
        height=720,
        vsync=False,
        show_fps_in_title=True
    )

    # Get a thread-safe handle
    handle = engine.get_handle()
    
    # Start background thread
    stop_event = threading.Event()
    line_count = [0]
    spawner_thread = threading.Thread(
        target=background_spawner, 
        args=(handle, stop_event, line_count)
    )
    spawner_thread.start()

    engine.log_info("Main loop started. Objects are spawning from a background thread.")

    running = True
    while running:
        # 1. Poll events
        running = engine.poll_events()
        
        if not running:
            break
            
        # Update title with line count
        engine.set_window_title(f"PyG Engine - Lines: {line_count[0]}")
            
        # 2. Update engine state
        # This will process the command queue and add the objects sent by the thread
        engine.update()
        
        # 3. Render frame
        engine.render()

    # Clean up
    stop_event.set()
    spawner_thread.join()
    engine.log_info("Demo finished.")


if __name__ == "__main__":
    main()
