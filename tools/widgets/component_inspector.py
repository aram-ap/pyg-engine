"""
Component Inspector Widget - Allows editing component properties in real-time.
"""

from PyQt6.QtWidgets import (
    QWidget, QVBoxLayout, QTreeWidget, QTreeWidgetItem, QLabel,
    QLineEdit, QDoubleSpinBox, QSpinBox, QCheckBox, QPushButton,
    QColorDialog, QHBoxLayout, QGroupBox, QScrollArea, QFrame
)
from PyQt6.QtCore import Qt, pyqtSignal
from PyQt6.QtGui import QColor, QDoubleValidator
from typing import Optional, Any, Dict
import inspect

import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../..'))

from pyg_engine.core.gameobject import GameObject
from pyg_engine.components.component import Component
from pyg_engine.utilities.vector2 import Vector2
from pyg_engine.utilities.color import Color
from pyg_engine.utilities.object_types import Tag, BasicShape, Size


class PropertyEditor(QWidget):
    """Widget for editing a single property value."""
    
    value_changed = pyqtSignal(str, object)  # property_name, new_value
    
    def __init__(self, property_name: str, value: Any, parent=None, readonly=False):
        super().__init__(parent)
        self.property_name = property_name
        self.original_value = value
        self.readonly = readonly
        self._value_type = 'readonly'
        self._widgets: Dict[str, Any] = {}
        self._last_color = None  # Cache for color to avoid expensive setStyleSheet
        self._create_editor(value)
    
    def _create_editor(self, value: Any):
        """Create appropriate editor widget based on value type."""
        layout = QHBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        
        # Label
        label = QLabel(self.property_name + ":")
        label.setMinimumWidth(120)
        if self.readonly:
            label.setStyleSheet("color: gray;")
        layout.addWidget(label)
        
        # Create editor based on type
        # Check complex types first (before primitives) to avoid isinstance issues
        # Check Vector2 using type name to avoid import path issues
        if isinstance(value, Vector2) or type(value).__name__ == 'Vector2':
            # Vector2 editor with two editable text boxes
            validator = QDoubleValidator(-999999.0, 999999.0, 3, self)
            validator.setNotation(QDoubleValidator.Notation.StandardNotation)
            
            val_x = getattr(value, 'x', value[0] if hasattr(value, '__getitem__') else 0)
            val_y = getattr(value, 'y', value[1] if hasattr(value, '__getitem__') else 0)
            
            x_editor = QLineEdit(f"{val_x:.3f}")
            x_editor.setValidator(validator)
            x_editor.setPlaceholderText("X")
            x_editor.setReadOnly(self.readonly)
            if self.readonly:
                x_editor.setStyleSheet("QLineEdit { background-color: #f0f0f0; color: gray; }")
            if not self.readonly:
                x_editor.editingFinished.connect(
                    lambda xe=x_editor: self._on_vector2_text_changed('x', xe)
                )
            
            y_editor = QLineEdit(f"{val_y:.3f}")
            y_editor.setValidator(validator)
            y_editor.setPlaceholderText("Y")
            y_editor.setReadOnly(self.readonly)
            if self.readonly:
                y_editor.setStyleSheet("QLineEdit { background-color: #f0f0f0; color: gray; }")
            if not self.readonly:
                y_editor.editingFinished.connect(
                    lambda ye=y_editor: self._on_vector2_text_changed('y', ye)
                )
            
            layout.addWidget(x_editor)
            layout.addWidget(y_editor)
            self._vector2_value = Vector2(val_x, val_y)
            self._value_type = 'vector2'
            self._widgets['x'] = x_editor
            self._widgets['y'] = y_editor
        
        elif isinstance(value, Size) or type(value).__name__ == 'Size':
            # Size editor with two editable text boxes (w, h)
            val_w = getattr(value, 'w', 0)
            val_h = getattr(value, 'h', 0)
            
            w_editor = QSpinBox()
            w_editor.setRange(0, 999999)
            w_editor.setValue(val_w)
            w_editor.setPrefix("W: ")
            w_editor.setReadOnly(self.readonly)
            if self.readonly:
                w_editor.setStyleSheet("QSpinBox { background-color: #f0f0f0; color: gray; }")
            if not self.readonly:
                w_editor.valueChanged.connect(lambda v: self._on_size_changed('w', v))
            
            h_editor = QSpinBox()
            h_editor.setRange(0, 999999)
            h_editor.setValue(val_h)
            h_editor.setPrefix("H: ")
            h_editor.setReadOnly(self.readonly)
            if self.readonly:
                h_editor.setStyleSheet("QSpinBox { background-color: #f0f0f0; color: gray; }")
            if not self.readonly:
                h_editor.valueChanged.connect(lambda v: self._on_size_changed('h', v))
            
            layout.addWidget(w_editor)
            layout.addWidget(h_editor)
            self._size_value = Size(val_w, val_h)
            self._value_type = 'size'
            self._widgets['w'] = w_editor
            self._widgets['h'] = h_editor
        
        elif isinstance(value, Color) or type(value).__name__ == 'Color':
            # Color editor with preview box and text
            val_r = getattr(value, 'r', 0)
            val_g = getattr(value, 'g', 0)
            val_b = getattr(value, 'b', 0)
            val_a = getattr(value, 'a', 255)
            
            # Create color preview box
            color_preview = QFrame()
            color_preview.setFixedSize(30, 20)
            color_preview.setFrameShape(QFrame.Shape.Box)
            color_preview.setStyleSheet(f"background-color: rgba({val_r}, {val_g}, {val_b}, {val_a}); border: 1px solid black;")
            layout.addWidget(color_preview)
            
            # Create text input for color
            color_text = QLineEdit()
            color_text.setText(f"RGB({val_r}, {val_g}, {val_b}, {val_a})")
            color_text.setReadOnly(True)
            if self.readonly:
                color_text.setStyleSheet("QLineEdit { background-color: #f0f0f0; color: gray; }")
            layout.addWidget(color_text)
            
            # Create button to open color picker
            if not self.readonly:
                color_button = QPushButton("Edit")
                color_button.clicked.connect(lambda: self._edit_color(Color(val_r, val_g, val_b, val_a)))
                layout.addWidget(color_button)
            
            self._color_value = Color(val_r, val_g, val_b, val_a)
            self._value_type = 'color'
            self._widgets['preview'] = color_preview
            self._widgets['text'] = color_text
            if not self.readonly:
                self._widgets['button'] = color_button
        
        elif isinstance(value, bool):
            editor = QCheckBox()
            editor.setChecked(value)
            editor.setEnabled(not self.readonly)
            if self.readonly:
                editor.setStyleSheet("QCheckBox { color: gray; }")
            if not self.readonly:
                editor.stateChanged.connect(lambda state: self._on_value_changed(state == Qt.CheckState.Checked.value))
            layout.addWidget(editor)
            self._value_type = 'bool'
            self._widgets['main'] = editor
        
        elif isinstance(value, int):
            editor = QSpinBox()
            editor.setRange(-999999, 999999)
            editor.setValue(value)
            editor.setReadOnly(self.readonly)
            if self.readonly:
                editor.setStyleSheet("QSpinBox { background-color: #f0f0f0; color: gray; }")
            if not self.readonly:
                editor.valueChanged.connect(self._on_value_changed)
            layout.addWidget(editor)
            self._value_type = 'int'
            self._widgets['main'] = editor
        
        elif isinstance(value, float):
            editor = QDoubleSpinBox()
            editor.setRange(-999999.0, 999999.0)
            editor.setDecimals(3)
            editor.setValue(value)
            editor.setReadOnly(self.readonly)
            if self.readonly:
                editor.setStyleSheet("QDoubleSpinBox { background-color: #f0f0f0; color: gray; }")
            if not self.readonly:
                editor.valueChanged.connect(self._on_value_changed)
            layout.addWidget(editor)
            self._value_type = 'float'
            self._widgets['main'] = editor
        
        elif isinstance(value, str):
            editor = QLineEdit()
            editor.setText(value)
            editor.setReadOnly(self.readonly)
            if self.readonly:
                editor.setStyleSheet("QLineEdit { background-color: #f0f0f0; color: gray; }")
            if not self.readonly:
                editor.textChanged.connect(self._on_value_changed)
            layout.addWidget(editor)
            self._value_type = 'str'
            self._widgets['main'] = editor
        
        elif isinstance(value, (Tag, BasicShape)):
            # Enum editor (read-only display for now)
            editor = QLabel(str(value.value) if hasattr(value, 'value') else str(value))
            layout.addWidget(editor)
            self._value_type = 'enum'
            self._widgets['label'] = editor
        
        else:
            # Generic editor (string representation)
            editor = QLineEdit()
            editor.setText(str(value))
            editor.setReadOnly(True)  # Read-only for unknown types
            layout.addWidget(editor)
            self._value_type = 'readonly'
            self._widgets['line'] = editor
        
        # Removed addStretch() to prevent layout recalculation flickering
    
    def refresh_value(self, value: Any):
        """Update the displayed value without triggering change callbacks."""
        self.original_value = value
        try:
            if self._value_type == 'bool':
                widget = self._widgets.get('main')
                new_val = bool(value)
                if widget and not widget.hasFocus() and widget.isChecked() != new_val:
                    prev = widget.blockSignals(True)
                    widget.setChecked(new_val)
                    widget.blockSignals(prev)
            elif self._value_type == 'int':
                widget = self._widgets.get('main')
                new_val = int(value)
                if widget and not widget.hasFocus() and widget.value() != new_val:
                    prev = widget.blockSignals(True)
                    widget.setValue(new_val)
                    widget.blockSignals(prev)
            elif self._value_type == 'float':
                widget = self._widgets.get('main')
                new_val = float(value)
                if widget and not widget.hasFocus() and widget.value() != new_val:
                    prev = widget.blockSignals(True)
                    widget.setValue(new_val)
                    widget.blockSignals(prev)
            elif self._value_type == 'str':
                widget = self._widgets.get('main')
                if widget and not widget.hasFocus() and widget.text() != str(value):
                    prev = widget.blockSignals(True)
                    widget.setText(str(value))
                    widget.blockSignals(prev)
            elif self._value_type == 'vector2':
                x_widget = self._widgets.get('x')
                y_widget = self._widgets.get('y')
                if isinstance(value, Vector2):
                    vec = value
                elif isinstance(value, (tuple, list)) and len(value) >= 2:
                    vec = Vector2(value[0], value[1])
                else:
                    return
                new_x = f"{vec.x:.3f}"
                new_y = f"{vec.y:.3f}"
                if x_widget and not x_widget.hasFocus() and x_widget.text() != new_x:
                    prev = x_widget.blockSignals(True)
                    x_widget.setText(new_x)
                    x_widget.blockSignals(prev)
                if y_widget and not y_widget.hasFocus() and y_widget.text() != new_y:
                    prev = y_widget.blockSignals(True)
                    y_widget.setText(new_y)
                    y_widget.blockSignals(prev)
                self._vector2_value = Vector2(vec.x, vec.y)
            elif self._value_type == 'color':
                col = value if isinstance(value, Color) else Color(*value)
                color_tuple = (col.r, col.g, col.b, col.a)
                
                # Only update if color changed (expensive operation)
                if self._last_color != color_tuple:
                    # Update preview box
                    preview = self._widgets.get('preview')
                    if preview:
                        preview.setStyleSheet(f"background-color: rgba({col.r}, {col.g}, {col.b}, {col.a}); border: 1px solid black;")
                    # Update text
                    text = self._widgets.get('text')
                    if text:
                        text.setText(f"RGB({col.r}, {col.g}, {col.b}, {col.a})")
                    self._last_color = color_tuple
                self._color_value = Color(col.r, col.g, col.b, col.a)
            elif self._value_type == 'size':
                w_widget = self._widgets.get('w')
                h_widget = self._widgets.get('h')
                if isinstance(value, Size):
                    size_val = value
                else:
                    return
                if w_widget and not w_widget.hasFocus() and w_widget.value() != size_val.w:
                    prev = w_widget.blockSignals(True)
                    w_widget.setValue(size_val.w)
                    w_widget.blockSignals(prev)
                if h_widget and not h_widget.hasFocus() and h_widget.value() != size_val.h:
                    prev = h_widget.blockSignals(True)
                    h_widget.setValue(size_val.h)
                    h_widget.blockSignals(prev)
                self._size_value = Size(size_val.w, size_val.h)
            elif self._value_type == 'tuple':
                x_widget = self._widgets.get('0')
                y_widget = self._widgets.get('1')
                if isinstance(value, (tuple, list)) and len(value) >= 2:
                    tuple_val = value
                else:
                    return
                new_0 = f"{tuple_val[0]}"
                new_1 = f"{tuple_val[1]}"
                if x_widget and not x_widget.hasFocus() and x_widget.text() != new_0:
                    prev = x_widget.blockSignals(True)
                    x_widget.setText(new_0)
                    x_widget.blockSignals(prev)
                if y_widget and not y_widget.hasFocus() and y_widget.text() != new_1:
                    prev = y_widget.blockSignals(True)
                    y_widget.setText(new_1)
                    y_widget.blockSignals(prev)
                self._tuple_value = list(tuple_val)
            elif self._value_type == 'enum':
                widget = self._widgets.get('label')
                if widget:
                    text = str(value.value) if hasattr(value, 'value') else str(value)
                    widget.setText(text)
            elif self._value_type == 'readonly':
                widget = self._widgets.get('line')
                if widget:
                    widget.setText(str(value))
        except Exception:
            # Ignore refresh errors to prevent spam
            pass
    
    def _on_value_changed(self, value):
        """Handle value change."""
        self.value_changed.emit(self.property_name, value)
    
    def _parse_float(self, text: str, default: float) -> float:
        try:
            return float(text)
        except (TypeError, ValueError):
            return default
    
    def _on_vector2_text_changed(self, component: str, line_edit: QLineEdit):
        """Handle Vector2 text edits."""
        current = self._vector2_value.x if component == 'x' else self._vector2_value.y
        value = self._parse_float(line_edit.text(), current)
        self._on_vector2_changed(component, value)
    
    def _on_vector2_changed(self, component: str, value: float):
        """Handle Vector2 component change."""
        if component == 'x':
            self._vector2_value.x = value
        else:
            self._vector2_value.y = value
        self.value_changed.emit(self.property_name, self._vector2_value)
    
    def _on_size_changed(self, component: str, value: int):
        """Handle Size component change."""
        if component == 'w':
            self._size_value.w = value
        else:
            self._size_value.h = value
        self.value_changed.emit(self.property_name, self._size_value)
    
    def _on_tuple_text_changed(self, index: int, line_edit: QLineEdit):
        """Handle tuple text edits."""
        current = self._tuple_value[index]
        value = self._parse_float(line_edit.text(), current)
        self._tuple_value[index] = value
        self.value_changed.emit(self.property_name, tuple(self._tuple_value))
    
    def _edit_color(self, current_color: Color):
        """Open color picker dialog."""
        qcolor = QColor(current_color.r, current_color.g, current_color.b, current_color.a)
        color = QColorDialog.getColor(qcolor, self, "Choose Color")
        if color.isValid():
            new_color = Color(color.red(), color.green(), color.blue(), color.alpha())
            self._color_value = new_color
            self._last_color = (new_color.r, new_color.g, new_color.b, new_color.a)  # Update cache
            # Update preview and text
            preview = self._widgets.get('preview')
            if preview:
                preview.setStyleSheet(f"background-color: rgba({new_color.r}, {new_color.g}, {new_color.b}, {new_color.a}); border: 1px solid black;")
            text = self._widgets.get('text')
            if text:
                text.setText(f"RGB({new_color.r}, {new_color.g}, {new_color.b}, {new_color.a})")
            self.value_changed.emit(self.property_name, new_color)


class ComponentInspectorWidget(QWidget):
    """Widget for inspecting and editing component properties."""
    
    def __init__(self, debug_interface):
        super().__init__()
        self.debug_interface = debug_interface
        self.current_game_object: Optional[GameObject] = None
        self.current_ui_element: Optional[dict] = None
        self.property_editors: Dict[str, PropertyEditor] = {}
        self._init_ui()
    
    def _init_ui(self):
        """Initialize the UI."""
        layout = QVBoxLayout(self)
        
        # Scroll area for components
        scroll = QScrollArea()
        scroll.setWidgetResizable(True)
        scroll.setHorizontalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAsNeeded)
        scroll.setVerticalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAsNeeded)
        
        # Create scroll widget with fixed minimum width to prevent resizing
        scroll_widget = QWidget()
        scroll_widget.setMinimumWidth(400)
        scroll.setWidget(scroll_widget)
        
        self.scroll_widget = scroll_widget
        self.scroll_layout = QVBoxLayout(self.scroll_widget)
        self.scroll_layout.addStretch()
        layout.addWidget(scroll)
        
        # Status label
        self.status_label = QLabel("No object selected")
        layout.addWidget(self.status_label)
    
    def set_game_object(self, target: Optional[Any]):
        """Set the object to inspect (GameObject or UI element dict)."""
        try:
            # Check if it's a GameObject by checking the type name
            # (isinstance may fail due to import path issues)
            if target is None:
                self.current_game_object = None
                self.current_ui_element = None
            elif isinstance(target, dict):
                self.current_ui_element = target
                self.current_game_object = None
            elif type(target).__name__ == 'GameObject' or isinstance(target, GameObject):
                self.current_game_object = target
                self.current_ui_element = None
            else:
                self.current_game_object = None
                self.current_ui_element = None
            
            self._refresh()
        except Exception as e:
            print(f"Error setting object in inspector: {e}")
            import traceback
            traceback.print_exc()
    
    def _refresh(self):
        """Refresh the component inspector."""
        try:
            # Clear existing editors
            for editor in self.property_editors.values():
                editor.deleteLater()
            self.property_editors.clear()
            
            # Clear layout
            while self.scroll_layout.count() > 1:  # Keep stretch
                item = self.scroll_layout.takeAt(0)
                if item.widget():
                    item.widget().deleteLater()
            
            # Handle UI element inspection
            if self.current_ui_element:
                self._refresh_ui_element()
                return
            
            # Handle no selection
            if not self.current_game_object:
                self.status_label.setText("No object selected")
                return
            
            # GameObject inspection
            # GameObject properties
            go_group = QGroupBox("GameObject Properties")
            go_layout = QVBoxLayout()
            
            # Basic properties
            self._add_property_editor(go_layout, "name", self.current_game_object.name, readonly=True)
            self._add_property_editor(go_layout, "enabled", self.current_game_object.enabled)
            self._add_property_editor(go_layout, "position", self.current_game_object.position)
            self._add_property_editor(go_layout, "size", self.current_game_object.size)
            self._add_property_editor(go_layout, "rotation", self.current_game_object.rotation)
            self._add_property_editor(go_layout, "color", self.current_game_object.color)
            
            go_group.setLayout(go_layout)
            self.scroll_layout.insertWidget(0, go_group)
            
            # Component properties
            for component_class, component in self.current_game_object.components.items():
                component_group = QGroupBox(component_class.__name__)
                component_layout = QVBoxLayout()
                
                # Get all properties of the component
                for attr_name in dir(component):
                    if attr_name.startswith('_'):
                        continue
                    
                    try:
                        attr_value = getattr(component, attr_name)
                        if callable(attr_value):
                            continue
                        if attr_name == 'game_object':
                            continue
                        
                        self._add_property_editor(component_layout, attr_name, attr_value, component)
                    except Exception:
                        continue
                
                component_group.setLayout(component_layout)
                self.scroll_layout.insertWidget(self.scroll_layout.count() - 1, component_group)
            
            # Script properties
            scripts = getattr(self.current_game_object, 'scripts', [])
            if scripts:
                scripts_container = QGroupBox("Scripts")
                scripts_container_layout = QVBoxLayout()
                
                for script in scripts:
                    script_group = QGroupBox(script.__class__.__name__)
                    script_layout = QVBoxLayout()
                    
                    if hasattr(script, 'enabled'):
                        self._add_property_editor(
                            script_layout,
                            "enabled",
                            getattr(script, 'enabled', True),
                            script
                        )
                    
                    for attr_name in dir(script):
                        if attr_name.startswith('_'):
                            continue
                        if attr_name in ('game_object', 'config', 'enabled'):
                            continue
                        try:
                            attr_value = getattr(script, attr_name)
                        except Exception:
                            continue
                        if callable(attr_value):
                            continue
                        self._add_property_editor(script_layout, attr_name, attr_value, script)
                    
                    config = getattr(script, 'config', None)
                    if isinstance(config, dict) and config:
                        for key, value in config.items():
                            self._add_property_editor(
                                script_layout,
                                f"config[{key}]",
                                value,
                                None,
                                readonly=True
                            )
                    
                    script_group.setLayout(script_layout)
                    scripts_container_layout.addWidget(script_group)
                
                scripts_container.setLayout(scripts_container_layout)
                self.scroll_layout.insertWidget(self.scroll_layout.count() - 1, scripts_container)
            
            self.status_label.setText(f"Inspecting: {self.current_game_object.name}")
        except Exception as e:
            print(f"Error refreshing component inspector: {e}")
            import traceback
            traceback.print_exc()
            self.status_label.setText(f"Error: {e}")
    
    def _refresh_ui_element(self):
        """Refresh the inspector for a UI element or canvas (live where possible)."""
        try:
            ui_elem = self.current_ui_element or {}
            element_type = ui_elem.get('type', 'Unknown')
            ui_obj = ui_elem.get('ref')  # Actual UIElement instance when available
            canvas_obj = ui_elem.get('canvas_ref')
            
            # Determine if this is a canvas or element
            is_canvas = 'elements' in ui_elem and ui_elem.get('elements') is not None
            
            if is_canvas:
                # Canvas properties (read-only for now)
                canvas_group = QGroupBox("UI Canvas")
                canvas_layout = QVBoxLayout()
                
                name_val = ui_elem.get('name', getattr(canvas_obj, 'name', ''))
                self._add_property_editor(canvas_layout, "name", name_val, readonly=True)
                self._add_property_editor(canvas_layout, "element_count", ui_elem.get('element_count', 0), readonly=True)
                
                canvas_group.setLayout(canvas_layout)
                self.scroll_layout.insertWidget(0, canvas_group)
                
                self.status_label.setText(f"Inspecting: {name_val or 'Canvas'}")
            else:
                # UI Element properties – live editable when we have a ref
                elem_group = QGroupBox(f"{element_type} Properties")
                elem_layout = QVBoxLayout()
                
                # Basic info
                self._add_property_editor(elem_layout, "type", element_type, readonly=True)
                
                # Visible / enabled toggles (edit when we have the object)
                visible_val = ui_elem.get('visible', True)
                enabled_val = ui_elem.get('enabled', True)
                self._add_property_editor(
                    elem_layout, "visible", visible_val, ui_obj if ui_obj is not None else None,
                    readonly=(ui_obj is None)
                )
                self._add_property_editor(
                    elem_layout, "enabled", enabled_val, ui_obj if ui_obj is not None else None,
                    readonly=(ui_obj is None)
                )
                
                # Layer and anchor (layer editable, anchor read-only for now)
                self._add_property_editor(
                    elem_layout, "layer", ui_elem.get('layer', 0), ui_obj if ui_obj is not None else None,
                    readonly=(ui_obj is None)
                )
                self._add_property_editor(
                    elem_layout, "anchor", ui_elem.get('anchor', ''), readonly=True
                )
                
                # Offset and size as Vector2 (two boxes each) – editable when ref exists
                offset_val = ui_elem.get('offset')
                if offset_val is not None:
                    self._add_property_editor(
                        elem_layout, "offset", offset_val, ui_obj if ui_obj is not None else None,
                        readonly=(ui_obj is None)
                    )
                size_val = ui_elem.get('size')
                if size_val is not None:
                    self._add_property_editor(
                        elem_layout, "size", size_val, ui_obj if ui_obj is not None else None,
                        readonly=(ui_obj is None)
                    )
                
                # Text/content editing where supported
                if ui_obj is not None and hasattr(ui_obj, 'text'):
                    text_val = getattr(ui_obj, 'text', ui_elem.get('details', ''))
                    self._add_property_editor(
                        elem_layout, "text", text_val, ui_obj, readonly=False
                    )
                elif 'details' in ui_elem and ui_elem['details']:
                    # Fallback read-only details
                    self._add_property_editor(
                        elem_layout, "details", ui_elem['details'], readonly=True
                    )
                
                elem_group.setLayout(elem_layout)
                self.scroll_layout.insertWidget(0, elem_group)
                
                name = ui_elem.get('details') or element_type
                self.status_label.setText(f"Inspecting: {name}")
        except Exception as e:
            print(f"Error refreshing UI element inspector: {e}")
            import traceback
            traceback.print_exc()
            self.status_label.setText(f"Error: {e}")
    
    def _add_property_editor(self, layout, property_name: str, value: Any, target_object=None, readonly=False):
        """Add a property editor to the layout."""
        editor = PropertyEditor(property_name, value, readonly=readonly)
        if not readonly:
            editor.value_changed.connect(
                lambda name, val: self._on_property_changed(name, val, target_object)
            )
        layout.addWidget(editor)
        key = (target_object, property_name)
        self.property_editors[key] = editor
    
    def refresh_values(self):
        """Refresh displayed values without rebuilding UI."""
        # Handle UI element refresh
        if self.current_ui_element:
            ui_obj = self.current_ui_element.get('ref')
            if not ui_obj:
                return
            for (target_obj, property_name), editor in list(self.property_editors.items()):
                source = target_obj if target_obj is not None else ui_obj
                if not source:
                    continue
                try:
                    if hasattr(source, property_name):
                        value = getattr(source, property_name)
                        editor.refresh_value(value)
                except Exception:
                    continue
            return
        
        # Handle GameObject refresh
        if not self.current_game_object:
            return
        for (target_obj, property_name), editor in list(self.property_editors.items()):
            source = target_obj if target_obj is not None else self.current_game_object
            if not source:
                continue
            try:
                if hasattr(source, property_name):
                    value = getattr(source, property_name)
                    editor.refresh_value(value)
            except Exception:
                continue

    def _on_property_changed(self, property_name: str, new_value: Any, target_object: Optional[Component]):
        """Handle property value change."""
        try:
            # Handle UI element property changes
            if self.current_ui_element and target_object is not None:
                if hasattr(target_object, property_name):
                    # Special handling for text property
                    if property_name == "text" and hasattr(target_object, 'set_text'):
                        target_object.set_text(new_value)
                    # Special handling for color property
                    elif property_name == "color" and hasattr(target_object, 'set_color'):
                        target_object.set_color(new_value)
                    else:
                        setattr(target_object, property_name, new_value)
                return
            
            # Handle GameObject property changes
            if not self.current_game_object:
                return
            
            if target_object is None:
                # GameObject property
                if property_name == "enabled":
                    self.current_game_object.enabled = new_value
                elif property_name == "position":
                    if isinstance(new_value, Vector2):
                        self.current_game_object.update_position(new_value)
                elif property_name == "size":
                    if isinstance(new_value, Vector2):
                        self.current_game_object.update_size(new_value)
                elif property_name == "rotation":
                    self.current_game_object.update_rotation(new_value)
                elif property_name == "color":
                    if isinstance(new_value, Color):
                        self.current_game_object.update_color(new_value)
            else:
                # Component property
                if hasattr(target_object, property_name):
                    setattr(target_object, property_name, new_value)
        except Exception as e:
            print(f"Error setting property {property_name}: {e}")

