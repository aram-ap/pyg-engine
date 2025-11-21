"""
Engine Stats Widget - Displays engine performance metrics and statistics.
"""

from PyQt6.QtWidgets import QWidget, QVBoxLayout, QLabel, QGridLayout, QGroupBox
from PyQt6.QtCore import Qt
from collections import deque
from typing import Deque


class EngineStatsWidget(QWidget):
    """Widget that displays engine statistics and performance metrics."""
    
    def __init__(self, debug_interface):
        super().__init__()
        self.debug_interface = debug_interface
        
        # FPS history for averaging
        self.fps_history: Deque[float] = deque(maxlen=60)
        self.min_fps = float('inf')
        self.max_fps = 0.0
        
        self._init_ui()
    
    def _init_ui(self):
        """Initialize the UI."""
        layout = QVBoxLayout(self)
        
        # Performance metrics group
        perf_group = QGroupBox("Performance")
        perf_layout = QGridLayout()
        
        self.fps_label = QLabel("FPS: --")
        self.avg_fps_label = QLabel("Avg FPS: --")
        self.min_fps_label = QLabel("Min FPS: --")
        self.max_fps_label = QLabel("Max FPS: --")
        
        perf_layout.addWidget(self.fps_label, 0, 0)
        perf_layout.addWidget(self.avg_fps_label, 0, 1)
        perf_layout.addWidget(self.min_fps_label, 1, 0)
        perf_layout.addWidget(self.max_fps_label, 1, 1)
        
        perf_group.setLayout(perf_layout)
        layout.addWidget(perf_group)
        
        # Timing metrics group
        timing_group = QGroupBox("Timing")
        timing_layout = QGridLayout()
        
        self.delta_time_label = QLabel("Delta Time: --")
        self.unscaled_dt_label = QLabel("Unscaled DT: --")
        self.time_scale_label = QLabel("Time Scale: --")
        
        timing_layout.addWidget(self.delta_time_label, 0, 0)
        timing_layout.addWidget(self.unscaled_dt_label, 0, 1)
        timing_layout.addWidget(self.time_scale_label, 1, 0)
        
        timing_group.setLayout(timing_layout)
        layout.addWidget(timing_group)
        
        # Object counts group
        counts_group = QGroupBox("Object Counts")
        counts_layout = QGridLayout()
        
        self.gameobject_count_label = QLabel("GameObjects: --")
        self.component_count_label = QLabel("Components: --")
        
        counts_layout.addWidget(self.gameobject_count_label, 0, 0)
        counts_layout.addWidget(self.component_count_label, 0, 1)
        
        counts_group.setLayout(counts_layout)
        layout.addWidget(counts_group)
        
        # Engine state group
        state_group = QGroupBox("Engine State")
        state_layout = QGridLayout()
        
        self.running_label = QLabel("Running: --")
        self.paused_label = QLabel("Paused: --")
        self.physics_paused_label = QLabel("Physics Paused: --")
        self.fps_cap_label = QLabel("FPS Cap: --")
        
        state_layout.addWidget(self.running_label, 0, 0)
        state_layout.addWidget(self.paused_label, 0, 1)
        state_layout.addWidget(self.physics_paused_label, 1, 0)
        state_layout.addWidget(self.fps_cap_label, 1, 1)
        
        state_group.setLayout(state_layout)
        layout.addWidget(state_group)
        
        layout.addStretch()
    
    def refresh(self):
        """Refresh all statistics."""
        try:
            stats = self.debug_interface.get_engine_stats()
            
            # FPS metrics
            fps = stats['fps']
            self.fps_history.append(fps)
            
            if fps > 0:
                if fps < self.min_fps:
                    self.min_fps = fps
                if fps > self.max_fps:
                    self.max_fps = fps
                
                avg_fps = sum(self.fps_history) / len(self.fps_history) if self.fps_history else fps
                
                self.fps_label.setText(f"FPS: {fps:.1f}")
                self.avg_fps_label.setText(f"Avg FPS: {avg_fps:.1f}")
                self.min_fps_label.setText(f"Min FPS: {self.min_fps:.1f}")
                self.max_fps_label.setText(f"Max FPS: {self.max_fps:.1f}")
            else:
                self.fps_label.setText("FPS: --")
            
            # Timing metrics
            self.delta_time_label.setText(f"Delta Time: {stats['delta_time']:.4f}s")
            self.unscaled_dt_label.setText(f"Unscaled DT: {stats['unscaled_delta_time']:.4f}s")
            self.time_scale_label.setText(f"Time Scale: {stats['time_scale']:.2f}x")
            
            # Object counts
            self.gameobject_count_label.setText(f"GameObjects: {stats['game_object_count']}")
            self.component_count_label.setText(f"Components: {stats['component_count']}")
            
            # Engine state
            self.running_label.setText(f"Running: {'Yes' if stats['is_running'] else 'No'}")
            self.paused_label.setText(f"Paused: {'Yes' if stats['is_paused'] else 'No'}")
            self.physics_paused_label.setText(f"Physics Paused: {'Yes' if stats['physics_paused'] else 'No'}")
            self.fps_cap_label.setText(f"FPS Cap: {stats['fps_cap']}")
            
        except Exception as e:
            print(f"Error refreshing engine stats: {e}")

