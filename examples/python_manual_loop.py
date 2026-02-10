#!/usr/bin/env python3
"""
Python manual loop demo for pyg_engine.

Demonstrates controlling the engine loop manually from Python using:
- engine.initialize()
- engine.poll_events()
- engine.update()
- engine.render()
"""

import math
import time
import pyg_engine as pyg


def main() -> None:
    # Initialize engine with logging
    engine = pyg.Engine(log_level="INFO")
    
    # Initialize window without starting the loop
    engine.initialize(
        title="PyG Engine - Manual Loop Demo",
        width=1280,
        height=720,
        background_color=pyg.Color.rgb(30, 30, 40),
        vsync=False,
        show_fps_in_title=True
    )

    engine.log_info("Starting manual loop...")

    # Create a moving circle
    circle_x = 640.0
    circle_y = 360.0
    radius = 50.0
    
    last_time = time.time()
    
    running = True
    while running:
        # 1. Poll events
        # Returns False if the window is closed or exit is requested
        running = engine.poll_events()
        
        if not running:
            break
            
        # Calculate delta time
        current_time = time.time()
        # dt = current_time - last_time # unused in this simple demo
        last_time = current_time
        
        # 2. Update logic
        t = time.time()
        circle_x = 640.0 + math.cos(t * 2.0) * 200.0
        circle_y = 360.0 + math.sin(t * 3.0) * 150.0
        
        # Clear previous frame's draw commands
        engine.clear_draw_commands()
        
        # Draw new frame
        engine.draw_circle(
            circle_x, 
            circle_y, 
            radius, 
            pyg.Color.CYAN, 
            filled=True
        )
        
        engine.draw_line(
            640.0, 360.0, 
            circle_x, circle_y, 
            pyg.Color.WHITE
        )
        
        # 3. Update engine state (time, input, etc.)
        engine.update()
        
        # 4. Render frame
        engine.render()

    engine.log_info("Manual loop finished.")


if __name__ == "__main__":
    main()
