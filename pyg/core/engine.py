try:
    from pyg import _native
except ImportError:
    # Fallback for direct import or weird paths
    import _native

class Engine:
    def __init__(self):
        self._core = _native.Core()
    
    @property
    def version(self):
        return self._core.get_version()
