//
// Created by Aram Aprahamian on 11/22/25.
//

#ifndef INPUTMANAGER_H
#define INPUTMANAGER_H
#include <vector>

#include "SFML/System/Vector2.hpp"

namespace pyg {
    // InputManager handles all input events and their corresponding callbacks.
    // From SFML, it's a singleton class.
    //
    class Input {
    public:
        enum axis {
            horizontal,
            vertical,
            left,
            right,
            jump,
            fire1,
            fire2,
            fire3,
            crouch,
            sprint,
            escape
        };


        enum KB {
            A='A', B='B', C='C', D='D', E='E', F='F', G='G', H='H', I='I', J='J', K='K', L='L', M='M', N='N', O='O', P='P', Q='Q', R='R', S='S', T='T', U='U', V='V', W='W', X='X', Y='Y', Z='Z',
            ZERO='0', ONE='1', TWO='2', THREE='3', FOUR='4', FIVE='5', SIX='6', SEVEN='7', EIGHT='8', NINE='9',
            MINUS='-', PLUS='+', L_BRKT='[', R_BRKT=']', BK_SLASH='\\', FWD_SLASH='/', SLASH='/', SEMI_COLON=';', QUOTE='\'', LESS_THAN='<', L_CARROT='<', GREATER_THAN='>', R_CARROT='>',
            L_ARROW, R_ARROW, UP_ARROW, DOWN_ARROW, ESCAPE, L_CTRL, R_CTRL, L_SHIFT, R_SHIFT, LEFT_SHIFT, RIGHT_SHIFT, L_ALT, R_ALT, SPACE, ENTER, BK_SPACE, TAB
        };

        enum MB {
            LEFT_CLICK='l', RIGHT_CLICK='r', MIDDLE_CLICK='m',
            L_CLK, R_CLK, M_CLK
        };

        union InputButton {
            KB button;
            MB mouse_button;
        };

        void update();

        int get_mouse_x();

        int get_mouse_y();

        auto isMouseButtonPressed(MB mouse_button) -> bool;

        bool is_mouse_button_released(MB mouse_button);

        bool is_mouse_button_down(MB mouse_button);

        bool is_mouse_button_up(MB mouse_button);

        bool is_key_pressed(KB key);

        bool is_key_released(KB key);

        bool is_key_down(KB key);

        bool is_key_up(KB key);

        /**
         * Allows multicharacter mapping to axis values for better usability
         * @param axis (i.e., Horizontal, Vertical, etc)
         * @return The float value [-1, 1] of the specific axis
         */
        auto getAxis(axis axis) -> float;

        // /**
        //  *
        //  * @param axis What axis to reassign
        //  * @param button The specific button to assign the button
        //  */
        // void assignAxis(Axis axis, InputButton button);
        //
        // /**
        //  *
        //  * @param axis What axis to reassign
        //  * @param Keybinds A vector of buttons
        //  */
        // void assignAxis(Axis axis, std::vector<InputButton> Keybinds);

        // Singleton class
        static Input &get_instance() {
            static Input instance;
            return instance;
        }

        ~Input() = default;
        // Input(Input const&) = delete;
        // void operator=(Input const&) = delete;


    private:
        Input() {
            // mousePosition = sf::Vector2i(0, 0);
            // for (int i = 0; i < 3; i++) {
            //     mouseButtonPressed[i] = false;
            //     mouseButtonReleased[i] = false;
            //     mouseButtonDown[i] = false;
            //     mouseButtonUp[i] = false;
            // }
            // for (int i = 0; i < 512; i++) {
            //     keyPressed[i] = false;
            //     keyReleased[i] = false;
            //     keyDown[i] = false;
            //     keyUp[i] = false;
            // }
        }

        Input(Input const&);
        void operator=(Input const&);

        sf::Vector2i mouse_position_ = sf::Vector2i(0, 0);
        bool mouse_button_pressed_[3];
        bool mouse_button_released_[3];
        bool mouse_button_down_[3];
        bool mouse_button_up_[3];
        bool key_pressed_[512];
        bool key_released_[512];
        bool key_down_[512];
        bool key_up_[512];
    };
} // pyg

#endif //INPUTMANAGER_H
