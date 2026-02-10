use super::logging;
use crate::core::game_object::GameObject;
use std::collections::HashMap;

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

    fn bump_scene_version(&mut self) {
        self.scene_version = self.scene_version.wrapping_add(1);
    }

    /// Marks object-managed scene state as changed.
    pub fn mark_scene_dirty(&mut self) {
        self.bump_scene_version();
    }

    /// Returns the current object scene version.
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

    /**
        Removes an object by id.
        @param id: The id of the object to remove.
    */
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

    /**
        Gets an object by id.
        @param id: The id of the object to get.
        @return: The object.
    */
    pub fn get_object_by_id(&self, id: u32) -> Option<&GameObject> {
        self.objects.get(&id)
    }

    /**
        Gets a mutable object by id.
        @param id: The id of the object to get.
        @return: The mutable object.
    */
    pub fn get_object_by_id_mut(&mut self, id: u32) -> Option<&mut GameObject> {
        if self.objects.contains_key(&id) {
            // Mutable access may change visual state; conservatively bump.
            self.bump_scene_version();
        }
        self.objects.get_mut(&id)
    }

    /**
        Gets the total number of objects.
        @return: The total number of objects.
    */
    pub fn get_total_objects(&self) -> u32 {
        self.total_objects
    }

    /**
        Gets the number of active objects.
        @return: The number of active objects.
    */
    pub fn get_active_objects(&self) -> u32 {
        self.active_objects
    }

    /**
        Gets the objects.
        @return: The objects.
    */
    pub fn get_objects(&self) -> Vec<&GameObject> {
        self.objects.values().collect::<Vec<&GameObject>>()
    }

    /// Gets the keys of the objects.
    /// @return: The keys of the objects.
    pub fn get_keys(&self) -> &[u32] {
        &self.keys_insertion
    }

    /// Gets the sorted keys of the objects.
    /// @return: The sorted keys of the objects.
    pub fn get_sorted_keys(&self) -> &[u32] {
        &self.keys_sorted
    }
}
