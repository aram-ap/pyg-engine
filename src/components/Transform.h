//
// Created by Aram Aprahamian on 11/22/25.
//
#pragma once
#include "../core/Component.h"
#include "../math/Math.h"

#ifndef TRANSFORM_H
#define TRANSFORM_H

namespace pyg {

class Transform final : Component{
public:
    Transform(long id, const std::string &name);
    virtual ~Transform();
    virtual void setPosition(const Vector3& position);
    virtual Vector3 getPosition() const;
    virtual void setRotation(const Vector3& rotation);
    virtual Vector3 getRotation() const;
    virtual void setScale(const Vector3& scale);
    virtual Vector3 getScale() const;
    virtual void setOrigin(const Vector3& origin);
    virtual Vector3 getOrigin() const;



private:

};

} // pyg

#endif //TRANSFORM_H
