use crate::core::ui::{Rect, UIComponentTrait};
use crate::core::ui::event::UIEventManager;
use crate::core::ui::style::UITheme;
use crate::core::ui::button::ButtonComponent;
use crate::core::ui::panel::PanelComponent;
use crate::core::ui::label::LabelComponent;
use crate::core::input_manager::InputManager;
use crate::core::object_manager::ObjectManager;
use crate::core::draw_manager::DrawManager;
use crate::core::game_object::ObjectType;

/// Manages the UI system
pub struct UIManager {
    /// Event manager for UI events
    event_manager: UIEventManager,
    /// UI theme
    theme: UITheme,
    /// Screen/root bounds
    root_bounds: Rect,
    /// HiDPI scale factor (logical to physical pixel ratio)
    scale_factor: f32,
    /// Start index of UI draw commands from the previous frame
    ui_cmd_start: Option<usize>,
}

impl UIManager {
    pub fn new(width: f32, height: f32, scale_factor: f32) -> Self {
        Self {
            event_manager: UIEventManager::new(),
            theme: UITheme::default_light(),
            root_bounds: Rect::new(0.0, 0.0, width, height),
            scale_factor,
            ui_cmd_start: None,
        }
    }

    /// Update UI system - process events and dispatch to components
    pub fn update(&mut self, input: &InputManager, object_manager: &mut ObjectManager) {
        crate::core::logging::log_debug("UIManager::update called");

        // Collect UI components with their IDs and depths
        let mut ui_components: Vec<(u32, f64)> = Vec::new();

        for obj in object_manager.get_objects() {
            if obj.get_object_type() == ObjectType::UIObject {
                // Get the UI depth from the first UI component found
                let mut depth = 0.0;

                // Try to get depth from button
                if let Some(comp) = obj.get_component_by_name("Button") {
                    if let Some(btn) = comp.as_any().downcast_ref::<ButtonComponent>() {
                        depth = btn.ui_depth();
                    }
                }

                // Try panel
                if depth == 0.0 {
                    if let Some(comp) = obj.get_component_by_name("Panel") {
                        if let Some(panel) = comp.as_any().downcast_ref::<PanelComponent>() {
                            depth = panel.ui_depth();
                        }
                    }
                }

                // Try label
                if depth == 0.0 {
                    if let Some(comp) = obj.get_component_by_name("Label") {
                        if let Some(label) = comp.as_any().downcast_ref::<LabelComponent>() {
                            depth = label.ui_depth();
                        }
                    }
                }

                ui_components.push((obj.get_id(), depth as f64));
            }
        }

        crate::core::logging::log_debug(&format!("UIManager::update: Found {} UI objects total", ui_components.len()));

        // Build component list for event manager
        let ui_comp_refs: Vec<(u32, &dyn UIComponentTrait, f64)> = ui_components
            .iter()
            .filter_map(|&(id, depth)| {
                object_manager.get_object_by_id(id).and_then(|obj| {
                    // Try to find a UI component
                    if let Some(comp) = obj.get_component_by_name("Button") {
                        comp.as_any().downcast_ref::<ButtonComponent>()
                            .map(|btn| (id, btn as &dyn UIComponentTrait, depth))
                    } else if let Some(comp) = obj.get_component_by_name("Panel") {
                        comp.as_any().downcast_ref::<PanelComponent>()
                            .map(|panel| (id, panel as &dyn UIComponentTrait, depth))
                    } else if let Some(comp) = obj.get_component_by_name("Label") {
                        comp.as_any().downcast_ref::<LabelComponent>()
                            .map(|label| (id, label as &dyn UIComponentTrait, depth))
                    } else {
                        None
                    }
                })
            })
            .collect();

        crate::core::logging::log_debug(&format!("UIManager: Found {} UI components to process", ui_comp_refs.len()));

        // Process input and generate events
        // Pass scale_factor to convert mouse position from physical to logical pixels
        let events = self.event_manager.process_input(input, &ui_comp_refs, self.scale_factor);

        crate::core::logging::log_debug(&format!("UIManager: Generated {} events", events.len()));

        // Dispatch events to components
        for (target_id, event) in events {
            if let Some(obj) = object_manager.get_object_by_id_mut(target_id) {
                // Try each component type
                if let Some(comp) = obj.get_component_by_name_mut("Button") {
                    if let Some(btn) = comp.as_any_mut().downcast_mut::<ButtonComponent>() {
                        btn.handle_event(&event);
                    }
                } else if let Some(comp) = obj.get_component_by_name_mut("Panel") {
                    if let Some(panel) = comp.as_any_mut().downcast_mut::<PanelComponent>() {
                        panel.handle_event(&event);
                    }
                } else if let Some(comp) = obj.get_component_by_name_mut("Label") {
                    if let Some(label) = comp.as_any_mut().downcast_mut::<LabelComponent>() {
                        label.handle_event(&event);
                    }
                }
            }
        }
    }

    /// Render all UI components
    pub fn render(&mut self, draw_manager: &mut DrawManager, object_manager: &ObjectManager) {
        crate::core::logging::log_debug("UIManager::render called");

        // Collect UI objects with their depths
        let mut ui_objects: Vec<(u32, f64)> = Vec::new();

        for obj in object_manager.get_objects() {
            if obj.get_object_type() == ObjectType::UIObject {
                let mut depth = 0.0;

                // Get depth from the component
                if let Some(comp) = obj.get_component_by_name("Button") {
                    if let Some(btn) = comp.as_any().downcast_ref::<ButtonComponent>() {
                        depth = btn.ui_depth() as f64;
                    }
                } else if let Some(comp) = obj.get_component_by_name("Panel") {
                    if let Some(panel) = comp.as_any().downcast_ref::<PanelComponent>() {
                        depth = panel.ui_depth() as f64;
                    }
                } else if let Some(comp) = obj.get_component_by_name("Label") {
                    if let Some(label) = comp.as_any().downcast_ref::<LabelComponent>() {
                        depth = label.ui_depth() as f64;
                    }
                }

                ui_objects.push((obj.get_id(), depth));
            }
        }

        crate::core::logging::log_debug(&format!("UIManager::render: Found {} UI objects to render", ui_objects.len()));

        // Only track and truncate UI commands if there are actually UI objects to render
        if ui_objects.is_empty() {
            // No UI objects - don't set ui_cmd_start or truncate anything
            // This preserves non-UI draw commands that should persist across frames
            return;
        }

        // Sort by depth (ascending, back to front)
        ui_objects.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        // Clear previous frame's UI draw commands to avoid accumulation
        if let Some(prev_start) = self.ui_cmd_start {
            draw_manager.truncate_from(prev_start);
        }

        // Record command count before UI rendering
        let cmd_start = draw_manager.commands().len();
        self.ui_cmd_start = Some(cmd_start);

        // Render each UI component (generates draw commands in logical pixels)
        for (id, _depth) in ui_objects {
            if let Some(obj) = object_manager.get_object_by_id(id) {
                if let Some(comp) = obj.get_component_by_name("Button") {
                    if let Some(btn) = comp.as_any().downcast_ref::<ButtonComponent>() {
                        btn.render(draw_manager, (0.0, 0.0));
                    }
                } else if let Some(comp) = obj.get_component_by_name("Panel") {
                    if let Some(panel) = comp.as_any().downcast_ref::<PanelComponent>() {
                        panel.render(draw_manager, (0.0, 0.0));
                    }
                } else if let Some(comp) = obj.get_component_by_name("Label") {
                    if let Some(label) = comp.as_any().downcast_ref::<LabelComponent>() {
                        label.render(draw_manager, (0.0, 0.0));
                    }
                }
            }
        }

        // Scale UI draw commands from logical to physical pixels
        if self.scale_factor != 1.0 {
            draw_manager.scale_commands_from(cmd_start, self.scale_factor);
        }
    }

    /// Check if input was consumed by UI this frame
    pub fn is_input_consumed(&self) -> bool {
        self.event_manager.is_input_consumed()
    }

    /// Update screen size
    pub fn resize(&mut self, width: f32, height: f32) {
        self.root_bounds = Rect::new(0.0, 0.0, width, height);
    }

    /// Get the current theme
    pub fn theme(&self) -> &UITheme {
        &self.theme
    }

    /// Get mutable theme
    pub fn theme_mut(&mut self) -> &mut UITheme {
        &mut self.theme
    }

    /// Reset UI command tracking when draw commands are cleared
    /// This should be called when clear_draw_commands() is invoked
    pub fn reset_command_tracking(&mut self) {
        self.ui_cmd_start = None;
    }
}
