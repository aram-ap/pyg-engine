// Dynamic AABB tree for broad-phase collision detection

use super::shapes::AABB;
use std::collections::HashMap;

const NULL_NODE: usize = usize::MAX;
const AABB_MARGIN: f32 = 0.1; // Fattening margin to reduce updates

#[derive(Debug, Clone)]
struct TreeNode {
    aabb: AABB,
    object_id: Option<u32>, // None for internal nodes, Some for leaf nodes
    parent: usize,
    left: usize,
    right: usize,
    height: i32,
}

impl TreeNode {
    fn new_leaf(object_id: u32, aabb: AABB) -> Self {
        Self {
            aabb,
            object_id: Some(object_id),
            parent: NULL_NODE,
            left: NULL_NODE,
            right: NULL_NODE,
            height: 0,
        }
    }

    fn new_internal(left: usize, right: usize) -> Self {
        Self {
            aabb: AABB::new(
                crate::types::vector::Vec2::new(0.0, 0.0),
                crate::types::vector::Vec2::new(0.0, 0.0),
            ),
            object_id: None,
            parent: NULL_NODE,
            left,
            right,
            height: 0,
        }
    }

    fn is_leaf(&self) -> bool {
        self.object_id.is_some()
    }
}

/// Dynamic AABB tree for efficient spatial partitioning
#[derive(Debug)]
pub struct AABBTree {
    nodes: Vec<TreeNode>,
    root: usize,
    free_list: Vec<usize>,
    object_to_node: HashMap<u32, usize>,
}

impl AABBTree {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root: NULL_NODE,
            free_list: Vec::new(),
            object_to_node: HashMap::new(),
        }
    }

    /// Insert an object with its AABB
    pub fn insert(&mut self, object_id: u32, aabb: AABB) {
        // Fatten the AABB
        let fattened = aabb.fatten(AABB_MARGIN);

        // Allocate a leaf node
        let leaf_index = self.allocate_node();
        self.nodes[leaf_index] = TreeNode::new_leaf(object_id, fattened);
        self.object_to_node.insert(object_id, leaf_index);

        // Insert into tree
        self.insert_leaf(leaf_index);
    }

    /// Remove an object from the tree
    pub fn remove(&mut self, object_id: u32) -> bool {
        if let Some(&node_index) = self.object_to_node.get(&object_id) {
            self.remove_leaf(node_index);
            self.free_node(node_index);
            self.object_to_node.remove(&object_id);
            true
        } else {
            false
        }
    }

    /// Update an object's AABB. Returns false if the fattened AABB still contains it.
    pub fn update(&mut self, object_id: u32, new_aabb: AABB) -> bool {
        if let Some(&node_index) = self.object_to_node.get(&object_id) {
            let current_aabb = self.nodes[node_index].aabb;

            // Check if the new AABB is still within the fattened AABB
            if current_aabb.contains_point(&new_aabb.min)
                && current_aabb.contains_point(&new_aabb.max)
            {
                return false; // No update needed
            }

            // Remove and reinsert
            self.remove_leaf(node_index);
            self.nodes[node_index].aabb = new_aabb.fatten(AABB_MARGIN);
            self.insert_leaf(node_index);
            true
        } else {
            false
        }
    }

    /// Query all objects whose AABBs overlap the given AABB
    pub fn query(&self, aabb: &AABB) -> Vec<u32> {
        let mut results = Vec::new();

        if self.root == NULL_NODE {
            return results;
        }

        let mut stack = vec![self.root];

        while let Some(node_index) = stack.pop() {
            let node = &self.nodes[node_index];

            if !node.aabb.overlaps(aabb) {
                continue;
            }

            if node.is_leaf() {
                if let Some(object_id) = node.object_id {
                    results.push(object_id);
                }
            } else {
                if node.left != NULL_NODE {
                    stack.push(node.left);
                }
                if node.right != NULL_NODE {
                    stack.push(node.right);
                }
            }
        }

        results
    }

    /// Get all object IDs in the tree
    pub fn get_all_objects(&self) -> Vec<u32> {
        self.object_to_node.keys().copied().collect()
    }

    fn allocate_node(&mut self) -> usize {
        if let Some(index) = self.free_list.pop() {
            index
        } else {
            let index = self.nodes.len();
            self.nodes.push(TreeNode::new_leaf(
                0,
                AABB::new(
                    crate::types::vector::Vec2::new(0.0, 0.0),
                    crate::types::vector::Vec2::new(0.0, 0.0),
                ),
            ));
            index
        }
    }

    fn free_node(&mut self, index: usize) {
        self.free_list.push(index);
    }

    fn insert_leaf(&mut self, leaf_index: usize) {
        if self.root == NULL_NODE {
            self.root = leaf_index;
            self.nodes[leaf_index].parent = NULL_NODE;
            return;
        }

        // Find the best sibling using Surface Area Heuristic (SAH)
        let leaf_aabb = self.nodes[leaf_index].aabb;
        let mut index = self.root;

        while !self.nodes[index].is_leaf() {
            let node = &self.nodes[index];
            let left = node.left;
            let right = node.right;

            let combined_aabb = node.aabb.merge(&leaf_aabb);
            let combined_area = combined_aabb.surface_area();

            let cost_here = 2.0 * combined_area;
            let inheritance_cost = 2.0 * (combined_area - node.aabb.surface_area());

            // Cost of descending to left
            let cost_left = if self.nodes[left].is_leaf() {
                let aabb = self.nodes[left].aabb.merge(&leaf_aabb);
                aabb.surface_area() + inheritance_cost
            } else {
                let aabb = self.nodes[left].aabb.merge(&leaf_aabb);
                let old_area = self.nodes[left].aabb.surface_area();
                let new_area = aabb.surface_area();
                (new_area - old_area) + inheritance_cost
            };

            // Cost of descending to right
            let cost_right = if self.nodes[right].is_leaf() {
                let aabb = self.nodes[right].aabb.merge(&leaf_aabb);
                aabb.surface_area() + inheritance_cost
            } else {
                let aabb = self.nodes[right].aabb.merge(&leaf_aabb);
                let old_area = self.nodes[right].aabb.surface_area();
                let new_area = aabb.surface_area();
                (new_area - old_area) + inheritance_cost
            };

            // Choose best option
            if cost_here < cost_left && cost_here < cost_right {
                break;
            }

            // Descend
            if cost_left < cost_right {
                index = left;
            } else {
                index = right;
            }
        }

        let sibling = index;

        // Create a new parent
        let old_parent = self.nodes[sibling].parent;
        let new_parent_index = self.allocate_node();
        self.nodes[new_parent_index] = TreeNode::new_internal(sibling, leaf_index);
        self.nodes[new_parent_index].parent = old_parent;
        self.nodes[new_parent_index].aabb = self.nodes[sibling].aabb.merge(&leaf_aabb);

        if old_parent != NULL_NODE {
            // Sibling was not the root
            if self.nodes[old_parent].left == sibling {
                self.nodes[old_parent].left = new_parent_index;
            } else {
                self.nodes[old_parent].right = new_parent_index;
            }
        } else {
            // Sibling was the root
            self.root = new_parent_index;
        }

        self.nodes[sibling].parent = new_parent_index;
        self.nodes[leaf_index].parent = new_parent_index;

        // Walk back up and fix heights and AABBs
        let mut index = new_parent_index;
        while index != NULL_NODE {
            index = self.balance(index);

            let left = self.nodes[index].left;
            let right = self.nodes[index].right;

            if left != NULL_NODE && right != NULL_NODE {
                let left_height = self.nodes[left].height;
                let right_height = self.nodes[right].height;
                self.nodes[index].height = 1 + left_height.max(right_height);
                self.nodes[index].aabb = self.nodes[left].aabb.merge(&self.nodes[right].aabb);
            }

            index = self.nodes[index].parent;
        }
    }

    fn remove_leaf(&mut self, leaf_index: usize) {
        if self.root == leaf_index {
            self.root = NULL_NODE;
            return;
        }

        let parent = self.nodes[leaf_index].parent;
        let grand_parent = self.nodes[parent].parent;
        let sibling = if self.nodes[parent].left == leaf_index {
            self.nodes[parent].right
        } else {
            self.nodes[parent].left
        };

        if grand_parent != NULL_NODE {
            // Connect sibling to grandparent
            if self.nodes[grand_parent].left == parent {
                self.nodes[grand_parent].left = sibling;
            } else {
                self.nodes[grand_parent].right = sibling;
            }
            self.nodes[sibling].parent = grand_parent;

            // Free parent
            self.free_node(parent);

            // Walk back up and fix AABBs and heights
            let mut index = grand_parent;
            while index != NULL_NODE {
                index = self.balance(index);

                let left = self.nodes[index].left;
                let right = self.nodes[index].right;

                if left != NULL_NODE && right != NULL_NODE {
                    let left_height = self.nodes[left].height;
                    let right_height = self.nodes[right].height;
                    self.nodes[index].height = 1 + left_height.max(right_height);
                    self.nodes[index].aabb =
                        self.nodes[left].aabb.merge(&self.nodes[right].aabb);
                }

                index = self.nodes[index].parent;
            }
        } else {
            self.root = sibling;
            self.nodes[sibling].parent = NULL_NODE;
            self.free_node(parent);
        }
    }

    fn balance(&mut self, index: usize) -> usize {
        if self.nodes[index].is_leaf() || self.nodes[index].height < 2 {
            return index;
        }

        let left = self.nodes[index].left;
        let right = self.nodes[index].right;

        let balance = self.nodes[right].height - self.nodes[left].height;

        // Rotate right branch up
        if balance > 1 {
            let right_left = self.nodes[right].left;
            let right_right = self.nodes[right].right;

            // Swap index and right
            self.nodes[right].left = index;
            self.nodes[right].parent = self.nodes[index].parent;
            self.nodes[index].parent = right;

            let right_parent = self.nodes[right].parent;
            if right_parent != NULL_NODE {
                if self.nodes[right_parent].left == index {
                    self.nodes[right_parent].left = right;
                } else {
                    self.nodes[right_parent].right = right;
                }
            } else {
                self.root = right;
            }

            // Rotate
            if self.nodes[right_left].height > self.nodes[right_right].height {
                self.nodes[right].right = right_left;
                self.nodes[index].right = right_right;
                self.nodes[right_right].parent = index;

                self.nodes[index].aabb = self.nodes[left].aabb.merge(&self.nodes[right_right].aabb);
                self.nodes[right].aabb = self.nodes[index].aabb.merge(&self.nodes[right_left].aabb);

                let left_height = self.nodes[left].height;
                let right_right_height = self.nodes[right_right].height;
                self.nodes[index].height = 1 + left_height.max(right_right_height);

                let index_height = self.nodes[index].height;
                let right_left_height = self.nodes[right_left].height;
                self.nodes[right].height = 1 + index_height.max(right_left_height);
            } else {
                self.nodes[right].right = right_right;
                self.nodes[index].right = right_left;
                self.nodes[right_left].parent = index;

                self.nodes[index].aabb = self.nodes[left].aabb.merge(&self.nodes[right_left].aabb);
                self.nodes[right].aabb = self.nodes[index].aabb.merge(&self.nodes[right_right].aabb);

                let left_height = self.nodes[left].height;
                let right_left_height = self.nodes[right_left].height;
                self.nodes[index].height = 1 + left_height.max(right_left_height);

                let index_height = self.nodes[index].height;
                let right_right_height = self.nodes[right_right].height;
                self.nodes[right].height = 1 + index_height.max(right_right_height);
            }

            return right;
        }

        // Rotate left branch up
        if balance < -1 {
            let left_left = self.nodes[left].left;
            let left_right = self.nodes[left].right;

            // Swap index and left
            self.nodes[left].left = index;
            self.nodes[left].parent = self.nodes[index].parent;
            self.nodes[index].parent = left;

            let left_parent = self.nodes[left].parent;
            if left_parent != NULL_NODE {
                if self.nodes[left_parent].left == index {
                    self.nodes[left_parent].left = left;
                } else {
                    self.nodes[left_parent].right = left;
                }
            } else {
                self.root = left;
            }

            // Rotate
            if self.nodes[left_left].height > self.nodes[left_right].height {
                self.nodes[left].right = left_left;
                self.nodes[index].left = left_right;
                self.nodes[left_right].parent = index;

                self.nodes[index].aabb = self.nodes[right].aabb.merge(&self.nodes[left_right].aabb);
                self.nodes[left].aabb = self.nodes[index].aabb.merge(&self.nodes[left_left].aabb);

                let right_height = self.nodes[right].height;
                let left_right_height = self.nodes[left_right].height;
                self.nodes[index].height = 1 + right_height.max(left_right_height);

                let index_height = self.nodes[index].height;
                let left_left_height = self.nodes[left_left].height;
                self.nodes[left].height = 1 + index_height.max(left_left_height);
            } else {
                self.nodes[left].right = left_right;
                self.nodes[index].left = left_left;
                self.nodes[left_left].parent = index;

                self.nodes[index].aabb = self.nodes[right].aabb.merge(&self.nodes[left_left].aabb);
                self.nodes[left].aabb = self.nodes[index].aabb.merge(&self.nodes[left_right].aabb);

                let right_height = self.nodes[right].height;
                let left_left_height = self.nodes[left_left].height;
                self.nodes[index].height = 1 + right_height.max(left_left_height);

                let index_height = self.nodes[index].height;
                let left_right_height = self.nodes[left_right].height;
                self.nodes[left].height = 1 + index_height.max(left_right_height);
            }

            return left;
        }

        index
    }
}

impl Default for AABBTree {
    fn default() -> Self {
        Self::new()
    }
}
