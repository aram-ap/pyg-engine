use std::sync::atomic::{AtomicU32, Ordering};
use crate::component::ComponentTrait;

// Keep track of the next game object id.
static GO_ID: AtomicU32 = AtomicU32::new(0);

pub struct GameObject {
    id: u32,
    name: Option<String>,
    children: Vec<GameObject>,
    components: Vec<Box<dyn ComponentTrait>>,
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
            components: Vec::new(),
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
            components: Vec::new(),
        }
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
    pub fn add_child(&mut self, child: GameObject) {
        self.children.push(child);
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
        @return: The removed child.
    */
    pub fn remove_child_by_id(&mut self, id: u32) -> Option<GameObject> {
        if let Some(index) = self.children.iter().position(|c| c.id == id) {
            Some(self.children.remove(index))
        } else {
            None
        }
    }

    /**
        Removes a child by name.
        @param name: The name of the child to remove.
        @return: The removed child.
    */
    pub fn remove_child_by_name(&mut self, name: &str) -> Option<GameObject> {
        if let Some(index) = self
            .children
            .iter()
            .position(|c| c.name.as_deref() == Some(name))
        {
            Some(self.children.remove(index))
        } else {
            None
        }
    }

    /**
        Gets a child by id.
        @param id: The id of the child to get.
        @return: The child.
    */
    pub fn get_child_by_id(&self, id: u32) -> Option<&GameObject> {
        self.children.iter().find(|c| c.id == id)
    }

    /**
        Gets a child by name.
        @param name: The name of the child to get.
        @return: The child.
    */
    pub fn get_child_by_name(&self, name: &str) -> Option<&GameObject> {
        self.children
            .iter()
            .find(|c| c.name.as_deref() == Some(name))
    }

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
        Updates the game object.
    */
    pub fn update(&self) {

    }

}