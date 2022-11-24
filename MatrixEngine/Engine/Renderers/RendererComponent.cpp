#include "RendererComponent.h"
#include "../Utils/Utils.h"

me::RendererComponent::RendererComponent(std::shared_ptr<sf::Drawable> ptr) : _drawable(std::move(ptr))
{}

void me::RendererComponent::draw(sf::RenderTarget & target, sf::RenderStates states) const
{
	auto cout = me::cout.write();
	**cout << "nice" << "\n";
	
	target.draw(*_drawable, states);
}

