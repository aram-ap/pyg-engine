//
// Created by Aram Aprahamian on 11/22/25.
//
#pragma once

#include <string>
#include <vector>
#include "Component.h"
#include "logging/Logger.h"

#ifndef GAMEOBJECT_H
#define GAMEOBJECT_H

namespace pyg {

class GameObject {
public:
    // Constructor with optional id and name (id=0 means auto-generate id)
    GameObject(long id = 0, const std::string& name = "");
    virtual ~GameObject();

    virtual long getId();
    virtual bool isEnabled();
    virtual void setEnabled(bool enabled);
    virtual std::string getName();
    virtual void setName(const std::string& name);
    virtual void addChild(GameObject* child);
    virtual GameObject* removeChild(GameObject* child);
    virtual GameObject* removeChildByName(const std::string& name);
    virtual GameObject* removeChildById(const long id);
    virtual GameObject* getChildByName(const std::string& name);
    virtual GameObject* getChildById(const long id);
    virtual GameObject* getParent();
    virtual void setParent(GameObject* parent);
    virtual Component* getComponentByName(const std::string& name);
    virtual Component* getComponentById(const long id);
    virtual std::vector<Component *> getAllComponents();
    virtual void addComponent(Component* component);
    virtual GameObject* clone();
    virtual std::vector<GameObject*> getChildren();
    virtual void removeAllChildren();

private:
    long id;
    bool enabled;
    std::string name;
    std::vector<GameObject*> children;
    GameObject* parent = nullptr;
};

} // pyg

#endif //GAMEOBJECT_H
