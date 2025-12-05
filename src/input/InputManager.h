//
// Created by Aram Aprahamian on 11/22/25.
//

#ifndef INPUTMANAGER_H
#define INPUTMANAGER_H

namespace pyg {

// InputManager handles all input events and their corresponding callbacks.
// From SFML, it's a singleton class.
//
class InputManager {
    public:
        static InputManager& getInstance();
        void update();
        void setMousePosition(int x, int y);
        int getMouseX();
        int getMouseY();
        bool isMouseButtonPressed(int button);
        bool isMouseButtonReleased(int button);
        bool isMouseButtonDown(int button);
        bool isMouseButtonUp(int button);
        bool isKeyPressed(int key);
        bool isKeyReleased(int key);
        bool isKeyDown(int key);
        bool isKeyUp(int key);

    private:
        InputManager();
        ~InputManager();

        sf::Vector2i mousePosition;
        bool mouseButtonPressed[3];
        bool mouseButtonReleased[3];
        bool mouseButtonDown[3];
        bool mouseButtonUp[3];
        bool keyPressed[512];
        bool keyReleased[512];
        bool keyDown[512];
        bool keyUp[512];
};

} // pyg

#endif //INPUTMANAGER_H
