#include "engine/Engine.h"

#include <iostream>
#include <chrono>
#include <thread>
#include <memory>



class MyApplication : public me::Application
{

};

class MySystem : public me::System<int, float>
{
public:

private:
	// Inherited via System
	virtual void onUpdate(me::SystemArgs& args,const me::Entity e, int& a, float& b) override
	{
		std::cout << a << " " << b << std::endl;
	}
};

std::unique_ptr<me::Application> createMainApp()
{
	srand((unsigned int)time(nullptr));

	using namespace me;

	auto app = std::make_unique<MyApplication>();

	Registry& reg = app->getRegistry();

	reg.pushSystem(std::make_unique<RendererSystem>(800,600,"test"));


	

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