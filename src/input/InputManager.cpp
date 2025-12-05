//
// Created by Aram Aprahamian on 11/22/25.
//

#include "InputManager.h"
#include <SFML/Window/Window.hpp>
#include <SFML/Window/Mouse.hpp>
#include <SFML/Window/Keyboard.hpp>
#include <SFML/Window/Event.hpp>

namespace pyg {
    InputManager::InputManager() {
        mousePosition = sf::Vector2i(0, 0);
        for (int i = 0; i < 3; i++) {
            mouseButtonPressed[i] = false;
            mouseButtonReleased[i] = false;
            mouseButtonDown[i] = false;
            mouseButtonUp[i] = false;
        }
        for (int i = 0; i < 512; i++) {
            keyPressed[i] = false;
            keyReleased[i] = false;
            keyDown[i] = false;
            keyUp[i] = false;
        }
    }

    InputManager::~InputManager() {
    }

    void InputManager::update() {
        mousePosition = sf::Mouse::getPosition();
        for (int i = 0; i < 3; i++) {
            mouseButtonPressed[i] = sf::Mouse::isButtonPressed(static_cast<sf::Mouse::Button>(i));
            mouseButtonReleased[i] = sf::Mouse::isButtonReleased(static_cast<sf::Mouse::Button>(i));
            mouseButtonDown[i] = sf::Mouse::isButtonDown(static_cast<sf::Mouse::Button>(i));
            mouseButtonUp[i] = sf::Mouse::isButtonUp(static_cast<sf::Mouse::Button>(i));
        }
        for (int i = 0; i < 512; i++) {
            keyPressed[i] = sf::Keyboard::isKeyPressed(static_cast<sf::Keyboard::Key>(i));
            keyReleased[i] = sf::Keyboard::isKeyReleased(static_cast<sf::Keyboard::Key>(i));
            keyDown[i] = sf::Keyboard::isKeyDown(static_cast<sf::Keyboard::Key>(i));
            keyUp[i] = sf::Keyboard::isKeyUp(static_cast<sf::Keyboard::Key>(i));
        }

        // TODO: Handle other input events
        // sf::Event event;
        // while (window.pollEvent(event)) {
        //     if (event.type == sf::Event::Closed) {
        //         window.close();
        //     }
        // }
    }

    void InputManager::setMousePosition(int x, int y) {
        mousePosition = sf::Vector2i(x, y);
    }

    int InputManager::getMouseX() {
        return mousePosition.x;
    }

    int InputManager::getMouseY() {
        return mousePosition.y;
    }

    bool InputManager::isMouseButtonPressed(int button) {
        return mouseButtonPressed[button];
    }

    bool InputManager::isMouseButtonReleased(int button) {
        return mouseButtonReleased[button];
    }

    bool InputManager::isMouseButtonDown(int button) {
        return mouseButtonDown[button];
    }

    bool InputManager::isMouseButtonUp(int button) {
        return mouseButtonUp[button];
    }

    bool InputManager::isKeyPressed(int key) {
        return keyPressed[key];
    }

    bool InputManager::isKeyReleased(int key) {
        return keyReleased[key];
    }

    bool InputManager::isKeyDown(int key) {
        return keyDown[key];
    }

    bool InputManager::isKeyUp(int key) {
        return keyUp[key];
    }

    InputManager& InputManager::getInstance() {
        static InputManager instance;
        return instance;
    }
} // pyg
