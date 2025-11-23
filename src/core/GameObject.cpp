//
// Created by Aram Aprahamian on 11/22/25.
//
#pragma once
#include "GameObject.h"
#include "Component.h"

namespace pyg {

    GameObject::GameObject(const long id, const std::string &name): id(0), enabled(true), name(name) {
        // Assign a unique ID if none is provided
        if (id == 0) {
            static long nextId = 1;
            this->id = nextId++;
        } else {
            this->id = id;
        }
    }

    GameObject::~GameObject() {
    }

    long GameObject::getId() {
    }

    bool GameObject::isEnabled() {
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

    void GameObject::removeAllChildren() {
    }
} // pyg