"""
GameObject List Widget - Displays all GameObjects in a tree/list view.
"""

from PyQt6.QtWidgets import (
    QWidget, QVBoxLayout, QTreeWidget, QTreeWidgetItem, QLineEdit,
    QLabel, QHBoxLayout, QComboBox
)
from PyQt6.QtCore import pyqtSignal, Qt
from typing import Optional, List
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../..'))

from pyg_engine.core.gameobject import GameObject
from pyg_engine.utilities.object_types import Tag


class GameObjectListWidget(QWidget):
    """Widget that displays all GameObjects in a searchable list."""
    
    selection_changed = pyqtSignal(object)  # Emits selected GameObject
    
    def __init__(self, debug_interface):
        super().__init__()
        self.debug_interface = debug_interface
        self.selected_item = None
        self.selected_ui_element = None
        self.ui_elements_map = {}  # Map canvas_index/element_index to element data
        self._init_ui()
    
    def _init_ui(self):
        """Initialize the UI."""
        layout = QVBoxLayout(self)
        
        # Filter controls
        filter_layout = QHBoxLayout()
        
        # Name filter
        filter_layout.addWidget(QLabel("Filter:"))
        self.name_filter = QLineEdit()
        self.name_filter.setPlaceholderText("Filter by name...")
        self.name_filter.textChanged.connect(self._apply_filters)
        filter_layout.addWidget(self.name_filter)
        
        # Tag filter
        filter_layout.addWidget(QLabel("Tag:"))
        self.tag_filter = QComboBox()
        self.tag_filter.addItem("All")
        for tag in Tag:
            self.tag_filter.addItem(tag.value)
        self.tag_filter.currentTextChanged.connect(self._apply_filters)
        filter_layout.addWidget(self.tag_filter)
        
        filter_layout.addStretch()
        layout.addLayout(filter_layout)
        
        # Tree widget
        self.tree = QTreeWidget()
        self.tree.setHeaderLabels(["Name", "Type", "Enabled", "Info"])
        self.tree.itemSelectionChanged.connect(self._on_selection_changed)
        self.tree.itemDoubleClicked.connect(self._on_item_double_clicked)
        layout.addWidget(self.tree)
        
        # Status label
        self.status_label = QLabel("No GameObjects")
        layout.addWidget(self.status_label)
    
    def refresh(self):
        """Refresh the GameObject list."""
        try:
            game_objects = self.debug_interface.get_game_objects()
            ui_canvases = self.debug_interface.get_ui_elements_info()
            
            # Store current selection and scroll position
            selected_key = None
            current_item = self.tree.currentItem()
            if current_item:
                selected_key = current_item.data(0, Qt.ItemDataRole.UserRole)
            scroll_value = self.tree.verticalScrollBar().value()
            
            self.tree.setUpdatesEnabled(False)
            self.tree.blockSignals(True)
            
            # Clear tree and UI elements map
            self.tree.clear()
            self.ui_elements_map.clear()
            
            # Filter GameObjects
            filtered_objects = self._filter_objects(game_objects)
            
            # Add GameObjects
            item_to_select = None
            for obj in filtered_objects:
                item = self._create_tree_item(obj)
                self.tree.addTopLevelItem(item)
                
                # Restore selection reference using ID
                if selected_key and str(obj.id) == selected_key:
                    item_to_select = item
            
            # Add UI Canvases with elements
            for canvas_info in ui_canvases:
                canvas_item = self._create_canvas_item(canvas_info)
                self.tree.addTopLevelItem(canvas_item)
                
                # Check if canvas or any of its children should be selected
                canvas_key = f"canvas_{canvas_info['index']}"
                if selected_key == canvas_key:
                    item_to_select = canvas_item
                
                # Add UI elements as children
                for elem_idx, element_info in enumerate(canvas_info['elements']):
                    element_key = f"canvas_{canvas_info['index']}_elem_{elem_idx}"
                    elem_item = self._create_ui_element_item(element_info, canvas_info['index'], elem_idx)
                    canvas_item.addChild(elem_item)
                    
                    if selected_key == element_key:
                        item_to_select = elem_item
                
                # Expand canvas by default to show elements
                canvas_item.setExpanded(True)
            
            if item_to_select:
                self.tree.setCurrentItem(item_to_select)
                self.selected_item = item_to_select
            else:
                self.selected_item = None
            
            # Restore scroll position
            self.tree.verticalScrollBar().setValue(scroll_value)
            
            # Update status
            total_count = len(game_objects)
            filtered_count = len(filtered_objects)
            ui_element_count = sum(canvas['element_count'] for canvas in ui_canvases)
            
            status_parts = []
            if filtered_count < total_count:
                status_parts.append(f"{filtered_count} of {total_count} GameObjects")
            else:
                status_parts.append(f"{total_count} GameObjects")
            
            if ui_element_count > 0:
                status_parts.append(f"{ui_element_count} UI Elements")
            
            self.status_label.setText(", ".join(status_parts))
            
            # Resize columns
            self.tree.resizeColumnToContents(0)
            self.tree.resizeColumnToContents(1)
            self.tree.resizeColumnToContents(2)
            self.tree.resizeColumnToContents(3)
        except Exception as e:
            print(f"Error refreshing hierarchy: {e}")
            import traceback
            traceback.print_exc()
        finally:
            self.tree.blockSignals(False)
            self.tree.setUpdatesEnabled(True)
            
            # If we restored a selection, manually trigger the handler
            # because Qt won't emit itemSelectionChanged if it's the same item
            if item_to_select and selected_key:
                self._on_selection_changed()
    
    def _create_tree_item(self, obj: GameObject) -> QTreeWidgetItem:
        """Create a tree item for a GameObject."""
        item = QTreeWidgetItem()
        
        # Store GameObject ID as the key (not name, to handle duplicates)
        item.setData(0, Qt.ItemDataRole.UserRole, str(obj.id))
        item.setData(1, Qt.ItemDataRole.UserRole, "GameObject")  # Type marker
        
        # Name
        item.setText(0, obj.name)
        
        # Type (tag value)
        item.setText(1, obj.tag.value if hasattr(obj.tag, 'value') else str(obj.tag))
        
        # Enabled
        enabled_text = "✓" if obj.enabled else "✗"
        item.setText(2, enabled_text)
        
        # Info (components)
        component_names = [cls.__name__ for cls in obj.components.keys()]
        scripts_count = len(getattr(obj, 'scripts', []))
        info_parts = []
        if component_names:
            info_parts.append(", ".join(component_names))
        if scripts_count > 0:
            info_parts.append(f"{scripts_count} script(s)")
        item.setText(3, "; ".join(info_parts) if info_parts else "None")
        
        # Visual styling
        if not obj.enabled:
            for i in range(4):
                item.setForeground(i, Qt.GlobalColor.gray)
        
        return item
    
    def _filter_objects(self, objects: List[GameObject]) -> List[GameObject]:
        """Filter GameObjects based on current filters."""
        filtered = []
        
        name_filter = self.name_filter.text().lower()
        tag_filter = self.tag_filter.currentText()
        
        for obj in objects:
            # Name filter
            if name_filter and name_filter not in obj.name.lower():
                continue
            
            # Tag filter
            if tag_filter != "All":
                obj_tag_value = obj.tag.value if hasattr(obj.tag, 'value') else str(obj.tag)
                if obj_tag_value != tag_filter:
                    continue
            
            filtered.append(obj)
        
        return filtered
    
    def _apply_filters(self):
        """Apply filters and refresh."""
        self.refresh()
    
    def _create_canvas_item(self, canvas_info: dict) -> QTreeWidgetItem:
        """Create a tree item for a UI Canvas."""
        item = QTreeWidgetItem()
        
        canvas_key = f"canvas_{canvas_info['index']}"
        item.setData(0, Qt.ItemDataRole.UserRole, canvas_key)
        item.setData(1, Qt.ItemDataRole.UserRole, "UICanvas")  # Type marker
        
        # Name
        item.setText(0, canvas_info['name'])
        
        # Type
        item.setText(1, "UICanvas")
        
        # Enabled (always enabled for canvas)
        item.setText(2, "✓")
        
        # Info (element count)
        item.setText(3, f"{canvas_info['element_count']} elements")
        
        # Store canvas info in map (include ref for inspector)
        self.ui_elements_map[canvas_key] = {
            **canvas_info,
            'ref': canvas_info.get('canvas_ref'),
        }
        
        return item
    
    def _create_ui_element_item(self, element_info: dict, canvas_index: int, elem_idx: int) -> QTreeWidgetItem:
        """Create a tree item for a UI element."""
        item = QTreeWidgetItem()
        
        element_key = f"canvas_{canvas_index}_elem_{elem_idx}"
        item.setData(0, Qt.ItemDataRole.UserRole, element_key)
        item.setData(1, Qt.ItemDataRole.UserRole, "UIElement")  # Type marker
        
        # Name (use details or type)
        name = element_info.get('details') or element_info['type']
        item.setText(0, name)
        
        # Type
        item.setText(1, element_info['type'])
        
        # Enabled
        enabled = element_info.get('enabled', True) and element_info.get('visible', True)
        item.setText(2, "✓" if enabled else "✗")
        
        # Info (layer, anchor)
        info_parts = []
        if 'layer' in element_info:
            info_parts.append(f"Layer {element_info['layer']}")
        if 'anchor' in element_info:
            info_parts.append(element_info['anchor'])
        item.setText(3, ", ".join(info_parts))
        
        # Visual styling
        if not enabled:
            for i in range(4):
                item.setForeground(i, Qt.GlobalColor.gray)
        
        # Store element info in map (preserve ref for inspector)
        self.ui_elements_map[element_key] = element_info
        
        return item
    
    def _on_selection_changed(self):
        """Handle selection change."""
        items = self.tree.selectedItems()
        if items:
            self.selected_item = items[0]
            item_key = items[0].data(0, Qt.ItemDataRole.UserRole)
            item_type = items[0].data(1, Qt.ItemDataRole.UserRole)
            
            # print(f"[GameObjectList] Selection changed: key={item_key}, type={item_type}")
            
            if item_type in ("UICanvas", "UIElement"):
                # UI element selected - emit the UI element data
                ui_element_data = self.ui_elements_map.get(item_key)
                if ui_element_data:
                    # print(f"[GameObjectList] Emitting UI element: {ui_element_data.get('type', 'Unknown')}")
                    self.selected_ui_element = ui_element_data
                    self.selection_changed.emit(ui_element_data)
                else:
                    # print(f"[GameObjectList] Warning: UI element data not found for key={item_key}")
                    self.selection_changed.emit(None)
            else:
                # GameObject selected - item_key is the GameObject ID
                try:
                    obj_id = int(item_key)
                    obj = self.debug_interface.find_game_object_by_id(obj_id)
                    if obj:
                        # print(f"[GameObjectList] Emitting GameObject: {obj.name} (ID: {obj.id})")
                        self.selected_ui_element = None
                        self.selection_changed.emit(obj)
                    else:
                        # print(f"[GameObjectList] Warning: GameObject not found for ID={obj_id}")
                        self.selection_changed.emit(None)
                except (ValueError, AttributeError):
                    # Fallback to name-based lookup for backwards compatibility
                    obj = self.debug_interface.find_game_object_by_name(item_key)
                    if obj:
                        self.selected_ui_element = None
                        self.selection_changed.emit(obj)
                    else:
                        self.selection_changed.emit(None)
        else:
            # print("[GameObjectList] No selection")
            self.selected_item = None
            self.selected_ui_element = None
            self.selection_changed.emit(None)
    
    def _on_item_double_clicked(self, item, column):
        """Handle double-click (could expand to show more details)."""
        pass

