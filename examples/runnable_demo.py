#!/usr/bin/env python3
"""
Runnable System Demo
Demonstrates how to use the optimized runnable system in the game engine.
"""

import pygame as pg
from pygame import Vector2
from pyg_engine import Engine, Priority
from pyg_engine import Size, GameObject

def main():
    """Main demo function showing various runnable usage patterns."""
    
    # Create engine
    engine = Engine(size=Size(w=800, h=600), windowName="Runnable Demo")
    
    # ===== BASIC USAGE EXAMPLES =====
    
    # 1. Simple one-time function on key press
    def print_hello(engine):
        print("Hello from runnable!")
    
    engine.add_runnable(print_hello, 'key_press', Priority.NORMAL, key=pg.K_h)
    
    # 2. Spawn objects with limited runs
    def spawn_enemy(engine):
        from pygame import Vector2
        import random
        
        x = random.randint(0, engine.getWindowSize().w)
        y = random.randint(0, engine.getWindowSize().h)
        enemy = GameObject("Enemy", position=Vector2(x, y))
        engine.addGameObject(enemy)
        print(f"Enemy spawned! (Run {engine.runnable_system.runnable_queues['key_press'][pg.K_e][0].runs_completed})")
    
    # Spawn 5 enemies then stop
    engine.add_runnable(spawn_enemy, 'key_press', Priority.HIGH, max_runs=5, key=pg.K_e)
    
    # 3. High priority rendering (always runs first)
    def render_fps(engine):
        font = pg.font.Font(None, 36)
        fps = engine.clock.get_fps()
        text = font.render(f"FPS: {fps:.1f}", True, pg.Color(255, 255, 255))
        engine.screen.blit(text, (10, 10))
    
    engine.add_runnable(render_fps, 'render', Priority.CRITICAL)
    
    # 4. One-time initialization
    def setup_game(engine):
        print("Game initialized!")
        # Setup initial game state
        engine.globals.set('score', 0, 'game_state')
        engine.globals.set('level', 1, 'game_state')
    
    engine.add_runnable(setup_game, 'start', Priority.CRITICAL, max_runs=1)
    
    # 5. Periodic physics effect
    def apply_gravity_pulse(engine):
        for obj in engine.getGameObjects():
            if hasattr(obj, 'rigidbody') and obj.rigidbody:
                obj.rigidbody.apply_force(Vector2(0, -500))
        print("Gravity pulse applied!")
    
    engine.add_runnable(apply_gravity_pulse, 'physics_update', Priority.NORMAL, max_runs=3)
    
    # 6. Complex formation spawning
    def spawn_formation(engine):
        import math
        from pygame import Vector2
        
        center = Vector2(engine.getWindowSize().w // 2, engine.getWindowSize().h // 2)
        radius = 100
        
        for i in range(8):
            angle = (i * 45) * math.pi / 180
            x = center.x + radius * math.cos(angle)
            y = center.y + radius * math.sin(angle)
            
            obj = GameObject(f"Formation_{i}", position=Vector2(x, y))
            engine.addGameObject(obj)
        
        print("Formation spawned!")
    
    engine.add_runnable(spawn_formation, 'key_press', Priority.NORMAL, max_runs=2, key=pg.K_f)
    
    # 7. Debug rendering with conditions
    def render_debug_info(engine):
        if len(engine.getGameObjects()) > 5:
            font = pg.font.Font(None, 36)
            text = font.render(f"Objects: {len(engine.getGameObjects())}", True, pg.Color(255, 0, 0))
            engine.screen.blit(text, (10, 50))
    
    engine.add_runnable(render_debug_info, 'render', Priority.LOW)
    
    # 8. Custom event handling
    def handle_custom_event(engine):
        print("Custom event triggered!")
        # Do something special
    
    engine.add_runnable(handle_custom_event, 'custom', Priority.NORMAL, key='special_event')
    
    # Trigger custom event from another function
    def trigger_special(engine):
        engine.runnable_system.execute_runnables('custom', 'special_event')
    
    engine.add_runnable(trigger_special, 'key_press', Priority.NORMAL, key=pg.K_t)
    
    # ===== ADVANCED USAGE EXAMPLES =====
    
    # 9. Lambda functions for quick actions
    engine.add_runnable(
        lambda eng: print("Quick action!"), 
        'key_press', Priority.NORMAL, key=pg.K_q
    )
    
    # 10. Object cleanup
    def cleanup_objects(engine):
        objects = engine.getGameObjects()
        for obj in objects[:]:  # Copy list to avoid modification during iteration
            if obj.position.x < 0 or obj.position.x > engine.getWindowSize().w:
                engine.removeGameObject(obj)
        print("Cleaned up off-screen objects!")
    
    engine.add_runnable(cleanup_objects, 'update', Priority.LOW)
    
    # 11. Score tracking
    def update_score(engine):
        current_score = engine.globals.get('score', 0, 'game_state')
        engine.globals.set('score', current_score + 1, 'game_state')
    
    engine.add_runnable(update_score, 'update', Priority.NORMAL, max_runs=100)  # Stop after 100 updates
    
    # 12. Level progression
    def check_level_up(engine):
        score = engine.globals.get('score', 0, 'game_state')
        level = engine.globals.get('level', 1, 'game_state')
        
        if score >= level * 10:
            engine.globals.set('level', level + 1, 'game_state')
            print(f"Level up! Now level {level + 1}")
    
    engine.add_runnable(check_level_up, 'update', Priority.NORMAL)
    
    # 13. Error handling example
    def error_prone_function(engine):
        # This will cause an error
        raise ValueError("This is a test error!")
    
    def error_handler(error, engine, runnable):
        print(f"Error handled gracefully: {error}")
        # Could log to file, send to server, etc.
    
    engine.add_runnable(error_prone_function, 'key_press', Priority.NORMAL, 
                       key=pg.K_x, error_handler=error_handler)
    
    # 14. Global error handler
    def global_error_handler(error, runnable_system, runnable):
        print(f"Global error handler caught: {error}")
        print(f"Function: {runnable.func.__name__}")
    
    engine.add_error_handler(global_error_handler)
    
    # 15. Debug mode (stricter error handling)
    engine.set_debug_mode(True)
    
    # Start the engine
    print("Starting Runnable Demo...")
    print("Controls:")
    print("  H - Print hello")
    print("  E - Spawn enemies (5 times)")
    print("  F - Spawn formation (2 times)")
    print("  T - Trigger custom event")
    print("  Q - Quick action")
    print("  X - Trigger error (for testing)")
    print("  ESC - Quit")
    
    engine.start()

if __name__ == "__main__":
    main() 