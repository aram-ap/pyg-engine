use crate::core::game_object::GameObject;
use super::logging;
use std::collections::HashMap;

pub struct ObjectManager {
    objects: HashMap<u32, GameObject>, // id -> object
    total_objects: u32,
    active_objects: u32,
}

impl ObjectManager {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            total_objects: 0,
            active_objects: 0,
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
        }
        
        // Update active_objects counter for the new/replaced object
        self.active_objects += if is_active { 1 } else { 0 };
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
    pub fn get_keys(&self) -> Vec<u32> {
        self.objects.keys().cloned().collect()
    }
}

