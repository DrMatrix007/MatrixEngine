#ifndef MATRIX_ENGINE_RENDERER_COMPONENT
#define MATRIX_ENGINE_RENDERER_COMPONENT

#include <memory>

#include "SFML/Graphics.hpp"

#include "../ECS/ECS.h"

namespace me
{
	class RendererComponent : public sf::Drawable
	{
	public:
		RendererComponent(std::shared_ptr<sf::Drawable> ptr);

		
		virtual void draw(sf::RenderTarget& target, sf::RenderStates states) const override;

	protected:
	private:
		std::shared_ptr<sf::Drawable> _drawable;


		// Inherited via Drawable

	};


};
#endif // !MATRIX_ENGINE_RENDERER_COMPONENT
