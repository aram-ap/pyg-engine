//
// Created by Aram Aprahamian on 11/22/25.
//
#pragma once
#include "../core/Component.h"

#ifndef TRANSFORM_H
#define TRANSFORM_H

namespace pyg {

class Transform final : Component{
public:
    Transform(long id, const std::string &name);
    virtual ~Transform();

private:

};

} // pyg

#endif //TRANSFORM_H
