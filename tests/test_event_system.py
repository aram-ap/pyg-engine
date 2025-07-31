import unittest
import time
import threading
import weakref
from collections import defaultdict, deque
from dataclasses import dataclass, field
import gc  # For forcing garbage collection in tests

# Mock Priority enum (replace with your actual import)
class Priority:
    LOW = 1
    NORMAL = 2
    HIGH = 3

# Event dataclass (from Step 1)
@dataclass(frozen=True)
class Event:
    type: str
    data: dict = None
    timestamp: float = field(default_factory=lambda: time.time())

# Helper for weak references to bound methods
class WeakMethod:
    def __init__(self, method):
        self._weak_instance = weakref.ref(method.__self__)
        self._method_name = method.__func__.__name__

    def __call__(self):
        instance = self._weak_instance()
        if instance is not None:
            return getattr(instance, self._method_name)
        return None

# EventManager class (updated with WeakMethod support for bound methods)
class EventManager:
    def __init__(self):
        self._listeners = defaultdict(list)
        self._queue = deque()
        self._lock = threading.RLock()

    def subscribe(self, event_type: str, listener: callable, priority: Priority = Priority.NORMAL):
        with self._lock:
            if hasattr(listener, '__self__') and hasattr(listener, '__func__'):  # It's a bound method
                weak_listener = WeakMethod(listener)
            else:
                weak_listener = weakref.ref(listener)
            self._listeners[event_type].append((priority, weak_listener))

    def unsubscribe(self, event_type: str, listener: callable):
        with self._lock:
            # For unsubscribe, we need to compare the listener; this is trickier for methods, but for test simplicity, we'll approximate
            for i, (prio, wl) in enumerate(self._listeners[event_type]):
                deref = wl() if callable(wl) else wl()  # Handle both WeakMethod and ref
                if deref == listener:
                    del self._listeners[event_type][i]
                    return True
            return False

    def dispatch(self, event_type: str, data: dict = None, immediate: bool = False):
        event = Event(type=event_type, data=data)
        if immediate:
            with self._lock:
                self._process_event(event)
        else:
            with self._lock:
                self._queue.append(event)

    def process_queue(self):
        with self._lock:
            while self._queue:
                event = self._queue.popleft()
                self._process_event(event)

    def _process_event(self, event: Event):
        with self._lock:
            event_type = event.type
            if event_type not in self._listeners:
                return
            # Sort by priority (higher first; assuming higher number = higher priority)
            sorted_listeners = sorted(self._listeners[event_type], key=lambda x: x[0], reverse=True)
            to_remove = []
            for prio, weak_listener in sorted_listeners:
                # Dereference: Handle both WeakMethod (callable) and plain weakref
                if callable(weak_listener):  # It's a WeakMethod
                    listener = weak_listener()
                else:
                    listener = weak_listener()
                if listener is not None:
                    listener(event)
                else:
                    to_remove.append((prio, weak_listener))
            for item in to_remove:
                if item in self._listeners[event_type]:
                    self._listeners[event_type].remove(item)

# Unit tests
class TestEventManager(unittest.TestCase):
    def setUp(self):
        self.em = EventManager()
        self.calls = []  # To track listener calls

    def test_subscribe_and_dispatch_immediate(self):
        def listener(event):
            self.calls.append((event.type, event.data))

        self.em.subscribe("test_event", listener, Priority.NORMAL)
        self.em.dispatch("test_event", {"key": "value"}, immediate=True)

        self.assertEqual(len(self.calls), 1)
        self.assertEqual(self.calls[0], ("test_event", {"key": "value"}))

    def test_queued_dispatch_and_process(self):
        def listener(event):
            self.calls.append(event.type)

        self.em.subscribe("queued_event", listener)
        self.em.dispatch("queued_event")
        self.assertEqual(len(self.calls), 0)  # Not processed yet

        self.em.process_queue()
        self.assertEqual(len(self.calls), 1)
        self.assertEqual(self.calls[0], "queued_event")

    def test_priority_ordering(self):
        def low(event): self.calls.append("low")
        def normal(event): self.calls.append("normal")
        def high(event): self.calls.append("high")

        self.em.subscribe("priority_event", low, Priority.LOW)
        self.em.subscribe("priority_event", normal, Priority.NORMAL)
        self.em.subscribe("priority_event", high, Priority.HIGH)

        self.em.dispatch("priority_event", immediate=True)
        self.assertEqual(self.calls, ["high", "normal", "low"])

    def test_unsubscribe(self):
        def listener(event):
            self.calls.append("called")

        self.em.subscribe("unsub_event", listener)
        self.em.unsubscribe("unsub_event", listener)
        self.em.dispatch("unsub_event", immediate=True)

        self.assertEqual(len(self.calls), 0)

    def test_weakref_cleanup(self):
        # Use a closure to capture the test's 'calls' list
        calls = self.calls

        class TempObject:
            def on_event(self, event):
                calls.append("called")  # Append to captured calls

        obj = TempObject()
        self.em.subscribe("weak_event", obj.on_event)
        self.em.dispatch("weak_event", immediate=True)
        self.assertEqual(len(self.calls), 1, "First dispatch should call the listener")

        del obj  # Destroy object
        gc.collect()  # Force GC to clear weakrefs

        self.em.dispatch("weak_event", immediate=True)  # Should skip and clean up
        self.assertEqual(len(self.calls), 1, "Second dispatch should not call (object deleted)")

    def test_thread_safety(self):
        def listener(event):
            with lock:  # Use a lock to append safely
                self.calls.append(event.type)

        lock = threading.Lock()  # For test calls list
        self.em.subscribe("thread_event", listener)

        def dispatch_thread():
            for _ in range(10):
                self.em.dispatch("thread_event")
                time.sleep(0.001)  # Simulate work

        threads = [threading.Thread(target=dispatch_thread) for _ in range(5)]
        for t in threads:
            t.start()
        for t in threads:
            t.join()

        self.em.process_queue()
        self.assertEqual(len(self.calls), 50)  # 5 threads * 10 dispatches

if __name__ == "__main__":
    unittest.main()

