# Physics System Tests

This directory contains comprehensive tests for the physics system, specifically focusing on rotation behavior and coordinate system fixes.

[NOTE: TESTS UNDER DEVELOPMENT, DO NOT EXPECT GREAT RESULTS YET]

## Test Files

### `test_rotation_fix.py`
**Purpose**: Tests the original rotation fix for the counter-clockwise rotation issue
- Tests basic rotation prevention during movement
- Verifies that the player doesn't rotate when moving on surfaces

### `test_rotation_direction.py`
**Purpose**: Tests the corrected rotation direction for balls
- Verifies that pushing left causes counter-clockwise rotation
- Verifies that pushing right causes clockwise rotation
- Tests natural rolling physics due to friction

### `test_rectangle_rotation.py`
**Purpose**: Tests rectangle rotation behavior specifically
- Tests rectangle rotation direction (fixed vertex order)
- Compares locked vs unlocked rotation for rectangles
- Tests different rectangle shapes (wide, tall)

### `test_rotation_system.py`
**Purpose**: Tests the complete rotation lock system
- Demonstrates both natural physics and locked rotation
- Shows the `lock_rotation` parameter functionality
- Tests circles and rectangles with both settings

### `test_comprehensive_rotation.py`
**Purpose**: Comprehensive test of all rotation behaviors
- Tests circles and rectangles together
- Shows natural physics vs locked rotation
- Demonstrates the complete rotation system working correctly

## Running Tests

All tests can be run from the project root directory:

```bash
python tests/test_rotation_direction.py
python tests/test_rectangle_rotation.py
python tests/test_comprehensive_rotation.py
```

## Expected Behavior

### Natural Physics (`lock_rotation=False`)
- **Circles**: Roll naturally due to friction when pushed
- **Rectangles**: Rotate naturally when pushed or falling
- **Direction**: Push LEFT = Counter-clockwise, Push RIGHT = Clockwise

### Locked Rotation (`lock_rotation=True`)
- **Circles**: Slide without rolling
- **Rectangles**: Slide without rotating
- **Use case**: Player characters, controlled objects

## Key Fixes Implemented

1. **Coordinate System Fix**: Corrected Pymunk/Pygame coordinate conversion
2. **Vertex Order Fix**: Fixed rectangle vertex order for proper rotation
3. **Rotation Lock System**: Unity-style rotation constraints
4. **Moment of Inertia**: Proper calculation for different shapes

## Test Results

All tests should run without errors and demonstrate:
- ✅ Correct rotation direction for all shapes
- ✅ Natural physics behavior when unlocked
- ✅ Stable movement when rotation is locked
- ✅ Proper collision detection and response
