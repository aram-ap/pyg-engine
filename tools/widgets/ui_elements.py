"""
UI Elements Widget - Displays registered UI canvases and their elements.
"""

from PyQt6.QtWidgets import QWidget, QVBoxLayout, QTreeWidget, QTreeWidgetItem, QLabel
from PyQt6.QtCore import Qt


class UIElementsWidget(QWidget):
    """Widget that lists UI canvases and elements."""

    def __init__(self, debug_interface):
        super().__init__()
        self.debug_interface = debug_interface
        self._init_ui()

    def _init_ui(self):
        layout = QVBoxLayout(self)

        self.tree = QTreeWidget()
        self.tree.setHeaderLabels([
            "Name", "Type", "Visible", "Enabled", "Layer", "Anchor", "Offset", "Size", "Details"
        ])
        layout.addWidget(self.tree)

        self.status_label = QLabel("No UI canvases detected")
        layout.addWidget(self.status_label)

    def refresh(self):
        """Refresh list of UI canvases and elements."""
        try:
            canvases = self.debug_interface.get_ui_elements_info()
            selected = self.tree.currentItem().text(0) if self.tree.currentItem() else None

            self.tree.setUpdatesEnabled(False)
            self.tree.clear()

            for canvas in canvases:
                canvas_item = QTreeWidgetItem([
                    canvas['name'],
                    "Canvas",
                    "",
                    "",
                    "",
                    "",
                    "",
                    str(canvas['element_count']),
                    ""
                ])
                self.tree.addTopLevelItem(canvas_item)

                for element in canvas['elements']:
                    offset = element.get('offset')
                    size = element.get('size')
                    offset_text = f"({getattr(offset, 'x', 0):.1f}, {getattr(offset, 'y', 0):.1f})" if offset else "-"
                    size_text = f"({getattr(size, 'x', 0):.1f}, {getattr(size, 'y', 0):.1f})" if size else "-"

                    item = QTreeWidgetItem([
                        element['type'],
                        "Element",
                        "✓" if element['visible'] else "✗",
                        "✓" if element['enabled'] else "✗",
                        str(element['layer']),
                        element['anchor'] or "-",
                        offset_text,
                        size_text,
                        element.get('details') or ""
                    ])
                    canvas_item.addChild(item)

            self.tree.expandAll()
            if selected:
                items = self.tree.findItems(selected, Qt.MatchFlag.MatchRecursive, 0)
                if items:
                    self.tree.setCurrentItem(items[0])

            if canvases:
                self.status_label.setText(f"{len(canvases)} canvas(es), "
                                          f"{sum(c['element_count'] for c in canvases)} element(s)")
            else:
                self.status_label.setText("No UI canvases detected")
        finally:
            self.tree.setUpdatesEnabled(True)

