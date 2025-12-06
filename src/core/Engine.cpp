// #pragma once
#include "Engine.h"
#include "rendering/Window.h"
#include "logging/Logger.h"
#include <SFML/System/Sleep.hpp>
#include <cstdlib>

namespace pyg {
    const std::string Engine::VERSION = "0.1.0";

    /* @param name: The name of the engine
     * @param tickRate: The tick rate of the engine
     *
     */
    Engine::Engine()
        : tickRate(60),
          _window(nullptr),
          _ownsWindow(true),
          _windowVisible(true),
          _isPaused(false),
          _isRunning(false),
          _windowIconPath("") {

        if (!Logger::getCoreLogger()) {
            Logger::init("pyg_engine");
        }

        PYG_INFO("Engine initialized - Version: {}", VERSION);
        _clock.restart();
    }

    Engine::~Engine() {
        if (_ownsWindow) {
            delete _window;
        }
        _window = nullptr;
    }

    void Engine::stop() {
        if (!_isRunning) {
            return;
        }
        PYG_INFO("Engine stopping");
        _isRunning = false;
        if (_window) {
            _window->close();
        }
    }

    bool Engine::isRunning() const {
        return _isRunning;
    }

    void Engine::start() {
        // Only create window if visibility is enabled
        if (!_window && _windowVisible) {
            _window = new Window();
            _window->setVisible(_windowVisible);  // Set visibility before creation
            _window->create();
            _ownsWindow = true;
        }

        PYG_INFO("Engine starting");
        _isRunning = true;

        // If no window was created (visibility is false), run without window
        if (!_window) {
            while (_isRunning) {
                sf::Time frameTime = _clock.restart();
                update(frameTime);
                render();
            }
        } else {
            while (_isRunning && _window->isOpen()) {
                sf::Time frameTime = _clock.restart();
                update(frameTime);
                render();
            }
        }

        PYG_INFO("Engine stopped");
        on_destroy();
    }

    /* @param deltaTime: The time elapsed since the last frame
     *
     */
    void Engine::update(sf::Time deltaTime) {
        if (_isPaused) {
            return;
        }

        if (tickRate > 0) {
            const float targetSeconds = 1.0f / static_cast<float>(tickRate);
            if (deltaTime.asSeconds() < targetSeconds) {
                sf::sleep(sf::seconds(targetSeconds - deltaTime.asSeconds()));
                deltaTime = _clock.restart();
            }
        }

        if (_window) {
            _window->pollEvents();
        }
    }

    void Engine::render() {
        if (!_window) {
            return;
        }

        _window->clear();
        _window->display();
    }

    void Engine::pause() {
        _isPaused = true;
    }

    void Engine::resume() {
        _isPaused = false;
    }

    void Engine::restart() {
        PYG_INFO("Restarting core");
        stop();
        start();
    }

    void Engine::exit() {
        stop();
        std::exit(0);
    }

    /* @param window: The window to set
     *
     */
    void Engine::setWindow(Window* window) {
        if (!window) {
            return;
        }

        if (_ownsWindow && _window) {
            delete _window;
        }

        _window = window;
        _ownsWindow = false;
    }

    /* @return: The window
     *
     */
    Window* Engine::getWindow() const {
        return _window;
    }

    void Engine::on_destroy() {
        PYG_INFO("Engine shutting down");
        Logger::shutdown();
    }

    /* @param msg: The message to log
     *
     */
    void Engine::log(std::string msg) {
        PYG_INFO(msg);
    }

    /* @param tickRateValue: The tick rate to set
     *
     */
    void Engine::setTickRate(int tickRateValue) {
        tickRate = tickRateValue;
    }

    /* @return: The tick rate
     *
     */
    int Engine::getTickRate() const {
        return tickRate;
    }

    /* @param type: The type of log
     * @param msg: The message to log
     *
     */
    void Engine::logType(Logger::Type type, std::string msg) {
        Logger::print(type, msg);
    }

    /* @return: The version of the engine
     *
     */
    std::string Engine::getVersion() const {
        return VERSION;
    }

    void Engine::setWindowTitle(std::string title) {
        if (!_window) {
            return;
        }
        _window->setTitle(title);
    }

    std::string Engine::getWindowTitle() const {
        if (!_window) {
            return "";
        }
        return _window->getTitle();
    }

    void Engine::setWindowIcon(std::string icon) {
        if (!_window) {
            return;
        }
        _windowIconPath = icon;
        _window->setIcon(icon);
    }

    std::string Engine::getWindowIcon() const {
        return _windowIconPath;
    }

    void Engine::setWindowIcon(int width, int height, const unsigned char* data) {
        if (!_window) {
            return;
        }
        _windowIconPath = ""; // Clear path when setting from raw data
        _window->setIcon(static_cast<unsigned int>(width), static_cast<unsigned int>(height), data);
    }

    void Engine::setWindowPosition(int x, int y) {
        if (!_window) {
            return;
        }
        _window->setPosition(sf::Vector2i(x, y));
    }

    void Engine::setWindowVerticalSyncEnabled(bool enabled) {
        if (!_window) {
            return;
        }
        _window->setVerticalSyncEnabled(enabled);
    }

    bool Engine::isWindowVerticalSyncEnabled() const {
        if (!_window) {
            return false;
        }
        return _window->isVerticalSyncEnabled();
    }

    void Engine::setWindowFramerateLimit(unsigned int limit) {
        if (!_window) {
            return;
        }
        _window->setFramerateLimit(limit);
    }

    unsigned int Engine::getWindowFramerateLimit() const {
        if (!_window) {
            return 0;
        }
        return _window->getFramerateLimit();
    }

    void Engine::setWindowMouseCursorVisible(bool visible) {
        if (!_window) {
            return;
        }
        _window->setMouseCursorVisible(visible);
    }

    bool Engine::isWindowMouseCursorVisible() const {
        if (!_window) {
            return true;
        }
        return _window->isMouseCursorVisible();
    }

    void Engine::setWindowMouseCursorGrabbed(bool grabbed) {
        if (!_window) {
            return;
        }
        _window->setMouseCursorGrabbed(grabbed);
    }

    bool Engine::isWindowMouseCursorGrabbed() const {
        if (!_window) {
            return false;
        }
        return _window->isMouseCursorGrabbed();
    }

    void Engine::setWindowSize(int width, int height) {
        if (!_window) {
            return;
        }
        _window->setSize(sf::Vector2u(static_cast<unsigned int>(width), static_cast<unsigned int>(height)));
    }

    void Engine::setWindowVisible(bool visible) {
        _windowVisible = visible;
        if (_window) {
            _window->setVisible(visible);
        }
    }

    bool Engine::isWindowVisible() const {
        if (_window) {
            return _window->isVisible();
        }
        return _windowVisible;
    }
} // namespace pyg
