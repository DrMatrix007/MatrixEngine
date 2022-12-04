#include "engine/Engine.h"

#include <iostream>
#include <chrono>
#include <thread>
#include <memory>



class PlayerComponent {};

class PlayerSystem : public me::System
{
	// Inherited via System
	virtual void onUpdate(me::SystemArgs& args) override
	{
		
	}
};

std::unique_ptr<me::Application> createMainApp()
{
	srand((unsigned int)time(nullptr));

	using namespace me;

	auto app = std::make_unique<me::Application>();

	Registry& reg = app->getRegistry();

	reg.setResource<sf::RenderWindow>(std::make_unique<sf::RenderWindow>(sf::VideoMode{ 800,600 }, "dsfsd"));

	reg.pushSystem(std::make_unique<RendererSystem>());


	Entity cam;

	reg.set(cam, CameraComponent{1000});
	reg.set(cam, PlayerComponent{});

	for (int i = 0; i <= 5; i++)
	{
		Entity e;
		reg.set(e, TransformComponent{});
		reg.set(e, RendererComponent{std::make_shared<sf::RectangleShape>(sf::Vector2f{10,10})});
	}
	//[](ReadGuard<float>& a, WriteGuard<int>& b)
	//{
	//	auto cout = me::cout.write();
	//	**cout << *b << '\n';
	//}


	return app;
}