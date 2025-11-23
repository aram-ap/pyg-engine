#include "Window.h"
#include "logging/Logger.h"
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/Image.hpp>
#include <SFML/Window/Event.hpp>

namespace pyg {

Window::Window()
    : _title("Pyg-Engine"),
      _isVisible(true),
      _framerateLimit(0),
      _vsyncEnabled(false),
      _cursorVisible(true),
      _cursorGrabbed(false) {
    create();
}

Window::~Window() {
    destroy();
}

void Window::create(const sf::VideoMode& mode, const std::string& title) {
    _title = title;
    _window.create(mode, title);
    _window.setVisible(_isVisible);
    if (_framerateLimit > 0) {
        _window.setFramerateLimit(_framerateLimit);
    }
    _window.setVerticalSyncEnabled(_vsyncEnabled);
    _window.setMouseCursorVisible(_cursorVisible);
    _window.setMouseCursorGrabbed(_cursorGrabbed);
}

bool Window::isOpen() const {
    return _window.isOpen();
}

void Window::pollEvents() {
    sf::Event event {};
    while (_window.pollEvent(event)) {
        if (event.type == sf::Event::Closed) {
            _window.close();
        }
    }
}

void Window::display() {
    _window.display();
}

void Window::clear(const sf::Color& color) {
    _window.clear(color);
}

void Window::setTitle(const std::string& title) {
    _title = title;
    _window.setTitle(title);
}

std::string Window::getTitle() const {
    return _title;
}

void Window::setIcon(const std::string& iconPath) {
    sf::Image image;
    if (!image.loadFromFile(iconPath)) {
        PYG_WARN("Failed to load window icon from {}", iconPath);
        return;
    }
    _window.setIcon(image.getSize().x, image.getSize().y, image.getPixelsPtr());
}

void Window::setSize(const sf::Vector2u& size) {
    _window.setSize(size);
}

sf::Vector2u Window::getSize() const {
    return _window.getSize();
}

void Window::setPosition(const sf::Vector2i& position) {
    _window.setPosition(position);
}

sf::Vector2i Window::getPosition() const {
    return _window.getPosition();
}

void Window::setVisible(bool visible) {
    _isVisible = visible;
    _window.setVisible(visible);
}

bool Window::isVisible() const {
    return _isVisible;
}

void Window::setFramerateLimit(unsigned int limit) {
    _framerateLimit = limit;
    _window.setFramerateLimit(limit);
}

unsigned int Window::getFramerateLimit() const {
    return _framerateLimit;
}

void Window::setVerticalSyncEnabled(bool enabled) {
    _vsyncEnabled = enabled;
    _window.setVerticalSyncEnabled(enabled);
}

bool Window::isVerticalSyncEnabled() const {
    return _vsyncEnabled;
}

void Window::setMouseCursorVisible(bool visible) {
    _cursorVisible = visible;
    _window.setMouseCursorVisible(visible);
}

bool Window::isMouseCursorVisible() const {
    return _cursorVisible;
}

void Window::setMouseCursorGrabbed(bool grabbed) {
    _cursorGrabbed = grabbed;
    _window.setMouseCursorGrabbed(grabbed);
}

bool Window::isMouseCursorGrabbed() const {
    return _cursorGrabbed;
}

void Window::setMouseCursorPosition(const sf::Vector2i& position) {
    sf::Mouse::setPosition(position, _window);
}

sf::Vector2i Window::getMouseCursorPosition() const {
    return sf::Mouse::getPosition(_window);
}

void Window::close() {
    _window.close();
}

void Window::destroy() {
    if (_window.isOpen()) {
        _window.close();
    }
}

} // namespace pyg
