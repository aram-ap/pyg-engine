#pragma once
#include "core/Engine.h"
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
          _window(new Window()),
          _ownsWindow(true),
          _isPaused(false),
          _isRunning(false) {

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
        if (!_window) {
            _window = new Window();
            _ownsWindow = true;
        }

        PYG_INFO("Engine starting");
        _isRunning = true;

        while (_isRunning && _window->isOpen()) {
            sf::Time frameTime = _clock.restart();
            update(frameTime);
            render();
        }

        PYG_INFO("Engine stopped");
        on_destroy();
    }

    /* @param deltaTime: The time elapsed since the last frame
     *
     */
    void Engine::update(sf::Time deltaTime) {
        if (_isPaused || !_window) {
            return;
        }

        if (tickRate > 0) {
            const float targetSeconds = 1.0f / static_cast<float>(tickRate);
            if (deltaTime.asSeconds() < targetSeconds) {
                sf::sleep(sf::seconds(targetSeconds - deltaTime.asSeconds()));
                deltaTime = _clock.restart();
            }
        }

        _window->pollEvents();
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
} // namespace pyg
