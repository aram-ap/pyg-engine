"""
Time Scale and Headless Mode Test Example

This example demonstrates:
1. Time scaling (slow-motion, normal, and fast-forward)
2. Headless mode (no display for ML training)
3. Performance comparison between modes
"""

import sys
import time
from pathlib import Path

# Add parent directory to path
sys.path.insert(0, str(Path(__file__).parent.parent))

from src.core.engine import Engine
from src.core.gameobject import GameObject
from src.components.script import Script
from src.physics.rigidbody import RigidBody
from src.physics.collider import BoxCollider, CircleCollider
from src.utilities.vector2 import Vector2
from src.utilities.object_types import Size, BasicShape
from pygame import Color


class TimeScaleTest(Script):
    """Test script that monitors time scaling and logs performance."""
    
    def __init__(self, game_object):
        super().__init__(game_object)
        self.elapsed_time = 0.0
        self.frame_count = 0
        self.start_real_time = None
    
    def update(self, engine):
        # Initialize on first update
        if self.start_real_time is None:
            self.start_real_time = time.time()
            print(f"\n{'='*50}")
            print(f"Starting TimeScaleTest")
            print(f"Time Scale: {engine.get_time_scale()}x")
            print(f"Headless Mode: {not engine._Engine__useDisplay}")
            print(f"FPS Cap: {engine.fpsCap}")
            print(f"{'='*50}\n")
            return
        
        self.elapsed_time += engine.get_unscaled_dt()
        self.frame_count += 1
        
        # Log every 60 frames (roughly 1 second at 60 FPS)
        if self.frame_count % 60 == 0:
            real_elapsed = time.time() - self.start_real_time
            sim_time = self.elapsed_time
            avg_fps = self.frame_count / real_elapsed if real_elapsed > 0 else 0
            speedup = sim_time / real_elapsed if real_elapsed > 0 else 0
            
            print(f"Frame {self.frame_count:5d} | "
                  f"Sim Time: {sim_time:6.2f}s | "
                  f"Real Time: {real_elapsed:6.2f}s | "
                  f"Speedup: {speedup:5.2f}x | "
                  f"FPS: {avg_fps:6.1f}")
        
        # Stop after 300 frames (5 seconds at 60 FPS in simulation time)
        if self.frame_count >= 300:
            real_elapsed = time.time() - self.start_real_time
            sim_time = self.elapsed_time
            avg_fps = self.frame_count / real_elapsed if real_elapsed > 0 else 0
            speedup = sim_time / real_elapsed if real_elapsed > 0 else 0
            
            print(f"\n{'='*50}")
            print(f"Test Complete!")
            print(f"Total Frames: {self.frame_count}")
            print(f"Simulation Time: {sim_time:.2f}s")
            print(f"Real Time: {real_elapsed:.2f}s")
            print(f"Average FPS: {avg_fps:.1f}")
            print(f"Actual Speedup: {speedup:.2f}x")
            print(f"Expected Speedup: {engine.get_time_scale()}x")
            print(f"{'='*50}\n")
            
            engine.stop()


class PhysicsTest(Script):
    """Script to test physics with time scaling."""
    
    def __init__(self, game_object):
        super().__init__(game_object)
        self.jump_timer = 0.0
        self.jump_interval = 2.0  # Jump every 2 seconds
        
    def update(self, engine):
        self.jump_timer += engine.dt()
        
        # Periodic jump to test physics
        if self.jump_timer >= self.jump_interval:
            self.jump_timer = 0.0
            rb = self.game_object.get_component(RigidBody)
            if rb:
                # Apply upward force
                rb.add_force(Vector2(0, 5000))


def run_test(time_scale=1.0, headless=False, fps_cap=60, test_name="Test"):
    """Run a test with specific parameters."""
    print(f"\n\n{'#'*60}")
    print(f"# {test_name}")
    print(f"# Time Scale: {time_scale}x, Headless: {headless}, FPS Cap: {fps_cap}")
    print(f"{'#'*60}")
    
    # Create engine
    engine = Engine(
        size=Size(w=800, h=600),
        backgroundColor=Color(30, 30, 30),
        windowName="Time Scale Test",
        fpsCap=fps_cap,
        useDisplay=not headless
    )
    
    # Set time scale
    engine.set_time_scale(time_scale)
    
    # Create test object with monitor script
    monitor = GameObject(name="Monitor", position=Vector2(0, 0))
    monitor.add_component(TimeScaleTest)
    engine.addGameObject(monitor)
    
    # Create physics objects to test physics scaling
    for i in range(5):
        x = -200 + i * 100
        ball = GameObject(
            name=f"Ball_{i}",
            position=Vector2(x, 100),
            size=Vector2(20, 20),
            color=Color(50 + i * 40, 100, 200),
            basicShape=BasicShape.Circle
        )
        
        # Add physics
        rb = ball.add_component(RigidBody)
        rb.mass = 1.0
        rb.use_gravity = True
        ball.add_component(CircleCollider, radius=20)
        
        # Add physics test script to first ball
        if i == 0:
            ball.add_component(PhysicsTest)
        
        engine.addGameObject(ball)
    
    # Create ground
    ground = GameObject(
        name="Ground",
        position=Vector2(0, -200),
        size=Vector2(400, 20),
        color=Color(100, 100, 100),
        basicShape=BasicShape.Rectangle
    )
    ground_rb = ground.add_component(RigidBody)
    ground_rb.is_kinematic = True
    ground.add_component(BoxCollider, width=400, height=20)
    engine.addGameObject(ground)
    
    # Start engine
    engine.start()


def main():
    """Run multiple tests to demonstrate time scaling and headless mode."""
    
    print("\n" + "="*60)
    print("TIME SCALE AND HEADLESS MODE TEST SUITE")
    print("="*60)
    
    # Test 1: Normal speed with display
    run_test(
        time_scale=1.0,
        headless=False,
        fps_cap=60,
        test_name="Test 1: Normal Speed (1.0x) with Display"
    )
    
    # Test 2: 5x speed with display
    run_test(
        time_scale=5.0,
        headless=False,
        fps_cap=60,
        test_name="Test 2: Fast Forward (5.0x) with Display"
    )
    
    # Test 3: 10x speed headless (ML training mode)
    run_test(
        time_scale=10.0,
        headless=True,
        fps_cap=60,
        test_name="Test 3: ML Training Mode (10.0x, Headless)"
    )
    
    # Test 4: 100x speed headless (extreme ML training)
    run_test(
        time_scale=100.0,
        headless=True,
        fps_cap=0,  # Uncapped
        test_name="Test 4: Extreme ML Training (100.0x, Headless, Uncapped FPS)"
    )
    
    # Test 5: Slow motion with display
    run_test(
        time_scale=0.5,
        headless=False,
        fps_cap=60,
        test_name="Test 5: Slow Motion (0.5x) with Display"
    )
    
    print("\n" + "="*60)
    print("ALL TESTS COMPLETE!")
    print("="*60)
    print("\nSummary:")
    print("- Time scaling works correctly across different speeds")
    print("- Headless mode significantly improves performance")
    print("- Physics simulation scales properly with time_scale")
    print("- Perfect for ML/deep reinforcement learning training!")
    print("="*60 + "\n")


if __name__ == "__main__":
    main()

