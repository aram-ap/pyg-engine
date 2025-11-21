#include "core/Core.h"
#include "logging/Logger.h"
#include <SFML/System/Sleep.hpp>
#include <cstdlib>

namespace pyg {
    const std::string Core::VERSION = "0.1.0";

    Core::Core()
        : tickRate(60),
          _window(new Window()),
          _ownsWindow(true),
          _isPaused(false),
          _isRunning(false) {

        if (!Logger::getCoreLogger()) {
            Logger::init("pyg_engine");
        }

        PYG_INFO("Core initialized - Version: {}", VERSION);
        _clock.restart();
    }

    Core::~Core() {
        if (_ownsWindow) {
            delete _window;
        }
        _window = nullptr;
    }

    void Core::stop() {
        if (!_isRunning) {
            return;
        }
        PYG_INFO("Core stopping");
        _isRunning = false;
        if (_window) {
            _window->close();
        }
    }

    bool Core::isRunning() const {
        return _isRunning;
    }

    void Core::start() {
        if (!_window) {
            _window = new Window();
            _ownsWindow = true;
        }

        PYG_INFO("Core starting");
        _isRunning = true;

        while (_isRunning && _window->isOpen()) {
            sf::Time frameTime = _clock.restart();
            update(frameTime);
            render();
        }

        PYG_INFO("Core stopped");
        on_destroy();
    }

    void Core::update(sf::Time deltaTime) {
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

    void Core::render() {
        if (!_window) {
            return;
        }

        _window->clear();
        _window->display();
    }

    void Core::pause() {
        _isPaused = true;
    }

    void Core::resume() {
        _isPaused = false;
    }

    void Core::restart() {
        PYG_INFO("Restarting core");
        stop();
        start();
    }

    void Core::exit() {
        stop();
        std::exit(0);
    }

    void Core::setWindow(Window* window) {
        if (!window) {
            return;
        }

        if (_ownsWindow && _window) {
            delete _window;
        }

        _window = window;
        _ownsWindow = false;
    }

    Window* Core::getWindow() const {
        return _window;
    }

    void Core::on_destroy() {
        PYG_INFO("Core shutting down");
        Logger::shutdown();
    }

    void Core::log(std::string msg) {
        PYG_INFO(msg);
    }

    void Core::setTickRate(int tickRateValue) {
        tickRate = tickRateValue;
    }

    int Core::getTickRate() const {
        return tickRate;
    }

    void Core::logType(Logger::Type type, std::string msg) {
        Logger::print(type, msg);
    }

    std::string Core::getVersion() const {
        return VERSION;
    }
} // namespace pyg
