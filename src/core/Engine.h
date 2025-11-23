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
    virtual void exit();
    virtual void setWindow(Window* window);
    virtual Window* getWindow() const;

private:
    int tickRate;

    Window* _window;
    bool _ownsWindow;
    sf::Clock _clock;
    bool _isPaused;
    bool _isRunning;
};


} // namespace pyg

