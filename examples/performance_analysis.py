#!/usr/bin/env python3
"""
Command-line Performance Analysis for Physics System
Tests performance with large numbers of physics bodies without GUI.
"""

import time
import psutil
import os
import pymunk
from pyg_engine.pymunk_physics_system import PhysicsSystem
from pyg_engine.rigidbody import RigidBody
from pyg_engine.collider import CircleCollider, BoxCollider
from pyg_engine.material import Materials

def get_memory_usage():
    """Get current memory usage in MB."""
    process = psutil.Process(os.getpid())
    return process.memory_info().rss / 1024 / 1024

def create_test_body(physics_system, position, is_circle=True):
    """Create a test physics body."""
    # Create a simple body for testing
    if is_circle:
        body = pymunk.Body(1.0, pymunk.moment_for_circle(1.0, 0, 10))
        shape = pymunk.Circle(body, 10)
    else:
        body = pymunk.Body(1.0, pymunk.moment_for_box(1.0, (20, 20)))
        shape = pymunk.Poly.create_box(body, (20, 20))
    
    body.position = position
    physics_system.space.add(body, shape)
    return body, shape

def test_physics_performance():
    """Test physics system performance with increasing body counts."""
    
    print("Physics Performance Analysis")
    print("=" * 50)
    
    # Test different body counts
    body_counts = [10, 25, 50, 100, 200, 500]
    
    for count in body_counts:
        print(f"\nTesting with {count} bodies...")
        
        # Create physics system
        physics_system = PhysicsSystem()
        bodies = []
        shapes = []
        
        # Measure memory before
        memory_before = get_memory_usage()
        
        # Create bodies
        start_time = time.time()
        for i in range(count):
            x = (i % 10) * 60 - 300
            y = (i // 10) * 60 - 200
            body, shape = create_test_body(physics_system, (x, y), i % 2 == 0)
            bodies.append(body)
            shapes.append(shape)
        creation_time = time.time() - start_time
        
        # Measure memory after creation
        memory_after_creation = get_memory_usage()
        
        # Test physics step performance
        step_times = []
        for _ in range(100):  # 100 physics steps
            start_time = time.time()
            physics_system.space.step(1.0/60.0)
            step_time = time.time() - start_time
            step_times.append(step_time * 1000)  # Convert to milliseconds
        
        # Calculate averages
        avg_step_time = sum(step_times) / len(step_times)
        min_step_time = min(step_times)
        max_step_time = max(step_times)
        
        # Measure memory after physics steps
        memory_after_physics = get_memory_usage()
        
        # Calculate theoretical FPS
        theoretical_fps = 1000 / avg_step_time if avg_step_time > 0 else 0
        
        print(f"  Creation time: {creation_time*1000:.2f}ms")
        print(f"  Memory before: {memory_before:.1f}MB")
        print(f"  Memory after creation: {memory_after_creation:.1f}MB")
        print(f"  Memory after physics: {memory_after_physics:.1f}MB")
        print(f"  Memory per body: {(memory_after_creation - memory_before) / count:.2f}MB")
        print(f"  Avg step time: {avg_step_time:.2f}ms")
        print(f"  Min step time: {min_step_time:.2f}ms")
        print(f"  Max step time: {max_step_time:.2f}ms")
        print(f"  Theoretical FPS: {theoretical_fps:.1f}")
        
        # Performance assessment
        if theoretical_fps >= 60:
            status = "GOOD"
        elif theoretical_fps >= 30:
            status = "ACCEPTABLE"
        else:
            status = "POOR"
        
        print(f"  Performance: {status}")
        
        # Clean up
        for body, shape in zip(bodies, shapes):
            physics_system.space.remove(body, shape)
        del physics_system
        bodies.clear()
        shapes.clear()

def test_collision_performance():
    """Test collision detection performance."""
    
    print("\n\nCollision Performance Analysis")
    print("=" * 50)
    
    physics_system = PhysicsSystem()
    
    # Create a grid of bodies that will collide
    bodies = []
    shapes = []
    
    for i in range(10):  # 10x10 grid = 100 bodies
        for j in range(10):
            x = i * 30
            y = j * 30
            body, shape = create_test_body(physics_system, (x, y), True)
            bodies.append(body)
            shapes.append(shape)
    
    print(f"Created {len(bodies)} bodies in collision grid")
    
    # Test collision detection performance
    step_times = []
    collision_counts = []
    
    for step in range(100):
        start_time = time.time()
        physics_system.space.step(1.0/60.0)
        step_time = time.time() - start_time
        step_times.append(step_time * 1000)
        
        # Count active collisions (approximate)
        # Note: pymunk doesn't expose arbiters directly, so we'll estimate
        active_collisions = len(physics_system.space.shapes) // 2  # Rough estimate
        collision_counts.append(active_collisions)
    
    avg_step_time = sum(step_times) / len(step_times)
    avg_collisions = sum(collision_counts) / len(collision_counts)
    max_collisions = max(collision_counts)
    
    print(f"  Bodies: {len(bodies)}")
    print(f"  Avg step time: {avg_step_time:.2f}ms")
    print(f"  Avg active collisions: {avg_collisions:.1f}")
    print(f"  Max active collisions: {max_collisions}")
    print(f"  Theoretical FPS: {1000/avg_step_time:.1f}")
    
    # Clean up
    for body, shape in zip(bodies, shapes):
        physics_system.space.remove(body, shape)

def main():
    """Run performance analysis."""
    print("Starting Physics Performance Analysis...")
    print("This will test the physics system with various body counts.")
    
    try:
        test_physics_performance()
        test_collision_performance()
        
        print("\n" + "=" * 50)
        print("Performance Analysis Complete!")
        print("\nRecommendations:")
        print("- If FPS drops below 30 with 100+ bodies, consider:")
        print("  * Reducing physics iterations")
        print("  * Using simpler collision shapes")
        print("  * Implementing object pooling")
        print("  * Adding spatial partitioning")
        print("  * Reducing physics update frequency")
        
    except Exception as e:
        print(f"Error during performance analysis: {e}")

if __name__ == "__main__":
    main() 