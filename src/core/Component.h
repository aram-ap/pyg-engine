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

namespace pyg {
    class Component {
    public:
        union PropertyValue {
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

        struct Property {
            std::string name;

            enum Type {
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
            PropertyValue value;
        };

        Component() = default;

        explicit Component(long id = 0, const std::string &name = "");

        virtual ~Component() = default;

        virtual void start();

        virtual void stop();

        virtual void pause();

        virtual void update();

        // Serialized properties
        virtual Property getProperty(const std::string &propertyName) const;

        virtual Property getPropertyById(long propertyId) const;

        virtual void setProperty(const std::string &propertyName, const Property &value);

        virtual std::vector<std::string> getAllPropertyNames() const;

        virtual std::string getName() const;

        virtual void setName(const std::string &name);

        virtual long getId() const;

    private:
        long id;
        std::string name;
        std::vector<Property> properties;
    };
} // pyg

#endif //COMPONENT_H
