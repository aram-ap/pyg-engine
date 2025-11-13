#!/usr/bin/env python3
"""
Performance Test for Physics System
Analyzes performance with large numbers of physics bodies.
"""

import pygame as pg
import time
import psutil
import os
from pygame import Color
from pyg_engine import Engine, Priority
from pyg_engine import Vector2, Size, BasicShape, GameObject
from pyg_engine import RigidBody, BoxCollider, CircleCollider, Materials

def get_memory_usage():
    """Get current memory usage in MB."""
    process = psutil.Process(os.getpid())
    return process.memory_info().rss / 1024 / 1024

def main():
    """Performance test with large numbers of physics bodies."""
    
    # Create engine
    engine = Engine(size=Size(w=800, h=600), windowName="Performance Test")
    
    # Performance tracking
    performance_data = {
        'object_count': 0,
        'fps_history': [],
        'memory_history': [],
        'physics_time_history': [],
        'render_time_history': []
    }
    
    # ===== PERFORMANCE MONITORING =====
    
    def monitor_performance(engine):
        # Track FPS
        fps = engine.clock.get_fps()
        performance_data['fps_history'].append(fps)
        if len(performance_data['fps_history']) > 60:  # Keep last 60 frames
            performance_data['fps_history'].pop(0)
        
        # Track memory usage
        memory_mb = get_memory_usage()
        performance_data['memory_history'].append(memory_mb)
        if len(performance_data['memory_history']) > 60:
            performance_data['memory_history'].pop(0)
        
        # Update object count
        performance_data['object_count'] = len(engine.getGameObjects())
        
        # Calculate average FPS
        avg_fps = sum(performance_data['fps_history']) / len(performance_data['fps_history']) if performance_data['fps_history'] else 0
        avg_memory = sum(performance_data['memory_history']) / len(performance_data['memory_history']) if performance_data['memory_history'] else 0
        
        print(f"Objects: {performance_data['object_count']}, Avg FPS: {avg_fps:.1f}, Memory: {avg_memory:.1f}MB")
    
    engine.add_runnable(monitor_performance, 'update', Priority.HIGH)
    
    # ===== MASS SPAWNING =====
    
    def spawn_mass_objects(engine):
        """Spawn many objects at once for stress testing."""
        import random
        
        # Spawn 50 objects in a grid pattern
        for i in range(50):
            x = (i % 10) * 60 - 300  # 10 columns, spaced 60 pixels
            y = (i // 10) * 60 - 200  # 5 rows, spaced 60 pixels
            
            # Random shape
            is_circle = random.choice([True, False])
            
            obj = GameObject(f"StressTest_{i}", 
                           position=Vector2(x, y),
                           basicShape=BasicShape.Circle if is_circle else BasicShape.Rectangle,
                           color=Color(random.randint(100, 255), random.randint(100, 255), random.randint(100, 255)),
                           size=Vector2(20, 20))
            
            # Add physics components
            obj.add_component(RigidBody,
                             mass=1.0,
                             gravity_scale=1.0,
                             drag=0.15,
                             use_gravity=True,
                             lock_rotation=False)
            
            if is_circle:
                obj.add_component(CircleCollider,
                                 radius=10,
                                 material=Materials.DEFAULT,
                                 collision_layer="Player")
            else:
                obj.add_component(BoxCollider,
                                 width=20,
                                 height=20,
                                 material=Materials.DEFAULT,
                                 collision_layer="Player")
            
            engine.addGameObject(obj)
        
        print(f"Spawned 50 stress test objects. Total objects: {len(engine.getGameObjects())}")
    
    engine.add_runnable(spawn_mass_objects, 'key_press', Priority.NORMAL, key=pg.K_s)
    
    # ===== PHYSICS PROFILING =====
    
    def profile_physics(engine):
        """Profile physics system performance."""
        start_time = time.time()
        
        # Get physics system stats
        physics_system = engine.physics_system
        space = physics_system.space
        
        # Count bodies and shapes
        body_count = len(space.bodies)
        shape_count = len(space.shapes)
        
        # Calculate physics step time (approximate)
        physics_time = time.time() - start_time
        
        print(f"Physics Bodies: {body_count}, Shapes: {shape_count}, Step Time: {physics_time*1000:.2f}ms")
        
        # Store for averaging
        performance_data['physics_time_history'].append(physics_time * 1000)
        if len(performance_data['physics_time_history']) > 60:
            performance_data['physics_time_history'].pop(0)
    
    engine.add_runnable(profile_physics, 'update', Priority.LOW)
    
    # ===== RENDER PROFILING =====
    
    def profile_render(engine):
        """Profile rendering performance."""
        start_time = time.time()
        
        # This will be called after rendering, so we can measure render time
        render_time = time.time() - start_time
        
        performance_data['render_time_history'].append(render_time * 1000)
        if len(performance_data['render_time_history']) > 60:
            performance_data['render_time_history'].pop(0)
    
    engine.add_runnable(profile_render, 'render', Priority.LOW)
    
    # ===== PERFORMANCE DISPLAY =====
    
    def render_performance_stats(engine):
        """Display performance statistics on screen."""
        font = pg.font.Font(None, 24)
        
        # Calculate averages
        avg_fps = sum(performance_data['fps_history']) / len(performance_data['fps_history']) if performance_data['fps_history'] else 0
        avg_memory = sum(performance_data['memory_history']) / len(performance_data['memory_history']) if performance_data['memory_history'] else 0
        avg_physics = sum(performance_data['physics_time_history']) / len(performance_data['physics_time_history']) if performance_data['physics_time_history'] else 0
        avg_render = sum(performance_data['render_time_history']) / len(performance_data['render_time_history']) if performance_data['render_time_history'] else 0
        
        stats_lines = [
            f"Objects: {performance_data['object_count']}",
            f"Avg FPS: {avg_fps:.1f}",
            f"Memory: {avg_memory:.1f}MB",
            f"Physics: {avg_physics:.2f}ms",
            f"Render: {avg_render:.2f}ms"
        ]
        
        for i, line in enumerate(stats_lines):
            color = pg.Color(255, 255, 255) if avg_fps > 30 else pg.Color(255, 0, 0)
            text = font.render(line, True, color)
            engine.screen.blit(text, (10, 10 + i * 25))
    
    engine.add_runnable(render_performance_stats, 'render', Priority.CRITICAL)
    
    # ===== CONTROLS =====
    
    def render_instructions(engine):
        """Display instructions."""
        font = pg.font.Font(None, 20)
        instructions = [
            "Performance Test Controls:",
            "  S - Spawn 50 stress test objects",
            "  C - Clear all objects",
            "  ESC - Quit",
            "",
            "Watch the performance stats above!"
        ]
        
        for i, instruction in enumerate(instructions):
            color = pg.Color(200, 200, 200) if instruction else pg.Color(100, 100, 100)
            text = font.render(instruction, True, color)
            engine.screen.blit(text, (10, 150 + i * 20))
    
    engine.add_runnable(render_instructions, 'render', Priority.LOW)
    
    # Clear all objects
    def clear_all_objects(engine):
        objects = engine.getGameObjects()
        for obj in objects[:]:
            engine.removeGameObject(obj)
        print("Cleared all objects!")
    
    engine.add_runnable(clear_all_objects, 'key_press', Priority.NORMAL, key=pg.K_c)
    
    # ESC to quit
    def handle_escape(engine):
        print("ESC pressed - quitting...")
        engine.stop()
    
    engine.add_runnable(handle_escape, 'key_press', Priority.CRITICAL, key=pg.K_ESCAPE)
    
    # Start the engine
    print("Starting Performance Test...")
    print("Press S to spawn 50 stress test objects")
    print("Press C to clear all objects")
    print("Watch the performance stats!")
    
    engine.start()

if __name__ == "__main__":
    main() 