#pragma once
#include <SFML/System.hpp>
#include "logging/Logger.h"
#include <string>
#include "../rendering/Window.h"

namespace pyg {

class Engine {
public:
    static const std::string VERSION;

    Engine();
    virtual ~Engine();

    virtual std::string getVersion() const;
    virtual int getTickRate() const;
    virtual void setTickRate(int tickRate);
    virtual void update(sf::Time deltaTime);
    virtual void render();
    virtual void on_destroy();
    virtual void logType(Logger::Type type, std::string msg);
    virtual void log(std::string msg);
    virtual bool isRunning() const;
    virtual void start();
    virtual void stop();
    virtual void pause();
    virtual void resume();
    virtual void restart();
    virtual void setWindowTitle(std::string title);
    virtual std::string getWindowTitle() const;
    virtual void setWindowIcon(std::string icon);
    virtual std::string getWindowIcon() const;
    virtual void setWindowIcon(int width, int height, const unsigned char* data);
    virtual void setWindowPosition(int x, int y);
    virtual void setWindowStyle(Window::Style style);
    virtual Window::Style getWindowStyle() const;
    virtual void setWindowVerticalSyncEnabled(bool enabled);
    virtual bool isWindowVerticalSyncEnabled() const;
    virtual void setWindowFramerateLimit(unsigned int limit);
    virtual unsigned int getWindowFramerateLimit() const;
    virtual void setWindowMouseCursorVisible(bool visible);
    virtual bool isWindowMouseCursorVisible() const;
    virtual void setWindowMouseCursorGrabbed(bool grabbed);
    virtual bool isWindowMouseCursorGrabbed() const;
    virtual void setWindowSize(int width, int height);
    virtual void setWindow(Window* window);
    virtual Window* getWindow() const;
    virtual void exit();

private:
    int tickRate;

    Window* _window;
    bool _ownsWindow;
    sf::Clock _clock;
    bool _isPaused;
    bool _isRunning;
};


} // namespace pyg

