try:
    from pyg import _native
except ImportError:
    import _native # Fallback?

from .core.engine import Engine
