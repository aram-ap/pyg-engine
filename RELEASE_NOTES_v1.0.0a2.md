# Pyg Engine v1.0.0a2

## Bug Fix Release

Fixed a bug where GameObject constructor would crash when using tuples for position and size parameters.

**The Bug:** When creating a GameObject with `position=(400, 300)` or `size=(50, 50)`, it would throw `AttributeError: 'tuple' object has no attribute 'x'` because the code expected Vector2 objects.

**The Fix:** Added automatic conversion of tuples and lists to Vector2 objects in the GameObject constructor.

**Before:**
```python
player = GameObject(name="Player", position=(400, 300), size=(50, 50))  # Crashed
```

**After:**
```python
player = GameObject(name="Player", position=(400, 300), size=(50, 50))  # Works
```

This release is fully backward compatible - existing code using Vector2 objects continues to work unchanged.

---

**Pyg Engine v1.0.0a2** - Making 2D game development in Python easier and more flexible! :) 