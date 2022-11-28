#include "CameraComponent.h"

me::CameraComponent::CameraComponent(float s) : _size(s)
{}

const float& me::CameraComponent::getSize() const 
{
	return _size;
}
