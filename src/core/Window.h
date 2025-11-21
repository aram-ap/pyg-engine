#pragma once
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Window/Event.hpp>
#include <SFML/Window/Mouse.hpp>
#include <string>

namespace pyg {

class Window {
public:
    Window();
    virtual ~Window();

    virtual void create(const sf::VideoMode& mode = sf::VideoMode(800, 600),
                        const std::string& title = "Pyg-Engine");
    virtual bool isOpen() const;
    virtual void pollEvents();
    virtual void display();
    virtual void clear(const sf::Color& color = sf::Color::Black);
    virtual void setTitle(const std::string& title);
    virtual std::string getTitle() const;
    virtual void setIcon(const std::string& iconPath);
    virtual void setSize(const sf::Vector2u& size);
    virtual sf::Vector2u getSize() const;
    virtual void setPosition(const sf::Vector2i& position);
    virtual sf::Vector2i getPosition() const;
    virtual void setVisible(bool visible);
    virtual bool isVisible() const;
    virtual void setFramerateLimit(unsigned int limit);
    virtual unsigned int getFramerateLimit() const;
    virtual void setVerticalSyncEnabled(bool enabled);
    virtual bool isVerticalSyncEnabled() const;
    virtual void setMouseCursorVisible(bool visible);
    virtual bool isMouseCursorVisible() const;
    virtual void setMouseCursorGrabbed(bool grabbed);
    virtual bool isMouseCursorGrabbed() const;
    virtual void setMouseCursorPosition(const sf::Vector2i& position);
    virtual sf::Vector2i getMouseCursorPosition() const;
    virtual void close();
    virtual void destroy();

private:
    sf::RenderWindow _window;
    std::string _title;
    bool _isVisible;
    unsigned int _framerateLimit;
    bool _vsyncEnabled;
    bool _cursorVisible;
    bool _cursorGrabbed;
};

} // namespace pyg
