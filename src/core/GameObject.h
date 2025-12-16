//
// Created by Aram Aprahamian on 11/22/25.
//
#pragma once
#include <string>
#include <vector>
#include "Component.h"
#include "logging/Logger.h"
#include "SFML/System/Time.hpp"

#ifndef GAMEOBJECT_H
#define GAMEOBJECT_H

namespace pyg {
class GameObject final {
public:
    // Constructor with optional id and name (id=0 means auto-generate id)
    explicit GameObject(const std::string& name = "");
    explicit GameObject(const GameObject& obj) {
        const auto game_object = new GameObject(obj);
        game_object->set_name(this->name_);
        game_object->set_parent(this->parent_);
    }
    ~GameObject();

    void destroy() {
        delete this;
    }

    long get_id();
    bool is_enabled();
    void set_enabled(bool enabled);
    std::string get_name();
    void set_name(const std::string& name);
    void add_child(GameObject* child);
    GameObject* remove_child(GameObject* child);
    GameObject* remove_child_by_name(const std::string& name);
    GameObject* remove_child_by_id(const long id);
    GameObject* get_child_by_name(const std::string& name);

    static GameObject* get_object_by_id(const long id);
    GameObject* get_parent();
    bool contains_child(GameObject* child);
    void set_parent(GameObject* parent);
    Component* get_component_by_name(const std::string& name);
    Component* get_component_by_id(const long id);
    std::vector<Component *>* get_all_components();
    void add_component(Component* component);
    GameObject* clone();
    std::vector<GameObject*>* get_children();
    void remove_all_children();
    void remove_all_components();
    void update(const sf::Time delta_time);
    void fixed_update(const sf::Time delta_time);

    static long generate_uid() {
        auto rng = static_cast<uint32_t>(std::rand());
        while (get_object_map()[rng] != nullptr) {
            rng = std::rand();
        }
        return rng;
    }

private:
    long id_ = 0;
    bool enabled_ = true;
    std::string name_;
    std::vector<GameObject*> children_;
    GameObject* parent_ = nullptr;
    std::vector<Component*> components_;

    static std::unordered_map<long, GameObject*> get_object_map() {
        static std::unordered_map<long, GameObject*> game_object_map;
        return game_object_map;
    }
};

} // pyg

#endif //GAMEOBJECT_H
