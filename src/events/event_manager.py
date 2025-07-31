from collections import defaultdict, deque
import weakref
import threading
from ..core.runnable import Priority
from .event import Event

class WeakMethod:
    """
    Wrapper for method references that prevents memory leaks by using weak references.
    Automatically handles cleanup when the bound object is garbage collected.
    """
    def __init__(self, method):
        self._weak_instance = weakref.ref(method.__self__)
        self._method_name = method.__func__.__name__

    def __call__(self):
        instance = self._weak_instance()
        if instance is not None:
            return getattr(instance, self._method_name)
        return None

class EventManager:
    """
    Manages event subscription, dispatch, and processing with thread-safe operations.
    Supports priority-based event handling and automatic cleanup of dead references.
    """
    def __init__(self):
        self._listeners = defaultdict(list)  # Event type -> list of (priority, weak_listener) tuples
        self._queue = deque()                # Pending events for deferred processing
        self._lock = threading.RLock()       # Thread safety for concurrent access

    def subscribe(self, event_type: str, listener: callable, priority: Priority = Priority.NORMAL):
        """
        Subscribe a listener to an event type with optional priority.
        
        Args:
            event_type: Type of event to listen for
            listener: Callable to invoke when event occurs
            priority: Priority level for event processing order
        """
        with self._lock:
            # Convert bound methods to weak references to prevent memory leaks
            if hasattr(listener, '__self__') and hasattr(listener, '__func__'):
                weak_listener = WeakMethod(listener)
            else:
                weak_listener = weakref.ref(listener)
            self._listeners[event_type].append((priority.value, weak_listener))

    def unsubscribe(self, event_type: str, listener: callable):
        """
        Remove a listener from an event type subscription.
        
        Args:
            event_type: Type of event to unsubscribe from
            listener: Callable to remove from subscription
            
        Returns:
            bool: True if listener was found and removed, False otherwise
        """
        with self._lock:
            for i, (prio, wl) in enumerate(self._listeners[event_type]):
                deref = wl() if callable(wl) else wl()
                if deref == listener:
                    del self._listeners[event_type][i]
                    return True
            return False

    def dispatch(self, event_type: str, data: dict = None, immediate: bool = False):
        """
        Dispatch an event to all subscribed listeners.
        
        Args:
            event_type: Type of event to dispatch
            data: Optional data payload for the event
            immediate: If True, process event immediately; otherwise queue for later
        """
        event = Event(type=event_type, data=data)
        if immediate:
            with self._lock:
                self._process_event(event)
        else:
            with self._lock:
                self._queue.append(event)

    def process_queue(self):
        """
        Process all queued events in FIFO order.
        Should be called regularly to handle deferred event processing.
        """
        with self._lock:
            while self._queue:
                event = self._queue.popleft()
                self._process_event(event)

    def _process_event(self, event: Event):
        """
        Internal method to process a single event through all registered listeners.
        Handles priority ordering and cleanup of dead references.
        
        Args:
            event: Event instance to process
        """
        with self._lock:
            event_type = event.type
            if event_type not in self._listeners:
                return
            # Sort listeners by priority (highest first)
            sorted_listeners = sorted(self._listeners[event_type], key=lambda x: x[0], reverse=True)
            to_remove = []
            for prio, weak_listener in sorted_listeners:
                # Dereference weak listener
                if callable(weak_listener):
                    listener = weak_listener()
                else:
                    listener = weak_listener()
                if listener is not None:
                    listener(event)
                else:
                    # Mark dead references for removal
                    to_remove.append((prio, weak_listener))
            # Clean up dead references
            for item in to_remove:
                if item in self._listeners[event_type]:
                    self._listeners[event_type].remove(item)

