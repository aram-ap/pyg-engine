//
// Created by Aram Aprahamian on 11/22/25.
//
#pragma once
#include "GameObject.h"
#include "Component.h"
#include "SFML/System/Time.hpp"

namespace pyg {

    GameObject::GameObject(const long id, const std::string &name): id(0), enabled(true), name(name) {
        // Constructor

        // Initialize the id
        if (id == 0) {
            static long nextId = 1;
            this->id = nextId++;
        } else {
            this->id = id;
        }

        // Initialize the enabled state

        // Initialize the name

        // Initialize the parent

        // Initialize the components

        // Initialize the children

        // Initialize the scene

        // Add this object to the scene

    }

    GameObject::~GameObject() {
        // Destructor
        // Clean up any resources
        // Remove this object from the scene
        // Remove all children
        // Remove all components
        // Delete this object
        // Reset the id
        // Reset the enabled state
        // Reset the name
        // Reset the parent
        // Reset the components
        // Reset the children
        // Reset the scene
    }

    long GameObject::getId() {
        // Get the id
        return id;
    }


    bool GameObject::isEnabled() {
        // Get the enabled state
        // Return true if enabled, false otherwise
        // Default is true
        return enabled;
    }

    void GameObject::setEnabled(bool enabled) {
    }

    std::string GameObject::getName() {
    }

    void GameObject::setName(const std::string &name) {
    }

    void GameObject::addChild(GameObject *child) {
    }

    GameObject * GameObject::removeChild(GameObject *child) {
    }

    GameObject * GameObject::removeChildByName(const std::string &name) {
    }

    GameObject * GameObject::removeChildById(const long id) {
    }

    GameObject * GameObject::getChildByName(const std::string &name) {
    }

    GameObject * GameObject::getChildById(const long id) {
    }

    GameObject * GameObject::getParent() {
    }

    void GameObject::setParent(GameObject *parent) {
    }

    GameObject * GameObject::clone() {
    }

    std::vector<GameObject *> GameObject::getChildren() {
    }

    Component * GameObject::getComponentByName(const std::string &name) {
    }

    Component * GameObject::getComponentById(const long id) {
    }

    std::vector<Component *> GameObject::getAllComponents() {
    }

    void GameObject::addComponent(Component *component) {
    }

    void GameObject::update(const sf::Time deltaTime) {
    }

    void GameObject::fixedUpdate(const sf::Time deltaTime) {

    }

    void GameObject::removeAllChildren() {
    }
} // pyg
