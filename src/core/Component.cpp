//
// Created by Aram Aprahamian on 11/22/25.
//

#include "Component.h"

namespace pyg {
    Component::Component(const long id, const std::string &name) : id_(id), name_(name) {
    }

    void Component::set_property(const std::string &property_name, const property &value) {
    }

    void Component::start() {
    }

    void Component::stop() {
    }

    void Component::pause() {
    }

    void Component::update() {
    }

    Component::property* Component::get_property(const std::string &property_name) const {
        property* find = nullptr;
        if (!property_name.empty()) {
            for (const auto prop : properties_) {
                if (prop == nullptr || prop->name != property_name) {
                    continue;
                }
                find = prop;
                break;
            }
        }
        return find;
    }

    Component::property* Component::get_property_by_id(long property_id) const {
        property* find = nullptr;
        if (property_id != 0) {
            for (const auto prop : properties_) {
                if (prop == nullptr || prop->id != property_id) {
                    continue;
                }
                find = prop;
                break;
            }
        }

        return find;
    }
} // pyg