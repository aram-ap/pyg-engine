#pragma once
#include <SFML/System.hpp>
#include "logging/Logger.h"
#include <string>
#include "GameObject.h"
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
    virtual void fixedUpdate(sf::Time deltaTime);
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

    // Game Object Items
    virtual void addGameObject(GameObject* gameObject);
    virtual GameObject* searchGameObjectByName(std::string name);
    virtual void removeGameObject(GameObject* gameObject);
    virtual void removeAllGameObjects();


    // Window things

    virtual void setWindowTitle(std::string title);
    virtual std::string getWindowTitle() const;
    virtual void setWindowIcon(std::string icon);
    virtual std::string getWindowIcon() const;
    virtual void setWindowIcon(int width, int height, const unsigned char* data);
    virtual void setWindowPosition(int x, int y);
    // virtual void setWindowStyle(Window::Style style);
    // virtual Window::Style getWindowStyle() const;
    virtual void setWindowVerticalSyncEnabled(bool enabled);
    virtual bool isWindowVerticalSyncEnabled() const;
    virtual void setWindowFramerateLimit(unsigned int limit);
    virtual unsigned int getWindowFramerateLimit() const;
    virtual void setWindowMouseCursorVisible(bool visible);
    virtual bool isWindowMouseCursorVisible() const;
    virtual void setWindowMouseCursorGrabbed(bool grabbed);
    virtual bool isWindowMouseCursorGrabbed() const;
    virtual void setWindowSize(int width, int height);
    virtual void setWindowVisible(bool visible);
    virtual bool isWindowVisible() const;
    virtual void setWindow(Window* window);
    virtual Window* getWindow() const;
    /**
     * @return The total engine runtime
     */
    virtual sf::Time getElapsedTime() const;
    virtual void exit();

private:
    int tickRate;
    Window* _window;
    bool _ownsWindow;
    bool _windowVisible;
    sf::Clock _clock;       //
    sf::Clock _systemClock;     // Basically don't restart this unless the engine is fully restarted
    bool _isPaused;
    bool _isRunning;
    std::string _windowIconPath;
    std::vector<pyg::GameObject*> _gameObjects;
};


} // namespace pyg

