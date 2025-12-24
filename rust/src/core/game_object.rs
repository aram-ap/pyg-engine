use std::sync::atomic::{AtomicU32, Ordering};
use super::component::ComponentTrait;
use super::time::Time;

// Keep track of the next game object id.
static GO_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjectType {
    GameObject,
    UIObject,
    ParticleSystem,
    Sound,
    Light,
    Camera
}

impl Default for ObjectType {
    fn default() -> Self {
        ObjectType::GameObject
    }
}

pub struct GameObject {
    id: u32,
    name: Option<String>,
    children: Vec<u32>,
    parent: Option<u32>,
    components: Vec<Box<dyn ComponentTrait>>,
    object_type: Option<ObjectType>,
    active: bool,
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
            components: Vec::new(),
            object_type: None,
            active: true,
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
            components: Vec::new(),
            object_type: None,
            active: true,
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
        Adds a child to the game object.
        @param child: The child to add.
    */
    pub fn add_child(&mut self, child: &GameObject) {
        let child_id = child.get_id();
        self.children.push(child_id);
    }

    /**
        Adds a component to the game object.
        @param component: The component to add.
    */
    pub fn add_component(&mut self, component: Box<dyn ComponentTrait>) {
        self.components.push(component);
    }

    /**
        Removes a child by id.
        @param id: The id of the child to remove.
        @return: The removed child id.
    */
    pub fn remove_child_by_id(&mut self, id: u32) -> Option<u32> {
        if let Some(index) = self.children.iter().position(|c| *c == id) {
            let child_id = self.children.swap_remove(index);
            // Note: Setting child.parent to None should be handled by the ObjectManager
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
        if let Some(index) = self.children.iter().position(|c| *c == id) {
            Some(self.children[index])
        } else {
            None
        }
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
        if let Some(index) = self.components.iter().position(|c| c.name() == name) {
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
        self.components
            .iter()
            .find(|c| c.name() == name)
            .map(|c| c.as_ref())
    }

    /**
        Checks if the game object is active.
        @return: True if the game object is active, false otherwise.
    */
    pub fn is_active(&self) -> bool {
        self.active
    }

    /**
        Sets the active state of the game object.
        @param active: The active state to set.
    */
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    /**
        Gets the object type of the game object.
        @return: The object type of the game object.
    */
    pub fn get_object_type(&self) -> ObjectType {
        self.object_type.as_ref().copied().unwrap_or(ObjectType::GameObject)
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
        for component in self.components.iter() {
            component.update(time);
        }
    }

    /**
        Updates the game object at a fixed time.
        @param time: The time.
        @param fixed_time: The fixed time.
    */
    pub fn fixed_update(&self, time: &Time, fixed_time: f32) {
        for component in self.components.iter() {
            component.fixed_update(time, fixed_time);
        }
    }
}