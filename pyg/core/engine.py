try:
    from pyg import _native
except ImportError:
    # Fallback for direct import or weird paths
    import _native  # pyright: ignore[reportMissingImports]

LogType = _native.LogType

class Engine:
    def __init__(self, name: str = "Pyg-Engine", tick_rate: int = 60):
        self._engine = _native.Engine()
        self.tick_rate = tick_rate
        self.input = Input()

        if name == "Pyg-Engine":
            self._name = f"Pyg-Engine v{self.version}"
        else:
            self._name = name

    def start(self):
        self._engine.log_type(LogType.Info, f"Engine started: {self.name}")

    def stop(self):
        self._engine.log_type(LogType.Info, f"Engine stopping: {self.name}")
        self._engine.log_type(LogType.Info, f"Engine stopped: {self.name}")
        self._engine.stop()

    @property
    def tick_rate(self) -> int:
        return self._engine.tick_rate

    @tick_rate.setter
    def tick_rate(self, tick_rate: int):
        if tick_rate <= 0:
            self._engine.log_type(LogType.Error, f"Tick rate must be positive, got {tick_rate}")
            raise ValueError("tick_rate must be positive")
        self._engine.tick_rate = tick_rate

    def set_window_title(self, title: str):
        self._engine.set_window_title(title)

    def get_window_title(self) -> str:
        return self._engine.get_window_title()

    def log(self, message):
        # Log to the engine
        self._engine.log(str(message))

    def log_type(self, log_type: LogType, message):
        # Log to the engine with LogType enum
        self._engine.log_type(log_type, str(message))

    def log_error(self, message):
        # Log to the engine with LogType.Error
        self._engine.log_type(LogType.Error, str(message))

    def log_warning(self, message):
        # Log to the engine with LogType.Warning
        self._engine.log_type(LogType.Warning, str(message))

    def log_info(self, message):
        # Log to the engine with LogType.Info
        self._engine.log_type(LogType.Info, str(message))

    def log_debug(self, message):
        # Log to the engine with LogType.Debug
        self._engine.log_type(LogType.Debug, str(message))

    def log_trace(self, message):
        # Log to the engine with LogType.Trace
        self._engine.log_type(LogType.Trace, str(message))

    @property
    def name(self) -> str:
        # Returns the window Name
        return self._name

    @property
    def version(self) -> str:
        # Pyg-Engine Version
        # e.g., '0.1.0'
        return self._engine.get_version()

    def __str__(self):
        return f"Engine(version={self.version})"
