#include "TransformComponent.h"

void me::TransformComponent::setLayer(const size_t& l)
{
	_layer = l;
}

const size_t& me::TransformComponent::getLayer() const
{
	return _layer;
}
