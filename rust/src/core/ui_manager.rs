use crate::core::component::ComponentTrait;
use crate::core::draw_manager::DrawManager;
use crate::core::time::Time;
use crate::core::game_object::{GameObject, ObjectType};
use crate::core::input_manager::InputManager;
use crate::core::object_manager::ObjectManager;
use crate::core::ui::button::ButtonComponent;
use crate::core::ui::event::{UIEvent, UIEventManager};
use crate::core::ui::label::LabelComponent;
use crate::core::ui::panel::PanelComponent;
use crate::core::ui::style::UITheme;
use crate::core::ui::{Rect, UIComponentTrait};
use std::any::Any;

#[derive(Clone, Copy)]
struct UIEntry {
    object_id: u32,
    depth: f64,
    bounds: Rect,
    render_offset: (f32, f32),
    enabled: bool,
}

#[derive(Clone, Copy, Debug)]
struct UIHitProxy {
    bounds: Rect,
    enabled: bool,
    depth: f32,
}

impl ComponentTrait for UIHitProxy {
    fn new(_name: String) -> Self
    where
        Self: Sized,
    {
        Self {
            bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
            enabled: true,
            depth: 0.0,
        }
    }

    fn name(&self) -> &str {
        "UIHitProxy"
    }

    fn id(&self) -> u32 {
        0
    }

    fn component_type(&self) -> &'static str {
        "UIHitProxy"
    }

    fn is_enabled_self(&self) -> bool {
        self.enabled
    }

    fn set_enabled_self(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn is_enabled_in_hierarchy(&self) -> bool {
        true
    }

    fn set_enabled_in_hierarchy(&mut self, _enabled: bool) {}

    fn update(&self, _time: &Time) {}
    fn fixed_update(&self, _time: &Time, _fixed_time: f32) {}
    fn on_start(&self) {}
    fn on_destroy(&self) {}
    fn on_enable(&self) {}
    fn on_disable(&self) {}

    fn clone_component(&self) -> Box<dyn ComponentTrait> {
        Box::new(*self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl UIComponentTrait for UIHitProxy {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn handle_event(&mut self, _event: &UIEvent) -> bool {
        false
    }

    fn render(&self, _draw_manager: &mut DrawManager, _offset: (f32, f32)) {}

    fn ui_depth(&self) -> f32 {
        self.depth
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

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

    pub fn update(&mut self, input: &InputManager, object_manager: &mut ObjectManager) {
        let entries = self.collect_ui_entries(object_manager);
        let proxies: Vec<UIHitProxy> = entries
            .iter()
            .map(|entry| UIHitProxy {
                bounds: entry.bounds,
                enabled: entry.enabled,
                depth: entry.depth as f32,
            })
            .collect();

        let ui_comp_refs: Vec<(u32, &dyn UIComponentTrait, f64)> = entries
            .iter()
            .zip(proxies.iter())
            .map(|(entry, proxy)| (entry.object_id, proxy as &dyn UIComponentTrait, entry.depth))
            .collect();

        let events = self.event_manager.process_input(input, &ui_comp_refs, self.scale_factor);
        for (target_id, event) in events {
            if let Some(obj) = object_manager.get_object_by_id_mut(target_id) {
                Self::dispatch_event(obj, &event);
            }
        }
    }

    pub fn render(&mut self, draw_manager: &mut DrawManager, object_manager: &ObjectManager) {
        let mut entries = self.collect_ui_entries(object_manager);
        if entries.is_empty() {
            return;
        }

        entries.sort_by(|a, b| a.depth.partial_cmp(&b.depth).unwrap_or(std::cmp::Ordering::Equal));
        if let Some(prev_start) = self.ui_cmd_start {
            draw_manager.truncate_from(prev_start);
        }

        let cmd_start = draw_manager.commands().len();
        self.ui_cmd_start = Some(cmd_start);

        for entry in entries {
            if !entry.enabled {
                continue;
            }

            if let Some(obj) = object_manager.get_object_by_id(entry.object_id) {
                Self::render_component(obj, draw_manager, entry.render_offset);
            }
        }

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

    fn collect_ui_entries(&self, object_manager: &ObjectManager) -> Vec<UIEntry> {
        let mut entries = Vec::new();
        let root_ids: Vec<u32> = object_manager
            .get_keys()
            .iter()
            .filter_map(|id| {
                let object = object_manager.get_object_by_id(*id)?;
                if object.get_object_type() != ObjectType::UIObject {
                    return None;
                }

                let is_root = object.parent_id().is_none_or(|parent_id| {
                    object_manager
                        .get_object_by_id(parent_id)
                        .map(|parent| parent.get_object_type() != ObjectType::UIObject)
                        .unwrap_or(true)
                });
                is_root.then_some(*id)
            })
            .collect();

        for root_id in root_ids {
            self.collect_ui_entries_recursive(object_manager, root_id, (0.0, 0.0), true, &mut entries);
        }

        entries
    }

    fn collect_ui_entries_recursive(
        &self,
        object_manager: &ObjectManager,
        object_id: u32,
        parent_offset: (f32, f32),
        inherited_enabled: bool,
        entries: &mut Vec<UIEntry>,
    ) {
        let Some(object) = object_manager.get_object_by_id(object_id) else {
            return;
        };
        let Some(component) = Self::ui_component(object) else {
            return;
        };

        let local_bounds = component.bounds();
        let render_offset = parent_offset;
        let absolute_bounds = Rect::new(
            local_bounds.x + parent_offset.0,
            local_bounds.y + parent_offset.1,
            local_bounds.width,
            local_bounds.height,
        );
        let enabled = inherited_enabled && object.is_enabled() && component.is_enabled();
        entries.push(UIEntry {
            object_id,
            depth: component.ui_depth() as f64,
            bounds: absolute_bounds,
            render_offset,
            enabled,
        });

        let child_offset = (absolute_bounds.x, absolute_bounds.y);
        for child_id in object.children() {
            self.collect_ui_entries_recursive(
                object_manager,
                *child_id,
                child_offset,
                enabled,
                entries,
            );
        }
    }

    fn ui_component(object: &GameObject) -> Option<&dyn UIComponentTrait> {
        if let Some(comp) = object.get_component_by_name("Button") {
            return comp
                .as_any()
                .downcast_ref::<ButtonComponent>()
                .map(|button| button as &dyn UIComponentTrait);
        }
        if let Some(comp) = object.get_component_by_name("Panel") {
            return comp
                .as_any()
                .downcast_ref::<PanelComponent>()
                .map(|panel| panel as &dyn UIComponentTrait);
        }
        if let Some(comp) = object.get_component_by_name("Label") {
            return comp
                .as_any()
                .downcast_ref::<LabelComponent>()
                .map(|label| label as &dyn UIComponentTrait);
        }
        None
    }

    fn render_component(object: &GameObject, draw_manager: &mut DrawManager, offset: (f32, f32)) {
        if let Some(comp) = object.get_component_by_name("Button")
            && let Some(button) = comp.as_any().downcast_ref::<ButtonComponent>()
        {
            button.render(draw_manager, offset);
            return;
        }
        if let Some(comp) = object.get_component_by_name("Panel")
            && let Some(panel) = comp.as_any().downcast_ref::<PanelComponent>()
        {
            panel.render(draw_manager, offset);
            return;
        }
        if let Some(comp) = object.get_component_by_name("Label")
            && let Some(label) = comp.as_any().downcast_ref::<LabelComponent>()
        {
            label.render(draw_manager, offset);
        }
    }

    fn dispatch_event(object: &mut GameObject, event: &UIEvent) {
        if let Some(comp) = object.get_component_by_name_mut("Button")
            && let Some(button) = comp.as_any_mut().downcast_mut::<ButtonComponent>()
        {
            button.handle_event(event);
            return;
        }
        if let Some(comp) = object.get_component_by_name_mut("Panel")
            && let Some(panel) = comp.as_any_mut().downcast_mut::<PanelComponent>()
        {
            panel.handle_event(event);
            return;
        }
        if let Some(comp) = object.get_component_by_name_mut("Label")
            && let Some(label) = comp.as_any_mut().downcast_mut::<LabelComponent>()
        {
            label.handle_event(event);
        }
    }
}
