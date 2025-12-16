//
// Created by Aram Aprahamian on 11/22/25.
//

#include "InputManager.h"
#include <SFML/Window/Window.hpp>
#include <SFML/Window/Mouse.hpp>
#include <SFML/Window/Keyboard.hpp>
#include <SFML/Window/Event.hpp>

namespace pyg {
    int Input::get_mouse_x() {
    }

    int Input::get_mouse_y() {
    }

    bool Input::isMouseButtonPressed(MB mouse_button) {
    }

    bool Input::is_mouse_button_released(MB mouse_button) {
    }

    bool Input::is_mouse_button_down(MB mouse_button) {
    }

    bool Input::is_mouse_button_up(MB mouse_button) {
    }

    bool Input::is_key_pressed(KB key) {
    }

    bool Input::is_key_released(KB key) {
    }

    bool Input::is_key_down(KB key) {
    }

    bool Input::is_key_up(KB key) {
    }

    float Input::getAxis(axis axis) {
    }

    void Input::update() {
        mouse_position_ = sf::Mouse::getPosition();
        for (int i = 0; i < 3; i++) {
            mouse_button_pressed_[i] = sf::Mouse::isButtonPressed(static_cast<sf::Mouse::Button>(i));
            // mouseButtonReleased[i] = sf::Mouse::isButtonReleased(static_cast<sf::Mouse::Button>(i));
            // mouseButtonDown[i] = sf::Mouse::isButtonDown(static_cast<sf::Mouse::Button>(i));
            // mouseButtonUp[i] = sf::Mouse::isButtonUp(static_cast<sf::Mouse::Button>(i));
        }
        for (int i = 0; i < 512; i++) {
            key_pressed_[i] = sf::Keyboard::isKeyPressed(static_cast<sf::Keyboard::Key>(i));
            // keyReleased[i] = sf::Keyboard::isKeyReleased(static_cast<sf::Keyboard::Key>(i));
            // keyDown[i] = sf::Keyboard::isKeyDown(static_cast<sf::Keyboard::Key>(i));
            // keyUp[i] = sf::Keyboard::isKeyUp(static_cast<sf::Keyboard::Key>(i));
        }

        // TODO: Handle other input events
        // sf::Event event;
        // while (window.pollEvent(event)) {
        //     if (event.type == sf::Event::Closed) {
        //         window.close();
        //     }
        // }
    }

    static int getMouseX() {
        // return mousePosition->x;
        return 0;
    }

    static int getMouseY() {
        // return mousePosition.y;
        return 0;
    }
} // pyg
