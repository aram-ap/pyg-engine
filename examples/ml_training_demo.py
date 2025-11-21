"""
ML Training Demo - Simple example showing time scaling and headless mode

This demonstrates the key features for machine learning / deep reinforcement learning:
1. Time scaling to run simulations faster than real-time
2. Headless mode to skip rendering overhead
3. Physics simulation that scales correctly with time
"""

import sys
import time
from pathlib import Path

# Add parent directory to path
sys.path.insert(0, str(Path(__file__).parent.parent))

from src.core.engine import Engine, configure_headless_mode
from src.core.gameobject import GameObject
from src.components.script import Script
from src.physics.rigidbody import RigidBody
from src.physics.collider import CircleCollider, BoxCollider
from src.utilities.vector2 import Vector2
from src.utilities.object_types import Size, BasicShape
from pygame import Color


class PerformanceMonitor(Script):
    """Monitor that tracks simulation performance."""
    
    def __init__(self, game_object):
        super().__init__(game_object)
        self.start_time = None
        self.frame_count = 0
        self.target_frames = 300  # Run for 300 frames (5 seconds at 60 FPS sim time)
        
    def update(self, engine):
        if self.start_time is None:
            self.start_time = time.time()
            print(f"\n{'='*60}")
            print(f"Performance Monitor Started")
            print(f"  Time Scale: {engine.get_time_scale()}x")
            print(f"  Headless: {not engine._Engine__useDisplay}")
            print(f"  FPS Cap: {engine.fpsCap}")
            print(f"  Target: {self.target_frames} frames")
            print(f"{'='*60}\n")
            return
        
        self.frame_count += 1
        
        # Stop after target frames
        if self.frame_count >= self.target_frames:
            real_time = time.time() - self.start_time
            sim_time = self.frame_count / 60.0  # Assuming 60 FPS
            speedup = sim_time / real_time if real_time > 0 else 0
            fps = self.frame_count / real_time if real_time > 0 else 0
            
            print(f"\n{'='*60}")
            print(f"Performance Results:")
            print(f"  Frames Processed: {self.frame_count}")
            print(f"  Simulation Time: {sim_time:.2f}s")
            print(f"  Real Time: {real_time:.2f}s")
            print(f"  Average FPS: {fps:.1f}")
            print(f"  Speedup: {speedup:.2f}x (expected: {engine.get_time_scale()}x)")
            print(f"  Efficiency: {(speedup/engine.get_time_scale()*100):.1f}%")
            print(f"{'='*60}\n")
            
            engine.stop()


def create_simple_scene(engine):
    """Create a simple physics scene for testing."""
    
    # Add performance monitor
    monitor = GameObject(name="Monitor", position=Vector2(0, 0))
    monitor.add_component(PerformanceMonitor)
    engine.addGameObject(monitor)
    
    # Create falling balls
    for i in range(5):
        x = -200 + i * 100
        ball = GameObject(
            name=f"Ball_{i}",
            position=Vector2(x, 200),
            size=Vector2(20, 20),
            color=Color(100 + i * 30, 150, 200),
            basicShape=BasicShape.Circle
        )
        
        rb = ball.add_component(RigidBody)
        rb.mass = 1.0
        rb.use_gravity = True
        ball.add_component(CircleCollider, radius=20)
        
        engine.addGameObject(ball)
    
    # Create ground
    ground = GameObject(
        name="Ground",
        position=Vector2(0, -250),
        size=Vector2(500, 20),
        color=Color(100, 100, 100),
        basicShape=BasicShape.Rectangle
    )
    ground_rb = ground.add_component(RigidBody)
    ground_rb.is_kinematic = True
    ground.add_component(BoxCollider, width=500, height=20)
    engine.addGameObject(ground)


def main():
    """Run demonstrations of different configurations."""
    
    # IMPORTANT: Configure headless mode BEFORE creating any engines
    # This must be called before pygame initializes
    configure_headless_mode()
    
    print("\n" + "="*70)
    print(" ML TRAINING DEMO - Time Scaling and Headless Mode")
    print("="*70)
    print("\nNOTE: All tests run in headless mode (no windows)")
    print("      Set useDisplay=True to see visualization\n")
    
    # Test 1: Normal speed (baseline)
    print("\nTest 1: Normal Speed (1x)")
    print("-" * 70)
    engine = Engine(
        size=Size(w=800, h=600),
        backgroundColor=Color(30, 30, 30),
        windowName="ML Training Demo - Normal Speed",
        fpsCap=60,
        useDisplay=False  # Headless
    )
    engine.set_time_scale(1.0)
    create_simple_scene(engine)
    engine.start()
    
    # Test 2: 10x speed headless (typical ML training)
    print("\nTest 2: ML Training Mode (10x speed, Headless)")
    print("-" * 70)
    engine = Engine(
        size=Size(w=800, h=600),
        backgroundColor=Color(30, 30, 30),
        windowName="ML Training Demo",
        fpsCap=60,
        useDisplay=False  # Headless mode
    )
    engine.set_time_scale(10.0)  # 10x speed
    create_simple_scene(engine)
    engine.start()
    
    # Test 3: 100x speed headless uncapped (extreme training)
    print("\nTest 3: Extreme ML Training (100x speed, Headless, Uncapped FPS)")
    print("-" * 70)
    engine = Engine(
        size=Size(w=800, h=600),
        backgroundColor=Color(30, 30, 30),
        windowName="ML Training Demo",
        fpsCap=0,  # Uncapped FPS
        useDisplay=False  # Headless mode
    )
    engine.set_time_scale(100.0)  # 100x speed
    create_simple_scene(engine)
    engine.start()
    
    print("\n" + "="*70)
    print(" ALL TESTS COMPLETE")
    print("="*70)
    print("\nKey Takeaways:")
    print("  • Time scaling allows faster-than-realtime simulation")
    print("  • Headless mode removes rendering overhead")
    print("  • Physics simulation scales correctly with time_scale")
    print("  • Perfect for ML/deep reinforcement learning training!")
    print("="*70 + "\n")


if __name__ == "__main__":
    main()

