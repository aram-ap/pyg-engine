#pragma once

#ifndef ENGINE_H
#define ENGINE_H

#include <SFML/System.hpp>
#include "logging/Logger.h"
#include "input/InputManager.h"
#include <string>
#include "core/GameObject.h"
#include "../rendering/Window.h"

namespace pyg {

class Engine final {
public:
    static const std::string VERSION;

    Engine();
    ~Engine();

    std::string get_version() const;
    int get_tick_rate() const;
    void set_tick_rate(int tick_rate);
    void update(sf::Time delta_time);
    void fixed_update(sf::Time delta_time);
    void render();
    void on_destroy();
    void log_type(Logger::Type type, std::string msg);
    void log(std::string msg);
    bool is_running() const;
    void start();
    void stop();
    void pause();
    void resume();
    void restart();

    // Game Object Items
    // void add_game_object(GameObject* game_object);
    void add_game_object(GameObject* game_object);
    GameObject* search_game_object_by_name(const std::string &name) const;
    void remove_game_object(GameObject* game_object) const;
    void remove_all_game_objects();

    // Window things

    void set_window_title(std::string title);
    std::string get_window_title() const;
    void set_window_icon(std::string icon);
    std::string get_window_icon() const;
    void set_window_icon(int width, int height, const unsigned char* data);
    void set_window_position(int x, int y);
    // virtual void setWindowStyle(Window::Style style);
    // virtual Window::Style getWindowStyle() const;
    void set_window_vertical_sync_enabled(bool enabled);
    bool is_window_vertical_sync_enabled() const;
    void set_window_framerate_limit(unsigned int limit);
    unsigned int get_window_framerate_limit() const;
    void set_window_mouse_cursor_visible(bool visible);
    bool is_window_mouse_cursor_visible() const;
    void set_window_mouse_cursor_grabbed(bool grabbed);
    bool is_window_mouse_cursor_grabbed() const;
    void set_window_size(int width, int height);
    void set_window_visible(bool visible);
    bool is_window_visible() const;
    void set_window(Window* window);


    Window* get_window() const;
    /**
     * @return The total engine runtime
     */
    sf::Time get_elapsed_time() const;
    void exit();

private:
    int tick_rate_ = 0;
    Window* window_ = nullptr;
    bool owns_window_ = false;
    bool window_visible_ = false;
    sf::Clock clock_ = sf::Clock();       //
    sf::Clock system_clock_ = sf::Clock();     // Basically don't restart this unless the engine is fully restarted
    bool is_paused_ = false;
    bool is_running_;
    std::string window_icon_path_ = std::string();
    std::vector<GameObject*> game_objects_ = std::vector<GameObject*>();
};


} // namespace pyg
#endif

