//
// Created by Aram Aprahamian on 11/22/25.
//

#include "Transform.h"

namespace pyg {
    Transform::Transform(long id, const std::string &name): Component(id, name) {

    }

    Transform::~Transform() {

    }

    void Transform::setPosition(const Vector3& position) {

    }

    Vector3 Transform::getPosition() const {
        return Vector3();
    }

    void Transform::setRotation(const Vector3& rotation) {

    }

    Vector3 Transform::getRotation() const {
        return Vector3();
    }

    void Transform::setScale(const Vector3& scale) {

    }

    Vector3 Transform::getScale() const {
        return Vector3();
    }

    void Transform::setOrigin(const Vector3& origin) {

    }

    Vector3 Transform::getOrigin() const {
        return Vector3();
    }
    void Transform::setSize(const Vector3& size) {

    }

    Vector3 Transform::getSize() const {
        return Vector3();
    }

    void Transform::setOffset(const Vector3& offset) {

    }

    Vector3 Transform::getOffset() const {
        return Vector3();
    }
    bool Transform::isFlipped() const {
        return false;
    }

    void Transform::setFlipped(bool flipped) {

    }

    bool Transform::isCentered() const {
        return false;
    }

    void Transform::setCentered(bool centered) {

    }

    bool Transform::isAnchored() const {
        return false;
    }

    void Transform::setAnchored(bool anchored) {

    }

    bool Transform::isVisible() const {
        return false;
    }

    void Transform::setVisible(bool visible) {

    }

    bool Transform::isEnabled() const {
        return false;
    }

    void Transform::setEnabled(bool enabled) {

    }
    std::string Transform::getName() const {
        return "";

    }
} // pyg
