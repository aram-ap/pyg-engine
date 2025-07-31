import heapq
import traceback
from enum import Enum
from dataclasses import dataclass
from typing import Callable, Optional

class Priority(Enum):
    """Priority levels for runnable execution order."""
    LOW = 3
    NORMAL = 2
    HIGH = 1
    CRITICAL = 0

@dataclass
class Runnable:
    """A runnable function with priority, execution limits, and error handling."""
    func: Callable
    priority: Priority = Priority.NORMAL
    max_runs: Optional[int] = None
    runs_completed: int = 0
    event_type: str = "update"
    key: Optional[int] = None
    error_handler: Optional[Callable] = None
    error_count: int = 0
    max_errors: int = 3
    
    def __lt__(self, other):
        """Compare runnables by priority for sorting."""
        return self.priority.value < other.priority.value
    
    def execute(self, engine) -> bool:
        """Execute the runnable with exception handling. Returns True if should continue."""
        try:
            if self.max_runs is None or self.runs_completed < self.max_runs:
                self.func(engine)
                self.runs_completed += 1
                self.error_count = 0  # Reset error count on success
                return self.max_runs is None or self.runs_completed < self.max_runs
            return False
        except Exception as e:
            self.error_count += 1
            self._handle_error(e, engine)
            return self.error_count < self.max_errors
    
    def _handle_error(self, error: Exception, engine):
        """Handle execution errors with logging and custom handlers."""
        error_msg = f"Runnable error in {self.event_type}: {error}"
        
        # Log the error
        print(f"ERROR: {error_msg}")
        print(f"Function: {self.func.__name__}")
        print(f"Error count: {self.error_count}/{self.max_errors}")
        traceback.print_exc()
        
        # Call custom error handler if provided
        if self.error_handler:
            try:
                self.error_handler(error, engine, self)
            except Exception as handler_error:
                print(f"Error handler also failed: {handler_error}")
        
        # Remove from engine if too many errors
        if self.error_count >= self.max_errors:
            print(f"Removing runnable after {self.max_errors} errors")
            self._remove_from_engine(engine)
    
    def _remove_from_engine(self, engine):
        """Remove this runnable from the engine queues."""
        if self.event_type in ['start', 'update', 'render', 'physics_update']:
            if self in engine.runnable_system.runnable_queues[self.event_type]:
                engine.runnable_system.runnable_queues[self.event_type].remove(self)
        elif self.event_type in ['key_press', 'key_release']:
            if self.key and self.key in engine.runnable_system.runnable_queues[self.event_type]:
                if self in engine.runnable_system.runnable_queues[self.event_type][self.key]:
                    engine.runnable_system.runnable_queues[self.event_type][self.key].remove(self)

        elif self.event_type == 'custom':
            if self.key and self.key in engine.runnable_system.runnable_queues.get('custom', {}):
                if self in engine.runnable_system.runnable_queues['custom'][self.key]:
                    engine.runnable_system.runnable_queues['custom'][self.key].remove(self)

class RunnableSystem:
    """Optimized runnable system with priority queues and error handling."""
    
    def __init__(self):
        self.runnable_queues = {
            'start': [],
            'update': [],
            'render': [],
            'key_press': {},
            'key_release': {},
            'physics_update': [],
            'custom': {}
        }
        self._queue_dirty = {
            'start': False,
            'update': False,
            'render': False,
            'key_press': {},
            'key_release': {},
            'physics_update': False,
            'custom': {}
        }
        self.error_callbacks = []
        self.debug_mode = False
    
    def add_runnable(self, func: Callable, event_type: str = "update", 
                    priority: Priority = Priority.NORMAL, max_runs: Optional[int] = None,
                    key: Optional[int] = None, error_handler: Optional[Callable] = None):
        """Add a runnable with priority, execution limits, and error handling."""
        runnable = Runnable(func, priority, max_runs, 0, event_type, key, error_handler)
        
        if event_type in ['start', 'update', 'render', 'physics_update']:
            self.runnable_queues[event_type].append(runnable)  # O(1)
            self._queue_dirty[event_type] = True  # Mark for sorting
        elif event_type in ['key_press', 'key_release']:
            if key is None:
                raise ValueError("Key required for key events")
            if key not in self.runnable_queues[event_type]:
                self.runnable_queues[event_type][key] = []
                self._queue_dirty[event_type][key] = False
            self.runnable_queues[event_type][key].append(runnable)  # O(1)
            self._queue_dirty[event_type][key] = True  # Mark for sorting

        elif event_type == 'custom':
            if key is None:
                raise ValueError("Custom event key required")
            if 'custom' not in self.runnable_queues:
                self.runnable_queues['custom'] = {}
                self._queue_dirty['custom'] = {}
            if key not in self.runnable_queues['custom']:
                self.runnable_queues['custom'][key] = []
                self._queue_dirty['custom'][key] = False
            self.runnable_queues['custom'][key].append(runnable)  # O(1)
            self._queue_dirty['custom'][key] = True  # Mark for sorting
    
    def _ensure_queue_sorted(self, event_type: str, key: Optional[int] = None):
        """Sort queue only when dirty - O(n log n) only when needed."""
        if event_type in ['start', 'update', 'render', 'physics_update']:
            if self._queue_dirty[event_type]:
                self.runnable_queues[event_type].sort()  # O(n log n)
                self._queue_dirty[event_type] = False
        elif event_type in ['key_press', 'key_release']:
            if key is not None and key in self._queue_dirty[event_type]:
                if self._queue_dirty[event_type][key]:
                    self.runnable_queues[event_type][key].sort()  # O(n log n)
                    self._queue_dirty[event_type][key] = False

        elif event_type == 'custom':
            if key is not None and key in self._queue_dirty.get('custom', {}):
                if self._queue_dirty['custom'][key]:
                    self.runnable_queues['custom'][key].sort()  # O(n log n)
                    self._queue_dirty['custom'][key] = False
    
    def execute_runnables(self, event_type: str, key: Optional[int] = None, engine=None):
        """Execute all runnables for an event type with comprehensive error handling."""
        try:
            # Get queue
            if event_type in ['start', 'update', 'render', 'physics_update']:
                queue = self.runnable_queues[event_type]
            elif event_type in ['key_press', 'key_release']:
                if key is None or key not in self.runnable_queues[event_type]:
                    return  # EARLY EXIT - O(1) when empty
                queue = self.runnable_queues[event_type][key]

            elif event_type == 'custom':
                if key is None or key not in self.runnable_queues.get('custom', {}):
                    return  # EARLY EXIT - O(1) when empty
                queue = self.runnable_queues['custom'][key]
            else:
                return  # EARLY EXIT - O(1)
            
            # Early exit if empty
            if not queue:
                return  # O(1) - No CPU time wasted
            
            # Sort only if dirty
            self._ensure_queue_sorted(event_type, key)
            
            # Execute runnables in order (no heap operations!)
            expired_indices = []
            for i, runnable in enumerate(queue):
                try:
                    # Execute the runnable with engine parameter
                    if not runnable.execute(engine):  # Returns False if should be removed
                        expired_indices.append(i)
                except Exception as e:
                    # Handle errors in the execution loop itself
                    self._handle_execution_error(e, runnable, event_type, key)
                    expired_indices.append(i)
            
            # Remove expired/errored runnables
            for i in reversed(expired_indices):
                del queue[i]  # O(n) but only when removing
                
        except Exception as e:
            # Handle errors in the execution system itself
            print(f"CRITICAL ERROR in runnable execution system: {e}")
            traceback.print_exc()
    
    def _handle_execution_error(self, error: Exception, runnable: Runnable, 
                              event_type: str, key: Optional[int] = None):
        """Handle errors in the execution system."""
        error_msg = f"Execution error in {event_type} runnable: {error}"
        print(f"CRITICAL: {error_msg}")
        print(f"Function: {runnable.func.__name__}")
        traceback.print_exc()
        
        # Call global error handlers
        for handler in self.error_callbacks:
            try:
                handler(error, self, runnable)
            except Exception as handler_error:
                print(f"Global error handler failed: {handler_error}")
        
        # In debug mode, stop the engine
        if self.debug_mode:
            print("Stopping engine due to error in debug mode")
            # Note: We don't have direct access to engine.stop() here
            # This would need to be handled by the engine calling this method
    
    def add_error_handler(self, handler: Callable):
        """Add global error handler."""
        self.error_callbacks.append(handler)
    
    def set_debug_mode(self, enabled: bool):
        """Enable/disable debug mode for stricter error handling."""
        self.debug_mode = enabled
    
    def get_queue_stats(self) -> dict:
        """Get statistics about runnable queues."""
        stats = {}
        for event_type, queue in self.runnable_queues.items():
            if isinstance(queue, list):
                stats[event_type] = len(queue)
            elif isinstance(queue, dict):
                stats[event_type] = sum(len(q) for q in queue.values())
        return stats
    
    def clear_queue(self, event_type: str, key: Optional[int] = None):
        """Clear all runnables from a specific queue."""
        if event_type in ['start', 'update', 'render', 'physics_update']:
            self.runnable_queues[event_type].clear()
            self._queue_dirty[event_type] = False
        elif event_type in ['key_press', 'key_release']:
            if key is not None and key in self.runnable_queues[event_type]:
                self.runnable_queues[event_type][key].clear()
                self._queue_dirty[event_type][key] = False
        elif event_type == 'custom':
            if key is not None and key in self.runnable_queues.get('custom', {}):
                self.runnable_queues['custom'][key].clear()
                self._queue_dirty['custom'][key] = False 