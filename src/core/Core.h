#pragma once
#include <SFML/System.hpp>
#include <string>

namespace pyg {

class Core {
public:
    static const std::string VERSION;

    Core();
    virtual ~Core() = default;

    virtual std::string getVersion() const;
    virtual void update(float deltaTime);
    virtual void render();
    virtual void on_destroy();
};

} // namespace pyg

