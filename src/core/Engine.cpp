#include "Engine.h"
#include "rendering/Window.h"
#include "logging/Logger.h"
#include <cstdlib>
#include <filesystem>
#include <vector>

namespace pyg {
    const std::string Engine::VERSION = "0.1.0";

    /* @param name: The name of the engine
     * @param tickRate: The tick rate of the engine
     *
     */
    Engine::Engine()
        : tick_rate_(60),
          window_(nullptr),
          owns_window_(true),
          window_visible_(true),
          is_paused_(false)
    {

        if (!Logger::getCoreLogger()) {
            Logger::init("pyg_engine");
        }


        PYG_INFO("Engine initialized - Version: {}", VERSION);
        clock_.restart();
    }

    Engine::~Engine() {
        // Free any GameObjects
        for (const GameObject* child : game_objects_) {
            if (child == nullptr)
                continue;
            delete child;
        }

        if (owns_window_) {
            delete window_;
        }
        window_ = nullptr;
    }

    /* @return: The version of the engine
     *
     */
    std::string Engine::get_version() const {
        return VERSION;
    }

    /* @return: The tick rate
     *
     */
    int Engine::get_tick_rate() const {
        return tick_rate_;
    }

    /* @param tickRateValue: The tick rate to set
     *
     */
    void Engine::set_tick_rate(int tickRateValue) {
        tick_rate_ = tickRateValue;
    }

    /* @param deltaTime: The time elapsed since the last frame
     * Called every frame
     */
    void Engine::update(const sf::Time delta_time) {
        Input::get_instance().update();

        if (is_paused_) {
            clock_.restart();
            return;
        }

        for (GameObject* _child : game_objects_) {
            if (_child == nullptr)
                continue;

            _child->update(delta_time);
        }

        if (window_) {
            window_->pollEvents();
        }
    }

    // TODO:
    void Engine::fixed_update(sf::Time delta_time) { }

    void Engine::render() {
        if (window_ == nullptr) {
            return;
        }

        window_->clear();
        window_->display();
    }

    void Engine::on_destroy() {
        PYG_INFO("Engine shutting down");
        Logger::shutdown();
    }

    /* @param type: The type of log
     * @param msg: The message to log
     *
     */
    void Engine::log_type(Logger::Type type, std::string msg) {
        Logger::print(type, msg);
    }

    /* @param msg: The message to log
     *
     */
    void Engine::log(std::string msg) {
        PYG_INFO(msg);
    }

    bool Engine::is_running() const {
        return is_running_;
    }

    void Engine::start() {
        // Only create window if visibility is enabled
        if (!window_ && window_visible_) {
            window_ = new Window();
            window_->setVisible(window_visible_);  // Set visibility before creation
            window_->create();
            owns_window_ = true;
        }
        clock_.restart();

        PYG_INFO("Engine starting");
        is_running_ = true;

        // If no window was created (visibility is false), run without window
        while (is_running_) {
            if (window_visible_ == false) {
                const sf::Time frame_time = clock_.restart();
                update(frame_time);
            } else if (window_->isOpen()) {
                const sf::Time frame_time = clock_.restart();
                update(frame_time);
                render();
            }
        }

        PYG_INFO("Engine stopped");
        on_destroy();
    }

    void Engine::stop() {
        PYG_INFO("Engine stopping");
        is_running_ = false;
        for (const GameObject* child : game_objects_) {
            if (child == nullptr)
                continue;

            delete child;
            child = nullptr;
        }

        if (window_) {
            window_->close();
        }
    }

    void Engine::pause() {
        is_paused_ = true;
    }

    void Engine::resume() {
        is_paused_ = false;
    }

    void Engine::restart() {
        PYG_INFO("Restarting core");
        stop();
        start();
    }

    // GameObject management methods
    void Engine::add_game_object(GameObject* game_object) {
        if (game_object != nullptr) {
            game_objects_.push_back(game_object);
        }
    }

    GameObject* Engine::search_game_object_by_name(const std::string &name) const {
        for (GameObject* obj : game_objects_) {
            if (obj != nullptr && obj->get_name() == name) {
                return obj;
            }
        }
        return nullptr;
    }

    void Engine::remove_game_object(GameObject* game_object) const {
        if (game_object == nullptr) {
            return;
        }

        for (GameObject* obj : game_objects_) {
            if (obj != nullptr && obj->get_name() == game_object->get_name()) {
                delete obj;
                break;
            }
        }
    }

    void Engine::remove_all_game_objects() {
        for (GameObject* obj : game_objects_) {
            if (obj != nullptr) {
                delete obj;
            }
        }

        game_objects_.clear();
    }

    void Engine::set_window_title(std::string title) {
        if (!window_) {
            return;
        }
        window_->setTitle(title);
    }

    std::string Engine::get_window_title() const {
        if (!window_) {
            return "";
        }
        return window_->getTitle();
    }

    void Engine::set_window_icon(std::string icon) {
        if (!window_) {
            return;
        }
        window_icon_path_ = icon;
        window_->setIcon(icon);
    }

    std::string Engine::get_window_icon() const {
        return window_icon_path_;
    }

    void Engine::set_window_icon(int width, int height, const unsigned char* data) {
        if (!window_) {
            return;
        }
        window_icon_path_ = ""; // Clear path when setting from raw data
        window_->setIcon(static_cast<unsigned int>(width), static_cast<unsigned int>(height), data);
    }

    void Engine::set_window_position(int x, int y) {
        if (!window_) {
            return;
        }
        window_->setPosition(sf::Vector2i(x, y));
    }

    void Engine::set_window_vertical_sync_enabled(bool enabled) {
        if (!window_) {
            return;
        }
        window_->setVerticalSyncEnabled(enabled);
    }

    bool Engine::is_window_vertical_sync_enabled() const {
        if (!window_) {
            return false;
        }
        return window_->isVerticalSyncEnabled();
    }

    void Engine::set_window_framerate_limit(unsigned int limit) {
        if (!window_) {
            return;
        }
        window_->setFramerateLimit(limit);
    }

    unsigned int Engine::get_window_framerate_limit() const {
        if (!window_) {
            return 0;
        }
        return window_->getFramerateLimit();
    }

    void Engine::set_window_mouse_cursor_visible(bool visible) {
        if (!window_) {
            return;
        }
        window_->setMouseCursorVisible(visible);
    }

    bool Engine::is_window_mouse_cursor_visible() const {
        if (!window_) {
            return true;
        }
        return window_->isMouseCursorVisible();
    }

    void Engine::set_window_mouse_cursor_grabbed(bool grabbed) {
        if (!window_) {
            return;
        }
        window_->setMouseCursorGrabbed(grabbed);
    }

    bool Engine::is_window_mouse_cursor_grabbed() const {
        if (!window_) {
            return false;
        }
        return window_->isMouseCursorGrabbed();
    }

    void Engine::set_window_size(int width, int height) {
        if (!window_) {
            return;
        }
        window_->setSize(sf::Vector2u(static_cast<unsigned int>(width), static_cast<unsigned int>(height)));
    }

    void Engine::set_window_visible(bool visible) {
        window_visible_ = visible;
        if (window_) {
            window_->setVisible(visible);
        }
    }

    bool Engine::is_window_visible() const {
        if (window_) {
            return window_->isVisible();
        }
        return window_visible_;
    }

    /* @param window: The window to set
     *
     */
    void Engine::set_window(Window* window) {
        if (!window) {
            return;
        }

        if (owns_window_ && window_) {
            delete window_;
        }

        window_ = window;
        owns_window_ = false;
    }

    /* @return: The window
     *
     */
    Window* Engine::get_window() const {
        return window_;
    }

    sf::Time Engine::get_elapsed_time() const {
        return system_clock_.getElapsedTime();
    }

    void Engine::exit() {
        stop();
        std::exit(0);
    }

} // namespace pyg
