#ifndef MATRIX_ENGINE_RENDERER_SYSTEM
#define MATRIX_ENGINE_RENDERER_SYSTEM

#include "SFML/Window.hpp"
#include "SFML/Graphics.hpp"

#include "RendererComponent.h"
#include "CameraComponent.h"

#include "../ECS/ECS.h"

namespace me
{
	class RendererSystem : public me::System
	{
	public:
		RendererSystem() = default;

	private:

		me::Entity _cameraEntity;
		sf::View _currentView;
		virtual void onUpdate(SystemArgs&);
		virtual void onLateUpdate(SystemArgs&) override;

		void searchCamera(SystemArgs&);
		void updateView(sf::RenderWindow&, const CameraComponent&, const TransformComponent&);
	};
}


#endif // !MATRIX_ENGINE_RENDERER_SYSTEM