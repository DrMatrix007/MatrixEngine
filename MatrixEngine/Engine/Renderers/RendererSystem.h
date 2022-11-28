#ifndef MATRIX_ENGINE_RENDERER_SYSTEM
#define MATRIX_ENGINE_RENDERER_SYSTEM

#include "SFML/Window.hpp"
#include "SFML/Graphics.hpp"

#include "RendererComponent.h"

#include "../ECS/ECS.h"

namespace me
{
	class RendererSystem : public me::System<TransformComponent, RendererComponent>
	{
	public:
		RendererSystem(unsigned int, unsigned int,const std::string&);

	private:
		sf::RenderWindow _window;

		me::Entity _cameraEntity;

		virtual inline Registry::QueryResult<TransformComponent*, RendererComponent*> getQuery(Registry& reg);
		virtual void onUpdate(SystemArgs&,const Entity, TransformComponent& t, RendererComponent& r);
		virtual void onLateUpdate(SystemArgs&) override;

		void searchCamera(SystemArgs&);
	};
}


#endif // !MATRIX_ENGINE_RENDERER_SYSTEM