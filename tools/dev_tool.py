"""
Developer Debug Tool - Main PyQt Application
Provides real-time inspection and editing of game engine state.
"""

import sys
import os
import threading
from typing import Optional

# Add parent directory to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from PyQt6.QtWidgets import (
    QApplication, QMainWindow, QTabWidget, QVBoxLayout, QWidget,
    QPushButton, QHBoxLayout, QLabel, QCheckBox, QSpinBox, QSplitter
)
from PyQt6.QtCore import QTimer, Qt, pyqtSignal, QObject
from PyQt6.QtGui import QFont

from pyg_engine.core.engine import Engine
from pyg_engine.core.gameobject import GameObject
from tools.widgets.gameobject_list import GameObjectListWidget
from tools.widgets.component_inspector import ComponentInspectorWidget
from tools.widgets.engine_stats import EngineStatsWidget
from tools.widgets.serialization_panel import SerializationPanelWidget
from tools.widgets.ui_elements import UIElementsWidget


_qt_app: Optional[QApplication] = None
_qt_window: Optional['DevToolWindow'] = None


class EngineUpdater(QObject):
    """Thread-safe signal emitter for engine updates."""
    update_signal = pyqtSignal()


class DevToolWindow(QMainWindow):
    """Main developer tool window."""
    
    def __init__(self, engine: Engine):
        super().__init__()
        self.engine = engine
        self.debug_interface = engine.debug_interface
        
        # Prevent window from being garbage collected
        self.setAttribute(Qt.WidgetAttribute.WA_DeleteOnClose, False)
        
        # Update timer
        self.update_timer = QTimer()
        self.update_timer.timeout.connect(self._update_all)
        self.refresh_rate = 30  # Updates per second
        
        # Selected GameObject
        self.selected_game_object = None
        
        self._init_ui()
        self._start_updates()
    
    def _init_ui(self):
        """Initialize the UI."""
        self.setWindowTitle("Pyg-Engine Developer Tool")
        self.setGeometry(100, 100, 1200, 800)
        
        # Central widget with tabs
        central_widget = QWidget()
        self.setCentralWidget(central_widget)
        
        layout = QVBoxLayout(central_widget)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.setSpacing(0)
        
        # Control panel at top (fixed height)
        control_panel = self._create_control_panel()
        control_panel.setFixedHeight(50)
        layout.addWidget(control_panel, 0)  # stretch factor = 0
        
        # Main content splitter (GameObjects on left, other panels on right)
        self.content_splitter = QSplitter(Qt.Orientation.Horizontal)
        self.content_splitter.setChildrenCollapsible(False)
        layout.addWidget(self.content_splitter, 1)  # stretch factor = 1
        
        # Create panels
        self._create_panels()
    
    def _create_control_panel(self) -> QWidget:
        """Create the control panel with pause/step controls."""
        panel = QWidget()
        layout = QHBoxLayout(panel)
        
        # Pause/Resume button
        self.pause_button = QPushButton("Pause")
        self.pause_button.clicked.connect(self._toggle_pause)
        layout.addWidget(self.pause_button)
        
        # Step frame button
        self.step_button = QPushButton("Step Frame")
        self.step_button.clicked.connect(self._step_frame)
        self.step_button.setEnabled(False)
        layout.addWidget(self.step_button)
        
        layout.addSpacing(20)
        
        # Refresh rate control
        layout.addWidget(QLabel("Refresh Rate (Hz):"))
        self.refresh_spinbox = QSpinBox()
        self.refresh_spinbox.setMinimum(1)
        self.refresh_spinbox.setMaximum(60)
        self.refresh_spinbox.setValue(self.refresh_rate)
        self.refresh_spinbox.valueChanged.connect(self._update_refresh_rate)
        layout.addWidget(self.refresh_spinbox)
        
        layout.addSpacing(20)
        
        # Auto-refresh checkbox
        self.auto_refresh_checkbox = QCheckBox("Auto-refresh")
        self.auto_refresh_checkbox.setChecked(True)
        self.auto_refresh_checkbox.stateChanged.connect(self._toggle_auto_refresh)
        layout.addWidget(self.auto_refresh_checkbox)
        
        layout.addStretch()
        
        # Status label
        self.status_label = QLabel("Ready")
        layout.addWidget(self.status_label)
        
        return panel
    
    def _create_panels(self):
        """Create the side-by-side panels."""
        # GameObject List tab
        self.gameobject_list = GameObjectListWidget(self.debug_interface)
        self.gameobject_list.selection_changed.connect(self._on_gameobject_selected)
        
        left_panel = QWidget()
        left_layout = QVBoxLayout(left_panel)
        left_layout.setContentsMargins(0, 0, 0, 0)
        left_layout.setSpacing(4)
        left_layout.addWidget(self.gameobject_list)
        self.content_splitter.addWidget(left_panel)
        
        # Component Inspector tab
        self.component_inspector = ComponentInspectorWidget(self.debug_interface)
        
        # UI Elements tab
        self.ui_elements = UIElementsWidget(self.debug_interface)
        
        # Engine Stats tab
        self.engine_stats = EngineStatsWidget(self.debug_interface)
        
        # Serialization tab
        self.serialization_panel = SerializationPanelWidget(self.engine, self.debug_interface)
        
        self.detail_tabs = QTabWidget()
        self.detail_tabs.addTab(self.component_inspector, "Inspector")
        self.detail_tabs.addTab(self.ui_elements, "UI Elements")
        self.detail_tabs.addTab(self.engine_stats, "Stats")
        self.detail_tabs.addTab(self.serialization_panel, "Serialization")
        self.content_splitter.addWidget(self.detail_tabs)
        
        self.content_splitter.setStretchFactor(0, 1)
        self.content_splitter.setStretchFactor(1, 2)
    
    def _toggle_pause(self):
        """Toggle pause state."""
        self.debug_interface.toggle_pause()
        is_paused = self.debug_interface.is_paused()
        
        if is_paused:
            self.pause_button.setText("Resume")
            self.step_button.setEnabled(True)
            self.status_label.setText("Paused")
        else:
            self.pause_button.setText("Pause")
            self.step_button.setEnabled(False)
            self.status_label.setText("Running")
    
    def _step_frame(self):
        """Step one frame when paused."""
        if self.debug_interface.is_paused():
            self.debug_interface.step_frame()
            self.status_label.setText("Stepped frame")
    
    def _update_refresh_rate(self, value: int):
        """Update refresh rate."""
        self.refresh_rate = value
        self.update_timer.setInterval(1000 // value)
    
    def _toggle_auto_refresh(self, state: int):
        """Toggle auto-refresh."""
        if state == Qt.CheckState.Checked.value:
            self._start_updates()
        else:
            self._stop_updates()
    
    def _start_updates(self):
        """Start the update timer."""
        self.update_timer.start(1000 // self.refresh_rate)
    
    def _stop_updates(self):
        """Stop the update timer."""
        self.update_timer.stop()
    
    def _update_all(self):
        """Update all widgets."""
        try:
            # Update GameObject list
            self.gameobject_list.refresh()
            
            # Refresh inspector values for selected object
            if self.selected_game_object:
                self.component_inspector.refresh_values()
            
            # Update UI elements panel
            self.ui_elements.refresh()
            
            # Update engine stats
            self.engine_stats.refresh()
            
            # Update status
            is_paused = self.debug_interface.is_paused()
            if is_paused:
                self.status_label.setText("Paused")
            else:
                fps = self.debug_interface.get_fps()
                self.status_label.setText(f"Running - FPS: {fps:.1f}")
        except Exception as e:
            print(f"Error updating dev tool: {e}")
    
    def _on_gameobject_selected(self, selected_object):
        """Handle GameObject or UI element selection."""
        # print(f"[DevTool] _on_gameobject_selected called with: {type(selected_object).__name__}")
        # if selected_object:
        #     if type(selected_object).__name__ == 'GameObject' or isinstance(selected_object, GameObject):
        #         print(f"[DevTool] GameObject: {selected_object.name}")
        #     elif isinstance(selected_object, dict):
        #         print(f"[DevTool] UI Element/Canvas: {selected_object.get('type', 'Unknown')}")
        # else:
        #     print("[DevTool] Selection cleared (None)")
        
        # Store selection (could be GameObject, dict, or None)
        if type(selected_object).__name__ == 'GameObject' or isinstance(selected_object, GameObject):
            self.selected_game_object = selected_object
        else:
            # UI element (dict) or None
            self.selected_game_object = None
        
        # Pass to inspector
        self.component_inspector.set_game_object(selected_object)
    
    def closeEvent(self, event):
        """Handle window close."""
        self._stop_updates()
        event.accept()


def start_dev_tool(engine: Engine) -> Optional[DevToolWindow]:
    """
    Start the developer tool in a separate window.
    
    Args:
        engine: Engine instance to debug
        
    Returns:
        DevToolWindow instance (or None if PyQt not available)
    """
    global _qt_app, _qt_window
    try:
        # Check if QApplication already exists
        app = QApplication.instance()
        if app is None:
            app = QApplication(sys.argv)
            _qt_app = app
            # Don't quit when last window closes (we manage lifecycle)
            app.setQuitOnLastWindowClosed(False)
        
        # Create and show window
        window = DevToolWindow(engine)
        _qt_window = window  # Keep reference to prevent garbage collection
        window.show()
        window.raise_()  # Bring window to front
        window.activateWindow()  # Activate on macOS
        
        # Process Qt events to ensure window appears
        app.processEvents()
        
        # Add a runnable to the engine to process Qt events each frame
        def process_qt_events(engine):
            """Process Qt events in the main thread during engine update."""
            if _qt_app and _qt_window and _qt_window.isVisible():
                _qt_app.processEvents()
        
        # Add to engine's update runnables (only once)
        from pyg_engine.core.runnable import Priority
        # Check if we already added this runnable
        if not hasattr(engine, '_qt_events_processed'):
            engine.add_runnable(process_qt_events, event_type='update', priority=Priority.LOW)
            engine._qt_events_processed = True
        
        # Also process events in render phase to keep window responsive
        def process_qt_events_render(engine):
            """Process Qt events during render phase."""
            if _qt_app and _qt_window and _qt_window.isVisible():
                _qt_app.processEvents()
        
        if not hasattr(engine, '_qt_events_render_processed'):
            engine.add_runnable(process_qt_events_render, event_type='render', priority=Priority.LOW)
            engine._qt_events_render_processed = True
        
        # print("Developer tool window opened successfully")
        return window
    except ImportError:
        print("PyQt6 not available. Install with: pip install PyQt6")
        return None
    except Exception as e:
        import traceback
        print(f"Error starting dev tool: {e}")
        traceback.print_exc()
        return None


if __name__ == "__main__":
    # For testing
    print("Dev tool must be started from game code using start_dev_tool(engine)")

