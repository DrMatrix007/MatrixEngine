#ifndef MATRIX_ENGINE_RENDERER_SYSTEM
#define MATRIX_ENGINE_RENDERER_SYSTEM

#include "SFML/Window.hpp"
#include "SFML/Graphics.hpp"

#include "RendererComponent.h"

#include "../ECS/ECS.h"

namespace me
{
	class RendererSystem : public me::System<Read<TransformComponent>, Read<RendererComponent>>
	{
	public:
		RendererSystem(unsigned int, unsigned int,const std::string&);


	private:
		sf::RenderWindow _window;

		virtual inline Registry::QueryResult<UniqueLocker<TransformComponent>*, UniqueLocker<RendererComponent>*> getQuery(Registry& reg);
		virtual void onUpdate(SystemArgs&,ReadGuard<TransformComponent> t, ReadGuard<RendererComponent> r);
		virtual void onLateUpdate(SystemArgs&) override;
	};
}


#endif // !MATRIX_ENGINE_RENDERER_SYSTEM