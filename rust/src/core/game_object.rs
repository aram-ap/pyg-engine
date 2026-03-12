use super::component::{ComponentTrait, MeshComponent, TransformComponent};
use super::time::Time;
use std::sync::atomic::{AtomicU32, Ordering};

// Keep track of the next game object id.
static GO_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjectType {
    GameObject,
    UIObject,
    ParticleSystem,
    Sound,
    Light,
    Camera,
}

impl Default for ObjectType {
    fn default() -> Self {
        ObjectType::GameObject
    }
}

#[derive(Clone, Debug)]
pub struct GameObject {
    id: u32,
    name: Option<String>,
    children: Vec<u32>,
    parent: Option<u32>,
    transform: TransformComponent,
    mesh: Option<MeshComponent>,
    components: Vec<Box<dyn ComponentTrait>>,
    object_type: Option<ObjectType>,
    enabled_self: bool,
    enabled_in_hierarchy: bool,
}

impl GameObject {
    /**
        Creates a new game object.
        @return: The new game object.
    */
    pub fn new() -> Self {
        let id = GO_ID.fetch_add(1, Ordering::SeqCst) + 1;
        Self {
            id,
            name: Some("GameObject".to_string()),
            children: Vec::new(),
            parent: None,
            transform: TransformComponent::new("Transform".to_string()),
            mesh: None,
            components: Vec::new(),
            object_type: None,
            enabled_self: true,
            enabled_in_hierarchy: true,
        }
    }

    /**
        Creates a new game object with a name.
        @param name: The name of the game object.
        @return: The new game object.
    */
    pub fn new_named(name: String) -> Self {
        let id = GO_ID.fetch_add(1, Ordering::SeqCst) + 1;
        Self {
            id,
            name: Some(name),
            children: Vec::new(),
            parent: None,
            transform: TransformComponent::new("Transform".to_string()),
            mesh: None,
            components: Vec::new(),
            object_type: None,
            enabled_self: true,
            enabled_in_hierarchy: true,
        }
    }

    /**
        Gets the id of the game object.
        @return: The id of the game object.
    */
    pub fn get_id(&self) -> u32 {
        self.id
    }

    /**
        Sets the name of the game object.
        @param name: The name to set.
    */
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /**
        Gets the name of the game object.
        @return: The object name, if set.
    */
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn parent_id(&self) -> Option<u32> {
        self.parent
    }

    pub fn set_parent_id(&mut self, parent: Option<u32>) {
        self.parent = parent;
    }

    pub fn children(&self) -> &[u32] {
        &self.children
    }

    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    pub fn add_child_id(&mut self, child_id: u32) {
        if !self.children.contains(&child_id) {
            self.children.push(child_id);
        }
    }

    /**
        Adds a component to the game object.
        @param component: The component to add.
    */
    pub fn add_component(&mut self, component: Box<dyn ComponentTrait>) {
        if component.as_any().is::<TransformComponent>() {
            let transform = component
                .into_any()
                .downcast::<TransformComponent>()
                .expect("transform downcast should succeed");
            self.transform = *transform;
            self.refresh_component_enabled_states();
            return;
        }

        if component.as_any().is::<MeshComponent>() {
            let mesh = component
                .into_any()
                .downcast::<MeshComponent>()
                .expect("mesh downcast should succeed");
            self.mesh = Some(*mesh);
            self.refresh_component_enabled_states();
            return;
        }

        self.components.push(component);
        self.refresh_component_enabled_states();
    }

    /// Gets the transform component for this game object.
    pub fn transform(&self) -> &TransformComponent {
        &self.transform
    }

    /// Gets a mutable transform component for this game object.
    pub fn transform_mut(&mut self) -> &mut TransformComponent {
        &mut self.transform
    }

    /// Replaces the transform component.
    pub fn set_transform(&mut self, transform: TransformComponent) {
        self.transform = transform;
        self.refresh_component_enabled_states();
    }

    /// Gets the position of the game object.
    pub fn position(&self) -> crate::types::vector::Vec2 {
        *self.transform.position()
    }

    /// Sets the position of the game object.
    pub fn set_position(&mut self, position: crate::types::vector::Vec2) {
        self.transform.set_position(position);
    }

    /// Gets the rotation of the game object.
    pub fn rotation(&self) -> f32 {
        self.transform.rotation()
    }

    /// Sets the rotation of the game object.
    pub fn set_rotation(&mut self, rotation: f32) {
        self.transform.set_rotation(rotation);
    }

    /// Gets the scale of the game object.
    pub fn scale(&self) -> crate::types::vector::Vec2 {
        *self.transform.scale()
    }

    /// Sets the scale of the game object.
    pub fn set_scale(&mut self, scale: crate::types::vector::Vec2) {
        self.transform.set_scale(scale);
    }

    /// Adds or replaces the mesh component.
    pub fn add_mesh_component(&mut self, mesh: MeshComponent) {
        self.mesh = Some(mesh);
        self.refresh_component_enabled_states();
    }

    /// Removes and returns the mesh component if present.
    pub fn remove_mesh_component(&mut self) -> Option<MeshComponent> {
        self.mesh.take()
    }

    /// Gets an immutable mesh component reference.
    pub fn mesh_component(&self) -> Option<&MeshComponent> {
        self.mesh.as_ref()
    }

    /// Gets a mutable mesh component reference.
    pub fn mesh_component_mut(&mut self) -> Option<&mut MeshComponent> {
        self.mesh.as_mut()
    }

    /// Returns true when this object has a mesh component.
    pub fn has_mesh_component(&self) -> bool {
        self.mesh.is_some()
    }

    /**
        Removes a child by id.
        @param id: The id of the child to remove.
        @return: The removed child id.
    */
    pub fn remove_child_by_id(&mut self, id: u32) -> Option<u32> {
        if let Some(index) = self.children.iter().position(|c| *c == id) {
            let child_id = self.children.swap_remove(index);
            Some(child_id)
        } else {
            None
        }
    }

    // /**
    //     Removes a child by name.
    //     @param name: The name of the child to remove.
    //     @return: The removed child.
    // */
    // pub fn remove_child_by_name(&mut self, name: &str) -> Option<GameObject> {
    //     if let Some(index) = self
    //         .children
    //         .iter()
    //         .position(|c| c.name.as_deref() == Some(name))
    //     {
    //         Some(self.children.remove(index))
    //     } else {
    //         None
    //     }
    // }

    /**
        Gets a child by id.
        @param id: The id of the child to get.
        @return: The child id.
    */
    pub fn get_child_by_id(&self, id: u32) -> Option<u32> {
        self.children.iter().find(|c| **c == id).copied()
    }

    // /**
    //     Gets a child by name.
    //     @param name: The name of the child to get.
    //     @return: The child.
    // */
    // pub fn get_child_by_name(&self, name: &str) -> Option<&GameObject> {
    //     self.children
    //         .iter()
    //         .find(|c| c.name.as_deref() == Some(name))
    // }

    /**
        Removes a component by name.
        @param name: The name of the component to remove.
        @return: The removed component.
    */
    pub fn remove_component_by_name(&mut self, name: &str) -> Option<Box<dyn ComponentTrait>> {
        if self.mesh_component().is_some_and(|mesh| mesh.name() == name) {
            return self
                .remove_mesh_component()
                .map(|component| Box::new(component) as Box<dyn ComponentTrait>);
        }

        if let Some(index) = self.components.iter().position(|c| c.name() == name) {
            Some(self.components.remove(index))
        } else {
            None
        }
    }

    pub fn remove_component_by_id(&mut self, component_id: u32) -> Option<Box<dyn ComponentTrait>> {
        if self.transform.id() == component_id {
            return None;
        }

        if self.mesh_component().is_some_and(|mesh| mesh.id() == component_id) {
            return self
                .remove_mesh_component()
                .map(|component| Box::new(component) as Box<dyn ComponentTrait>);
        }

        if let Some(index) = self.components.iter().position(|c| c.id() == component_id) {
            Some(self.components.remove(index))
        } else {
            None
        }
    }

    /**
        Gets a component by name.
        @param name: The name of the component to get.
        @return: The component.
    */
    pub fn get_component_by_name(&self, name: &str) -> Option<&dyn ComponentTrait> {
        if self.transform.name() == name {
            return Some(&self.transform);
        }

        if let Some(mesh) = &self.mesh
            && mesh.name() == name
        {
            return Some(mesh);
        }

        self.components
            .iter()
            .find(|c| c.name() == name)
            .map(|c| c.as_ref())
    }

    pub fn get_component_by_id(&self, component_id: u32) -> Option<&dyn ComponentTrait> {
        if self.transform.id() == component_id {
            return Some(&self.transform);
        }

        if let Some(mesh) = &self.mesh
            && mesh.id() == component_id
        {
            return Some(mesh);
        }

        self.components
            .iter()
            .find(|c| c.id() == component_id)
            .map(|c| c.as_ref())
    }

    /**
        Gets a mutable component by name.
        @param name: The name of the component to get.
        @return: The mutable component.
    */
    pub fn get_component_by_name_mut(&mut self, name: &str) -> Option<&mut (dyn ComponentTrait + '_)> {
        if self.transform.name() == name {
            return Some(&mut self.transform);
        }

        if let Some(mesh) = &mut self.mesh
            && mesh.name() == name
        {
            return Some(mesh);
        }

        for component in self.components.iter_mut() {
            if component.name() == name {
                return Some(component.as_mut());
            }
        }
        None
    }

    pub fn get_component_by_id_mut(
        &mut self,
        component_id: u32,
    ) -> Option<&mut (dyn ComponentTrait + '_)> {
        if self.transform.id() == component_id {
            return Some(&mut self.transform);
        }

        if let Some(mesh) = &mut self.mesh
            && mesh.id() == component_id
        {
            return Some(mesh);
        }

        for component in self.components.iter_mut() {
            if component.id() == component_id {
                return Some(component.as_mut());
            }
        }
        None
    }

    /**
        Gets a component by type using downcast.
        @return: The component if found and type matches.
    */
    pub fn get_component<T: ComponentTrait + 'static>(&self) -> Option<&T> {
        if let Some(concrete) = self.transform.as_any().downcast_ref::<T>() {
            return Some(concrete);
        }

        if let Some(mesh) = &self.mesh
            && let Some(concrete) = mesh.as_any().downcast_ref::<T>()
        {
            return Some(concrete);
        }

        for component in self.components.iter() {
            if let Some(concrete) = component.as_any().downcast_ref::<T>() {
                return Some(concrete);
            }
        }
        None
    }

    /**
        Gets a mutable component by type using downcast.
        @return: The mutable component if found and type matches.
    */
    pub fn get_component_mut<T: ComponentTrait + 'static>(&mut self) -> Option<&mut T> {
        if let Some(concrete) = self.transform.as_any_mut().downcast_mut::<T>() {
            return Some(concrete);
        }

        if let Some(mesh) = &mut self.mesh
            && let Some(concrete) = mesh.as_any_mut().downcast_mut::<T>()
        {
            return Some(concrete);
        }

        for component in self.components.iter_mut() {
            if let Some(concrete) = component.as_any_mut().downcast_mut::<T>() {
                return Some(concrete);
            }
        }
        None
    }

    /**
        Gets an iterator over all components.
        @return: An iterator over all components.
    */
    pub fn components_iter(&self) -> impl Iterator<Item = &Box<dyn ComponentTrait>> {
        self.components.iter()
    }

    pub fn all_components(&self) -> Vec<&dyn ComponentTrait> {
        let mut components: Vec<&dyn ComponentTrait> =
            Vec::with_capacity(self.components.len() + 2 + usize::from(self.mesh.is_some()));
        components.push(&self.transform);
        if let Some(mesh) = &self.mesh {
            components.push(mesh);
        }
        components.extend(self.components.iter().map(|component| component.as_ref()));
        components
    }

    pub fn get_components<T: ComponentTrait + 'static>(&self) -> Vec<&T> {
        let mut matches = Vec::new();

        if let Some(concrete) = self.transform.as_any().downcast_ref::<T>() {
            matches.push(concrete);
        }

        if let Some(mesh) = &self.mesh
            && let Some(concrete) = mesh.as_any().downcast_ref::<T>()
        {
            matches.push(concrete);
        }

        for component in &self.components {
            if let Some(concrete) = component.as_any().downcast_ref::<T>() {
                matches.push(concrete);
            }
        }

        matches
    }

    /**
        Checks if the game object is active.
        @return: True if the game object is active, false otherwise.
    */
    pub fn is_active(&self) -> bool {
        self.is_enabled()
    }

    /**
        Sets the active state of the game object.
        @param active: The active state to set.
    */
    pub fn set_active(&mut self, active: bool) {
        self.set_enabled_self(active);
    }

    pub fn enabled_self(&self) -> bool {
        self.enabled_self
    }

    pub fn enabled_in_hierarchy(&self) -> bool {
        self.enabled_in_hierarchy
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled_self && self.enabled_in_hierarchy
    }

    pub fn set_enabled_self(&mut self, enabled: bool) -> bool {
        let old_enabled = self.is_enabled();
        self.enabled_self = enabled;
        self.refresh_component_enabled_states();
        let new_enabled = self.is_enabled();
        self.dispatch_enabled_callbacks(old_enabled, new_enabled);
        old_enabled != new_enabled
    }

    pub fn set_enabled_in_hierarchy(&mut self, enabled: bool) -> bool {
        let old_enabled = self.is_enabled();
        self.enabled_in_hierarchy = enabled;
        self.refresh_component_enabled_states();
        let new_enabled = self.is_enabled();
        self.dispatch_enabled_callbacks(old_enabled, new_enabled);
        old_enabled != new_enabled
    }

    /**
        Gets the object type of the game object.
        @return: The object type of the game object.
    */
    pub fn get_object_type(&self) -> ObjectType {
        self.object_type
            .as_ref()
            .copied()
            .unwrap_or(ObjectType::GameObject)
    }

    /**
        Sets the object type of the game object.
        @param object_type: The object type to set.
    */
    pub fn set_object_type(&mut self, object_type: ObjectType) {
        self.object_type = Some(object_type);
    }

    /**
        Updates the game object.
    */
    pub fn update(&self, time: &Time) {
        if !self.is_enabled() {
            return;
        }

        if self.transform.is_effectively_enabled() {
            self.transform.update(time);
        }
        if let Some(mesh) = &self.mesh
            && mesh.is_effectively_enabled()
        {
            mesh.update(time);
        }
        for component in self.components.iter() {
            if component.is_effectively_enabled() {
                component.update(time);
            }
        }
    }

    /**
        Updates the game object at a fixed time.
        @param time: The time.
        @param fixed_time: The fixed time.
    */
    pub fn fixed_update(&self, time: &Time, fixed_time: f32) {
        if !self.is_enabled() {
            return;
        }

        if self.transform.is_effectively_enabled() {
            self.transform.fixed_update(time, fixed_time);
        }
        if let Some(mesh) = &self.mesh
            && mesh.is_effectively_enabled()
        {
            mesh.fixed_update(time, fixed_time);
        }
        for component in self.components.iter() {
            if component.is_effectively_enabled() {
                component.fixed_update(time, fixed_time);
            }
        }
    }

    pub fn invoke_on_destroy(&self) {
        for component in self.all_components() {
            component.on_destroy();
        }
    }

    fn refresh_component_enabled_states(&mut self) {
        let object_enabled = self.is_enabled();
        self.transform.set_enabled_in_hierarchy(object_enabled);
        if let Some(mesh) = &mut self.mesh {
            mesh.set_enabled_in_hierarchy(object_enabled);
        }
        for component in &mut self.components {
            component.set_enabled_in_hierarchy(object_enabled);
        }
    }

    fn dispatch_enabled_callbacks(&self, was_enabled: bool, is_enabled: bool) {
        if was_enabled == is_enabled {
            return;
        }

        for component in self.all_components() {
            if !component.is_enabled_self() {
                continue;
            }

            if is_enabled {
                component.on_enable();
            } else {
                component.on_disable();
            }
        }
    }
}
