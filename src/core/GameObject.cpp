//
// Created by Aram Aprahamian on 11/22/25.
//
#include "GameObject.h"
#include "Component.h"
#include "SFML/System/Time.hpp"

namespace pyg {

    GameObject::GameObject(const std::string &name) : enabled_(true), name_(name) {
        // Constructor

        // Initialize the id
        id_ = generate_uid();
        get_object_map()[id_] = this;

        // Initialize the enabled state
        enabled_ = true;

        // Initialize the parent
        parent_ = nullptr;
    }

    GameObject::~GameObject() {
        // Destructor

        // Remove this object from the scene
        // TODO: Fill This

        // Reset the parent
        if (parent_ != nullptr) {
            parent_->remove_child(this);
            parent_ = nullptr;
        }
        // remove components
        this->remove_all_components();
        // remove children
        this->remove_all_children();
        // Reset the scene

        // Delete this object
        delete this;
    }

    long GameObject::get_id() {
        // Get the id
        return id_;
    }


    bool GameObject::is_enabled() {
        // Get the enabled state
        // Return true if enabled, false otherwise
        // Default is true
        return enabled_;
    }

    void GameObject::set_enabled(bool enabled) {
        this->enabled_ = enabled;
    }

    std::string GameObject::get_name() {
        return name_;
    }

    void GameObject::set_name(const std::string &name) {
        if (name.empty()) {
            return;
        }

        this->name_ = name;
    }

    void GameObject::add_child(GameObject *child) {


    }

    GameObject * GameObject::remove_child(GameObject *child) {
        GameObject * rtrn = nullptr;
        for (int i = 0; i < children_.size(); i++) {
            if (children_[i] == child) {
                rtrn = children_[i];
                children_.erase(children_.begin() + i);
                break;
            }
        }
        return rtrn;
    }

    GameObject * GameObject::remove_child_by_name(const std::string &name) {
        GameObject *rtrn = nullptr;
        for (int i = 0; i < children_.size(); i++) {
            if (children_[i] != nullptr && children_[i]->get_name() == name) {
                rtrn = children_[i];
                children_.erase(children_.begin() + i);
                break;
            }
        }

        return rtrn;
    }

    GameObject * GameObject::remove_child_by_id(const long id) {
        GameObject *rtrn = nullptr;
        for (int i = 0; i < children_.size(); i++) {
            if (children_[i] != nullptr && children_[i]->get_id() == id) {
                rtrn = children_[i];
                children_.erase(children_.begin() + i);
                break;
            }
        }
        return rtrn;
    }

    /**
     * Searches and returns a game object by name
     * NOTE: This function searches in O(N) time, consider using getChildById
     * @param name
     * @return nullptr if no child by the name exists
     */
    GameObject * GameObject::get_child_by_name(const std::string &name) {
        for (const auto child: children_) {
            if (child != nullptr && child->get_name() == name) {
                return child;
            }
        }
        return nullptr;
    }

    /**
     * Searches and returns a game object by ID
     * Searches in O(1) time
     * @param id GameObject UID
     * @return nullptr if no child by the ID exists
     */
    GameObject * GameObject::get_object_by_id(const long id) {
        return get_object_map()[id];
    }

    GameObject * GameObject::get_parent() {
        return parent_;
    }

    bool GameObject::contains_child(GameObject *child) {
        if (child == nullptr) {
            return false;
        }

        for (const auto obj : children_) {
            if (obj == child) {
                return true;
            }
        }

        return false;
    }

    void GameObject::set_parent(GameObject *parent) {
        this->parent_ = parent;
    }

    GameObject * GameObject::clone() {
        return new GameObject(*this);
    }

    std::vector<GameObject *>* GameObject::get_children() {
        return &children_;
    }

    Component * GameObject::get_component_by_name(const std::string &name) {
        if (name.empty()) {
            return nullptr;
        }

        for (const auto component : components_) {
            if (component->get_name() == name) {
                return component;
            }
        }
        return nullptr;
    }

    Component * GameObject::get_component_by_id(const long id) {
        for (const auto component : components_) {
            if (component->get_id() == id) {
                return component;
            }
        }
        return nullptr;
    }

    std::vector<Component *>* GameObject::get_all_components() {
        return &components_;
    }

    void GameObject::add_component(Component *component) {
        if (component == nullptr) {
            return;
        }

        if (get_object_by_id(component->get_id()) == nullptr) {
            components_.push_back(component);
        }
    }

    void GameObject::update(const sf::Time delta_time) {
        for (const auto component : components_) {
            if (component == nullptr) {
                continue;
            }
            component->update();
        }
    }

    void GameObject::fixed_update(const sf::Time delta_time) {
        // TODO: Fill this
    }

    void GameObject::remove_all_children() {
        for (const auto child : children_) {
            if (child == nullptr) {
                continue;
            }
            delete child;
        }
    }

    void GameObject::remove_all_components() {
        for (const auto component : components_) {
            if (component == nullptr) {
                continue;
            }
            delete component;
        }
    }
} // pyg
