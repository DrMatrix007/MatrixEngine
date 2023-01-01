#include "CameraComponent.h"

me::CameraComponent::CameraComponent(float s) : _size(s)
{}

const float& me::CameraComponent::getSize() const 
{
	return _size;
}

const bool& me::CameraComponent::getIsMain() const
{
	return _isMain;
}

void me::CameraComponent::setIsMain(bool v)
{
	_isMain = v;
}
