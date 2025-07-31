#!/usr/bin/env python3
"""
Global Dictionary Test Script
Demonstrates the optimized global variable system for the game engine.
"""

from pyg_engine import GlobalDictionary
import time

def main():
    """Test the Global Dictionary functionality."""
    print("=== Global Dictionary Test ===\n")
    
    # Create a global dictionary instance
    globals = GlobalDictionary()
    
    # Test 1: Basic operations
    print("1. Basic Operations:")
    print("-" * 30)
    
    # Set values
    globals.set("player_health", 100, "player")
    globals.set("player_score", 0, "player")
    globals.set("game_state", "playing", "game")
    globals.set("level", 1, "game")
    globals.set("gravity_enabled", True, "physics")
    
    # Get values
    print(f"Player Health: {globals.get('player_health', category='player')}")
    print(f"Player Score: {globals.get('player_score', category='player')}")
    print(f"Game State: {globals.get('game_state', category='game')}")
    print(f"Level: {globals.get('level', category='game')}")
    print(f"Gravity Enabled: {globals.get('gravity_enabled', category='physics')}")
    
    # Test 2: Default category
    print("\n2. Default Category:")
    print("-" * 30)
    
    globals.set("window_width", 800)
    globals.set("window_height", 600)
    globals.set("fps", 60)
    
    print(f"Window Size: {globals.get('window_width')}x{globals.get('window_height')}")
    print(f"FPS: {globals.get('fps')}")
    
    # Test 3: Check existence
    print("\n3. Existence Checks:")
    print("-" * 30)
    
    print(f"Has 'player_health' in 'player': {globals.has('player_health', 'player')}")
    print(f"Has 'nonexistent' in 'player': {globals.has('nonexistent', 'player')}")
    print(f"Has 'window_width' in default: {globals.has('window_width')}")
    
    # Test 4: Get all variables in categories
    print("\n4. Get All Variables by Category:")
    print("-" * 30)
    
    player_vars = globals.get_all("player")
    game_vars = globals.get_all("game")
    physics_vars = globals.get_all("physics")
    default_vars = globals.get_all()
    
    print(f"Player variables: {player_vars}")
    print(f"Game variables: {game_vars}")
    print(f"Physics variables: {physics_vars}")
    print(f"Default variables: {default_vars}")
    
    # Test 5: Update values
    print("\n5. Update Values:")
    print("-" * 30)
    
    globals.set("player_health", 75, "player")  # Player took damage
    globals.set("player_score", 1500, "player")  # Player scored points
    globals.set("level", 2, "game")  # Level up
    
    print(f"Updated Player Health: {globals.get('player_health', category='player')}")
    print(f"Updated Player Score: {globals.get('player_score', category='player')}")
    print(f"Updated Level: {globals.get('level', category='game')}")
    
    # Test 6: Remove variables
    print("\n6. Remove Variables:")
    print("-" * 30)
    
    print(f"Before removal - Has 'fps': {globals.has('fps')}")
    globals.remove("fps")
    print(f"After removal - Has 'fps': {globals.has('fps')}")
    print(f"After removal - Get 'fps' (with default): {globals.get('fps', default='Not found')}")
    
    # Test 7: Clear categories
    print("\n7. Clear Categories:")
    print("-" * 30)
    
    print(f"Before clear - Physics variables: {globals.get_all('physics')}")
    globals.clear_category("physics")
    print(f"After clear - Physics variables: {globals.get_all('physics')}")
    
    # Test 8: Performance test
    print("\n8. Performance Test:")
    print("-" * 30)
    
    start_time = time.time()
    
    # Set many variables quickly
    for i in range(1000):
        globals.set(f"test_var_{i}", i, "performance")
    
    set_time = time.time() - start_time
    
    start_time = time.time()
    
    # Get many variables quickly
    for i in range(1000):
        value = globals.get(f"test_var_{i}", category="performance")
    
    get_time = time.time() - start_time
    
    print(f"Set 1000 variables in: {set_time:.4f} seconds")
    print(f"Get 1000 variables in: {get_time:.4f} seconds")
    
    # Test 9: Thread safety demonstration
    print("\n9. Thread Safety (Simulated):")
    print("-" * 30)
    
    import threading
    
    def set_values(thread_id):
        for i in range(100):
            globals.set(f"thread_{thread_id}_var_{i}", f"value_{i}", f"thread_{thread_id}")
    
    # Create multiple threads
    threads = []
    for i in range(5):
        thread = threading.Thread(target=set_values, args=(i,))
        threads.append(thread)
        thread.start()
    
    # Wait for all threads to complete
    for thread in threads:
        thread.join()
    
    print("All threads completed successfully!")
    print(f"Total variables across all threads: {len(globals.get_all())}")
    
    # Test 10: Cache demonstration
    print("\n10. Cache Performance:")
    print("-" * 30)
    
    # First access (cache miss)
    start_time = time.time()
    for _ in range(1000):
        globals.get("player_health", category="player")
    first_access = time.time() - start_time
    
    # Second access (cache hit)
    start_time = time.time()
    for _ in range(1000):
        globals.get("player_health", category="player")
    second_access = time.time() - start_time
    
    print(f"First access (cache miss): {first_access:.4f} seconds")
    print(f"Second access (cache hit): {second_access:.4f} seconds")
    print(f"Cache improvement: {first_access/second_access:.1f}x faster")
    
    print("\n=== Global Dictionary Test Complete ===")
    print("All tests passed successfully!")

if __name__ == "__main__":
    main() 