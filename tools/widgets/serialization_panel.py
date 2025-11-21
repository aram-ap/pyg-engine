"""
Serialization Panel Widget - Export/import game state to/from JSON.
"""

from PyQt6.QtWidgets import (
    QWidget, QVBoxLayout, QHBoxLayout, QPushButton, QTextEdit,
    QLabel, QFileDialog, QMessageBox, QSplitter
)
from PyQt6.QtCore import Qt
import json

import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../..'))

from pyg_engine.core.engine import Engine
from pyg_engine.core.serialization import (
    serialize_scene, deserialize_scene, to_json, from_json
)


class SerializationPanelWidget(QWidget):
    """Widget for serializing and deserializing game state."""
    
    def __init__(self, engine: Engine, debug_interface):
        super().__init__()
        self.engine = engine
        self.debug_interface = debug_interface
        self._init_ui()
    
    def _init_ui(self):
        """Initialize the UI."""
        layout = QVBoxLayout(self)
        
        # Button panel
        button_layout = QHBoxLayout()
        
        # Export buttons
        export_label = QLabel("Export:")
        button_layout.addWidget(export_label)
        
        self.export_button = QPushButton("Export to JSON")
        self.export_button.clicked.connect(self._export_to_json)
        button_layout.addWidget(self.export_button)
        
        self.export_file_button = QPushButton("Export to File...")
        self.export_file_button.clicked.connect(self._export_to_file)
        button_layout.addWidget(self.export_file_button)
        
        button_layout.addSpacing(20)
        
        # Import buttons
        import_label = QLabel("Import:")
        button_layout.addWidget(import_label)
        
        self.import_button = QPushButton("Import from JSON")
        self.import_button.clicked.connect(self._import_from_json)
        button_layout.addWidget(self.import_button)
        
        self.import_file_button = QPushButton("Import from File...")
        self.import_file_button.clicked.connect(self._import_from_file)
        button_layout.addWidget(self.import_file_button)
        
        button_layout.addStretch()
        layout.addLayout(button_layout)
        
        # Text editor for JSON
        splitter = QSplitter(Qt.Orientation.Vertical)
        
        # Export text area
        export_group = QWidget()
        export_layout = QVBoxLayout(export_group)
        export_layout.addWidget(QLabel("Exported JSON:"))
        self.export_text = QTextEdit()
        self.export_text.setReadOnly(True)
        self.export_text.setFontFamily("Courier")
        export_layout.addWidget(self.export_text)
        splitter.addWidget(export_group)
        
        # Import text area
        import_group = QWidget()
        import_layout = QVBoxLayout(import_group)
        import_layout.addWidget(QLabel("JSON to Import (paste here):"))
        self.import_text = QTextEdit()
        self.import_text.setFontFamily("Courier")
        import_layout.addWidget(self.import_text)
        splitter.addWidget(import_group)
        
        splitter.setSizes([300, 300])
        layout.addWidget(splitter)
        
        # Status label
        self.status_label = QLabel("Ready")
        layout.addWidget(self.status_label)
    
    def _export_to_json(self):
        """Export current scene to JSON and display in text area."""
        try:
            scene_data = serialize_scene(self.engine)
            json_str = to_json(scene_data, indent=2)
            self.export_text.setPlainText(json_str)
            self.status_label.setText("Scene exported to JSON")
        except Exception as e:
            QMessageBox.critical(self, "Export Error", f"Failed to export scene:\n{e}")
            self.status_label.setText(f"Export error: {e}")
    
    def _export_to_file(self):
        """Export current scene to a JSON file."""
        try:
            filename, _ = QFileDialog.getSaveFileName(
                self, "Export Scene", "", "JSON Files (*.json);;All Files (*)"
            )
            
            if filename:
                scene_data = serialize_scene(self.engine)
                json_str = to_json(scene_data, indent=2)
                
                with open(filename, 'w') as f:
                    f.write(json_str)
                
                # Also update text area
                self.export_text.setPlainText(json_str)
                self.status_label.setText(f"Scene exported to {filename}")
        except Exception as e:
            QMessageBox.critical(self, "Export Error", f"Failed to export scene:\n{e}")
            self.status_label.setText(f"Export error: {e}")
    
    def _import_from_json(self):
        """Import scene from JSON in text area."""
        try:
            json_str = self.import_text.toPlainText()
            if not json_str.strip():
                QMessageBox.warning(self, "Import Error", "No JSON data to import")
                return
            
            scene_data = from_json(json_str)
            
            # Confirm before importing
            reply = QMessageBox.question(
                self, "Import Scene",
                "This will add GameObjects to the current scene. Continue?",
                QMessageBox.StandardButton.Yes | QMessageBox.StandardButton.No
            )
            
            if reply == QMessageBox.StandardButton.Yes:
                deserialize_scene(scene_data, self.engine)
                self.status_label.setText("Scene imported successfully")
                
                # Clear import text
                self.import_text.clear()
        except json.JSONDecodeError as e:
            QMessageBox.critical(self, "Import Error", f"Invalid JSON:\n{e}")
            self.status_label.setText(f"JSON error: {e}")
        except Exception as e:
            QMessageBox.critical(self, "Import Error", f"Failed to import scene:\n{e}")
            self.status_label.setText(f"Import error: {e}")
    
    def _import_from_file(self):
        """Import scene from a JSON file."""
        try:
            filename, _ = QFileDialog.getOpenFileName(
                self, "Import Scene", "", "JSON Files (*.json);;All Files (*)"
            )
            
            if filename:
                with open(filename, 'r') as f:
                    json_str = f.read()
                
                # Update text area
                self.import_text.setPlainText(json_str)
                
                # Import
                scene_data = from_json(json_str)
                
                # Confirm before importing
                reply = QMessageBox.question(
                    self, "Import Scene",
                    "This will add GameObjects to the current scene. Continue?",
                    QMessageBox.StandardButton.Yes | QMessageBox.StandardButton.No
                )
                
                if reply == QMessageBox.StandardButton.Yes:
                    deserialize_scene(scene_data, self.engine)
                    self.status_label.setText(f"Scene imported from {filename}")
        except FileNotFoundError:
            QMessageBox.warning(self, "Import Error", "File not found")
        except json.JSONDecodeError as e:
            QMessageBox.critical(self, "Import Error", f"Invalid JSON:\n{e}")
        except Exception as e:
            QMessageBox.critical(self, "Import Error", f"Failed to import scene:\n{e}")

