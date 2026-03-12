//! GameObject management and lifecycle tracking.
//!
//! The object manager maintains a registry of all game objects in the scene,
//! tracking their IDs, active states, and insertion order. It provides efficient
//! lookup, addition, and removal operations while maintaining scene version
//! tracking for change detection.
//!
//! # Scene Version Tracking
//!
//! The manager tracks a `scene_version` that increments whenever objects are
//! added, removed, or modified. This enables:
//! - Change detection for conditional rendering
//! - Optimization of frame updates (skip unchanged scenes)
//! - Synchronization with rendering pipeline
//!
//! # Object IDs
//!
//! Each GameObject has a unique ID assigned at creation. The manager uses this
//! ID as the key for all operations (lookup, removal, etc.).
//!
//! # Usage
//!
//! ```rust
//! use pyg_engine::ObjectManager;
//! use pyg_engine::GameObject;
//!
//! let mut manager = ObjectManager::new();
//!
//! // Add objects
//! let player = GameObject::new("Player");
//! let player_id = manager.add_object(player).unwrap();
//!
//! // Retrieve objects
//! if let Some(obj) = manager.get_object_by_id(player_id) {
//!     println!("Found: {}", obj.name());
//! }
//!
//! // Remove objects
//! manager.remove_object(player_id);
//!
//! // Check counts
//! println!("Total: {}, Active: {}",
//!     manager.get_total_objects(),
//!     manager.get_active_objects()
//! );
//! ```

use super::logging;
use crate::core::game_object::GameObject;
use crate::types::vector::Vec2;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub struct WorldTransform {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

/// Manages the lifecycle and storage of game objects.
///
/// `ObjectManager` maintains a registry of all GameObjects in the scene,
/// providing efficient lookup by ID, iteration in insertion order, and
/// scene change tracking via version numbers.
///
/// # Scene Version
///
/// The manager tracks a monotonically increasing version number that increments
/// whenever the scene changes (objects added, removed, or modified). This enables
/// efficient change detection for rendering optimizations.
///
/// # Object Ordering
///
/// Objects are tracked in both insertion order and sorted by ID:
/// - **Insertion order**: Maintained for predictable iteration
/// - **Sorted by ID**: Enables efficient binary search lookups
pub struct ObjectManager {
    objects: HashMap<u32, GameObject>, // id -> object
    total_objects: u32,
    active_objects: u32,
    keys_insertion: Vec<u32>,
    keys_sorted: Vec<u32>,
    scene_version: u64,
}

impl ObjectManager {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            total_objects: 0,
            active_objects: 0,
            keys_insertion: Vec::new(),
            keys_sorted: Vec::new(),
            scene_version: 0,
        }
    }

    /// Increment the scene version number.
    ///
    /// Internal method that wraps on overflow to prevent panics.
    fn bump_scene_version(&mut self) {
        self.scene_version = self.scene_version.wrapping_add(1);
    }

    /// Manually mark the scene as changed.
    ///
    /// Increments the scene version to indicate that the scene state has changed.
    /// Useful when modifying objects in ways that don't automatically trigger
    /// version updates.
    pub fn mark_scene_dirty(&mut self) {
        self.bump_scene_version();
    }

    /// Get the current scene version number.
    ///
    /// The scene version increments whenever objects are added, removed, or modified.
    /// Use this for change detection to optimize rendering (e.g., skip rendering
    /// if the scene hasn't changed since last frame).
    ///
    /// # Returns
    /// A monotonically increasing version number (wraps on overflow)
    pub fn scene_version(&self) -> u64 {
        self.scene_version
    }

    fn insert_key(&mut self, id: u32) {
        self.keys_insertion.push(id);
        match self.keys_sorted.binary_search(&id) {
            Ok(_) => {}
            Err(index) => self.keys_sorted.insert(index, id),
        }
    }

    fn remove_key(&mut self, id: u32) {
        if let Some(index) = self.keys_insertion.iter().position(|key| *key == id) {
            self.keys_insertion.remove(index);
        }
        if let Ok(index) = self.keys_sorted.binary_search(&id) {
            self.keys_sorted.remove(index);
        }
    }

    pub fn add_object(&mut self, object: GameObject) -> Option<u32> {
        let id = object.get_id();

        if self.objects.insert(id, object).is_none() {
            self.total_objects += 1;
            self.insert_key(id);
        } else {
        }

        self.refresh_enabled_counts();
        self.bump_scene_version();
        Some(id)
    }

    pub fn remove_object(&mut self, id: u32) {
        self.destroy_object_recursive(id);
    }

    /// Get an immutable reference to an object by its ID.
    ///
    /// # Arguments
    /// * `id` - The unique ID of the object to retrieve
    ///
    /// # Returns
    /// `Some(&GameObject)` if found, `None` otherwise
    ///
    /// # Examples
    /// ```rust
    /// use pyg_engine::ObjectManager;
    /// use pyg_engine::GameObject;
    ///
    /// let mut manager = ObjectManager::new();
    /// let obj = GameObject::new("Player");
    /// let id = manager.add_object(obj).unwrap();
    ///
    /// if let Some(player) = manager.get_object_by_id(id) {
    ///     println!("Found player: {}", player.name());
    /// }
    /// ```
    pub fn get_object_by_id(&self, id: u32) -> Option<&GameObject> {
        self.objects.get(&id)
    }

    /// Get a mutable reference to an object by its ID.
    ///
    /// **Note:** Increments the scene version conservatively, assuming the
    /// mutable access may change visual state.
    ///
    /// # Arguments
    /// * `id` - The unique ID of the object to retrieve
    ///
    /// # Returns
    /// `Some(&mut GameObject)` if found, `None` otherwise
    ///
    /// # Examples
    /// ```rust
    /// use pyg_engine::ObjectManager;
    /// use pyg_engine::GameObject;
    /// use pyg_engine::Vec2;
    ///
    /// let mut manager = ObjectManager::new();
    /// let obj = GameObject::new("Player");
    /// let id = manager.add_object(obj).unwrap();
    ///
    /// if let Some(player) = manager.get_object_by_id_mut(id) {
    ///     player.set_position(Vec2::new(100.0, 100.0));
    /// }
    /// ```
    pub fn get_object_by_id_mut(&mut self, id: u32) -> Option<&mut GameObject> {
        if self.objects.contains_key(&id) {
            self.bump_scene_version();
        }
        self.objects.get_mut(&id)
    }

    pub fn get_object_clone(&self, id: u32) -> Option<GameObject> {
        self.objects.get(&id).cloned()
    }

    pub fn get_object_ids_by_name(&self, name: &str) -> Vec<u32> {
        self.keys_insertion
            .iter()
            .filter_map(|id| {
                self.objects
                    .get(id)
                    .and_then(|object| (object.name() == Some(name)).then_some(*id))
            })
            .collect()
    }

    pub fn get_object_clones_by_name(&self, name: &str) -> Vec<GameObject> {
        self.get_object_ids_by_name(name)
            .into_iter()
            .filter_map(|id| self.get_object_clone(id))
            .collect()
    }

    pub fn world_transform(&self, id: u32) -> Option<WorldTransform> {
        let object = self.objects.get(&id)?;
        let local = WorldTransform {
            position: object.position(),
            rotation: object.rotation(),
            scale: object.scale(),
        };

        let parent_id = object.parent_id();
        let Some(parent_id) = parent_id else {
            return Some(local);
        };

        let parent = self.world_transform(parent_id)?;
        let scaled_local = local.position.multiply(&parent.scale);
        let sin_r = parent.rotation.sin();
        let cos_r = parent.rotation.cos();
        let rotated_local = Vec2::new(
            scaled_local.x() * cos_r - scaled_local.y() * sin_r,
            scaled_local.x() * sin_r + scaled_local.y() * cos_r,
        );

        Some(WorldTransform {
            position: parent.position.add(&rotated_local),
            rotation: parent.rotation + local.rotation,
            scale: parent.scale.multiply(&local.scale),
        })
    }

    pub fn world_position(&self, id: u32) -> Option<Vec2> {
        self.world_transform(id).map(|transform| transform.position)
    }

    /// Get the total number of objects in the manager.
    ///
    /// Returns the count of all objects, both active and inactive.
    ///
    /// # Returns
    /// Total object count
    pub fn get_total_objects(&self) -> u32 {
        self.total_objects
    }

    /// Get the number of active objects.
    ///
    /// Returns the count of objects where `is_active()` returns `true`.
    /// Active objects are typically updated and rendered.
    ///
    /// # Returns
    /// Active object count
    pub fn get_active_objects(&self) -> u32 {
        self.active_objects
    }

    /// Check if there are any UI objects in the scene
    pub fn has_ui_objects(&self) -> bool {
        use crate::core::game_object::ObjectType;
        self.objects.values().any(|obj| obj.get_object_type() == ObjectType::UIObject)
    }

    /// Get references to all objects in the manager.
    ///
    /// Returns a vector of immutable references to all GameObjects, regardless
    /// of their active state. The order is not guaranteed.
    ///
    /// # Returns
    /// Vector of immutable GameObject references
    ///
    /// # Examples
    /// ```rust
    /// use pyg_engine::ObjectManager;
    /// use pyg_engine::GameObject;
    ///
    /// let mut manager = ObjectManager::new();
    /// manager.add_object(GameObject::new("Player"));
    /// manager.add_object(GameObject::new("Enemy"));
    ///
    /// for obj in manager.get_objects() {
    ///     println!("Object: {}", obj.name());
    /// }
    /// ```
    pub fn get_objects(&self) -> Vec<&GameObject> {
        self.objects.values().collect::<Vec<&GameObject>>()
    }

    /// Get object IDs in insertion order.
    ///
    /// Returns a slice of object IDs in the order they were added to the manager.
    /// Useful for deterministic iteration order.
    ///
    /// # Returns
    /// Slice of object IDs in insertion order
    pub fn get_keys(&self) -> &[u32] {
        &self.keys_insertion
    }

    /// Get object IDs sorted numerically.
    ///
    /// Returns a slice of object IDs sorted in ascending order. This list is
    /// maintained via binary search for efficient lookups.
    ///
    /// # Returns
    /// Slice of object IDs in sorted order
    pub fn get_sorted_keys(&self) -> &[u32] {
        &self.keys_sorted
    }

    pub fn set_object_enabled(&mut self, id: u32, enabled: bool) -> bool {
        let changed = if let Some(object) = self.objects.get_mut(&id) {
            object.set_enabled_self(enabled)
        } else {
            return false;
        };

        self.refresh_enabled_from(id);
        self.refresh_enabled_counts();
        if changed {
            self.bump_scene_version();
        }
        true
    }

    pub fn add_child(&mut self, parent_id: u32, child_id: u32) -> Result<(), String> {
        if parent_id == child_id {
            return Err("Cannot parent an object to itself".to_string());
        }

        if !self.objects.contains_key(&parent_id) {
            return Err(format!("Parent object {parent_id} not found"));
        }
        if !self.objects.contains_key(&child_id) {
            return Err(format!("Child object {child_id} not found"));
        }
        if self.would_create_cycle(parent_id, child_id) {
            return Err("Cannot create a parent/child cycle".to_string());
        }

        let old_parent = self.objects.get(&child_id).and_then(GameObject::parent_id);
        if let Some(old_parent_id) = old_parent
            && let Some(old_parent_obj) = self.objects.get_mut(&old_parent_id)
        {
            old_parent_obj.remove_child_by_id(child_id);
        }

        if let Some(parent) = self.objects.get_mut(&parent_id) {
            parent.add_child_id(child_id);
        }
        if let Some(child) = self.objects.get_mut(&child_id) {
            child.set_parent_id(Some(parent_id));
        }

        self.refresh_enabled_from(child_id);
        self.refresh_enabled_counts();
        self.bump_scene_version();
        Ok(())
    }

    pub fn detach_child(&mut self, child_id: u32) -> bool {
        let parent_id = self.objects.get(&child_id).and_then(GameObject::parent_id);
        let Some(parent_id) = parent_id else {
            return false;
        };

        if let Some(parent) = self.objects.get_mut(&parent_id) {
            parent.remove_child_by_id(child_id);
        }
        if let Some(child) = self.objects.get_mut(&child_id) {
            child.set_parent_id(None);
        }
        self.refresh_enabled_from(child_id);
        self.refresh_enabled_counts();
        self.bump_scene_version();
        true
    }

    pub fn get_child_ids(&self, parent_id: u32) -> Vec<u32> {
        self.objects
            .get(&parent_id)
            .map(|object| object.children().to_vec())
            .unwrap_or_default()
    }

    pub fn get_child_by_id(&self, parent_id: u32, child_id: u32) -> Option<GameObject> {
        self.objects.get(&parent_id).and_then(|parent| {
            parent
                .get_child_by_id(child_id)
                .and_then(|id| self.get_object_clone(id))
        })
    }

    pub fn get_children_by_name(&self, parent_id: u32, name: &str) -> Vec<GameObject> {
        self.get_child_ids(parent_id)
            .into_iter()
            .filter_map(|id| self.get_object_by_id(id))
            .filter(|object| object.name() == Some(name))
            .cloned()
            .collect()
    }

    pub fn destroy_object_recursive(&mut self, id: u32) -> Vec<u32> {
        if !self.objects.contains_key(&id) {
            logging::log_warn(&format!("Object {id} not found"));
            return Vec::new();
        }

        let ids = self.collect_subtree_ids(id);
        for object_id in ids.iter().rev() {
            if let Some(object) = self.objects.get(object_id) {
                object.invoke_on_destroy();
            }
        }

        if let Some(parent_id) = self.objects.get(&id).and_then(GameObject::parent_id)
            && let Some(parent) = self.objects.get_mut(&parent_id)
        {
            parent.remove_child_by_id(id);
        }

        for object_id in ids.iter().rev() {
            if let Some(object) = self.objects.remove(object_id) {
                if let Some(parent_id) = object.parent_id()
                    && let Some(parent) = self.objects.get_mut(&parent_id)
                {
                    parent.remove_child_by_id(*object_id);
                }
                self.remove_key(*object_id);
                self.total_objects = self.total_objects.saturating_sub(1);
            }
        }

        self.refresh_enabled_counts();
        self.bump_scene_version();
        ids
    }

    fn collect_subtree_ids(&self, root_id: u32) -> Vec<u32> {
        let mut ids = Vec::new();
        self.collect_subtree_ids_recursive(root_id, &mut ids);
        ids
    }

    fn collect_subtree_ids_recursive(&self, current_id: u32, out: &mut Vec<u32>) {
        out.push(current_id);
        if let Some(object) = self.objects.get(&current_id) {
            for child_id in object.children() {
                self.collect_subtree_ids_recursive(*child_id, out);
            }
        }
    }

    fn would_create_cycle(&self, parent_id: u32, child_id: u32) -> bool {
        let mut current = Some(parent_id);
        while let Some(current_id) = current {
            if current_id == child_id {
                return true;
            }
            current = self.objects.get(&current_id).and_then(GameObject::parent_id);
        }
        false
    }

    fn refresh_enabled_from(&mut self, root_id: u32) {
        let parent_enabled = self
            .objects
            .get(&root_id)
            .and_then(GameObject::parent_id)
            .and_then(|parent_id| self.objects.get(&parent_id))
            .map(GameObject::is_enabled)
            .unwrap_or(true);
        self.refresh_enabled_recursive(root_id, parent_enabled);
    }

    fn refresh_enabled_recursive(&mut self, object_id: u32, parent_enabled: bool) {
        let Some(child_ids) = self.objects.get(&object_id).map(|object| object.children().to_vec()) else {
            return;
        };

        let current_enabled = if let Some(object) = self.objects.get_mut(&object_id) {
            object.set_enabled_in_hierarchy(parent_enabled);
            object.is_enabled()
        } else {
            return;
        };

        for child_id in child_ids {
            self.refresh_enabled_recursive(child_id, current_enabled);
        }
    }

    fn refresh_enabled_counts(&mut self) {
        self.active_objects = self
            .objects
            .values()
            .filter(|object| object.is_enabled())
            .count() as u32;
    }
}
