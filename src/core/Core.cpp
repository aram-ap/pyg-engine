#include "core/Core.h"

namespace pyg {

const std::string Core::VERSION = "0.1.0";

Core::Core() {
    // SFML is linked, we can use it if we want, e.g. checking system time
    sf::Time t = sf::seconds(1.0f);
}

void Core::update(float deltaTime) {
    // Update logic here
}

void Core::render() {
    // Render logic here
}

void Core::on_destroy() {
    // Cleanup logic here
}

std::string Core::getVersion() const {
    return VERSION;
}

} // namespace pyg

