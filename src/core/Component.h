//
// Created by Aram Aprahamian on 11/22/25.
//

#pragma once


#ifndef COMPONENT_H
#define COMPONENT_H
#include <string>
#include <vector>

#include "SFML/Graphics/Color.hpp"
#include "SFML/Graphics/Sprite.hpp"
#include "SFML/Graphics/Texture.hpp"
#include "SFML/System/String.hpp"
#include "SFML/System/Vector2.hpp"
#include "SFML/System/Vector3.hpp"
#include "spdlog/fmt/bundled/chrono.h"

namespace pyg {
    class Component {
    public:
        union property_value {
            int intValue;
            float floatValue;
            bool boolValue;
            sf::Color colorValue;
            sf::Vector2f vector2fValue;
            sf::Vector2i vector2iValue;
            sf::Vector2u vector2uValue;
            sf::Vector3f vector3iValue;
            sf::Vector3i vector3uValue;
            sf::Texture* textureValue;
            std::string stringValue;
            sf::String sfStringValue;
        };

        struct property {
            std::string name;
            int id;

            enum type {
                INT,
                FLOAT,
                BOOL,
                COLOR,
                VECTOR2F,
                VECTOR2I,
                VECTOR2U,
                VECTOR3F,
                VECTOR3I,
                VECTOR3U,
                TEXTURE,
                STRING,
                SF_STRING,
            } type;

            bool isEditable;
            property_value value;
        };

        Component() = default;
        ~Component() = default;

        explicit Component(long id = 0, const std::string &name = "");


        virtual void start();

        virtual void stop();

        virtual void pause();

        virtual void update();

        // Serialized properties
        virtual property* get_property(const std::string &property_name) const;

        virtual property* get_property_by_id(long property_id) const;

        virtual void set_property(const std::string &property_name, const property &value);

        virtual std::vector<std::string> get_all_property_names() const {
            std::vector<std::string> names;
            for (const auto prop : properties_) {
                if (prop == nullptr) {
                    continue;
                }

                names.push_back(prop->name);
            }
            return names;
        }

        virtual std::string get_name() const {
            return name_;
        }

        virtual void set_name(const std::string &name) {
            this->name_ = name;
        }

        virtual long get_id() const {
            return id_;
        }

    private:
        long id_ = 0;
        std::string name_ = std::string();
        std::vector<property*> properties_ = std::vector<property*>();
    };
} // pyg

#endif //COMPONENT_H
