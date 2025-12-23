use std::sync::atomic::{AtomicU32, Ordering};

// Keep track of the next game object id.
static GO_ID: AtomicU32 = AtomicU32::new(0);

struct GameObject {
    id: u32,
    name: Option<String>,
    children: Vec<GameObject>,
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
        Updates the game object.
    */
    pub fn update(&self) {

    }

}