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
use std::collections::HashMap;

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
    /// Create a new empty ObjectManager.
    ///
    /// Initializes with no objects and scene version 0.
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

    /// Add a GameObject to the manager and return its ID.
    ///
    /// Inserts the object into the registry, updates counters, and increments
    /// the scene version. If an object with the same ID already exists, it
    /// will be replaced.
    ///
    /// # Arguments
    /// * `object` - The GameObject to add
    ///
    /// # Returns
    /// The object's ID wrapped in `Some`, or `None` if the operation failed
    /// (currently always returns `Some`)
    ///
    /// # Examples
    /// ```rust
    /// use pyg_engine::ObjectManager;
    /// use pyg_engine::GameObject;
    ///
    /// let mut manager = ObjectManager::new();
    /// let obj = GameObject::new("Player");
    ///
    /// if let Some(id) = manager.add_object(obj) {
    ///     println!("Object added with ID: {}", id);
    /// }
    /// ```
    pub fn add_object(&mut self, object: GameObject) -> Option<u32> {
        let id = object.get_id();
        let is_active = object.is_active();

        // If replacing an existing object, update counters for the old object
        if let Some(old_object) = self.objects.insert(id, object) {
            // Object with this ID already existed, so we're replacing it
            self.active_objects -= if old_object.is_active() { 1 } else { 0 };
            // total_objects doesn't change since we're replacing, not adding
        } else {
            // New object, increment total_objects
            self.total_objects += 1;
            self.insert_key(id);
        }

        // Update active_objects counter for the new/replaced object
        self.active_objects += if is_active { 1 } else { 0 };
        self.bump_scene_version();
        Some(id)
    }

    /// Remove an object by its ID.
    ///
    /// Removes the object from the registry, updates counters, and increments
    /// the scene version. If the object doesn't exist, logs a warning.
    ///
    /// # Arguments
    /// * `id` - The unique ID of the object to remove
    ///
    /// # Examples
    /// ```rust
    /// use pyg_engine::ObjectManager;
    /// use pyg_engine::GameObject;
    ///
    /// let mut manager = ObjectManager::new();
    /// let obj = GameObject::new("Enemy");
    /// let id = manager.add_object(obj).unwrap();
    ///
    /// // Later, remove the object
    /// manager.remove_object(id);
    /// ```
    pub fn remove_object(&mut self, id: u32) {
        if self.total_objects == 0 {
            return;
        }

        // Remove the object from the hash map using the ID as the key
        // This is more efficient than doing get() then remove() - single hash lookup
        if let Some(removed_object) = self.objects.remove(&id) {
            let was_active = removed_object.is_active();
            self.total_objects -= 1;
            self.active_objects -= if was_active { 1 } else { 0 };
            self.remove_key(id);
            self.bump_scene_version();
        } else {
            logging::log_warn(&format!("Object {id} not found"));
        }
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
            // Mutable access may change visual state; conservatively bump.
            self.bump_scene_version();
        }
        self.objects.get_mut(&id)
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
}
