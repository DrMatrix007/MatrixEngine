#include "RendererSystem.h"

#include "../Application.h"
#include "CameraComponent.h"

using namespace me;

me::RendererSystem::RendererSystem(unsigned int width, unsigned int height, const std::string& name): _window({width,height},name)
{}

inline Registry::QueryResult<TransformComponent*, RendererComponent*> me::RendererSystem::getQuery(Registry& reg)
{
	auto ans = System<TransformComponent, RendererComponent>::getQuery(reg);
	ans.orderBy([](std::tuple<Entity, TransformComponent*, RendererComponent*> a,std::tuple<Entity, TransformComponent*, RendererComponent*>& b)
	{
		return std::get<1>(a)->getLayer() < std::get<1>(b)->getLayer();
	});
	return ans;
}

void me::RendererSystem::onUpdate(SystemArgs& args, Entity, TransformComponent& t, RendererComponent& r)
{
	r.draw(this->_window, sf::RenderStates{t.getTransform()});
}

void me::RendererSystem::onLateUpdate(SystemArgs& args)
{
	searchCamera(args);

	me::System<TransformComponent, RendererComponent>::onLateUpdate(args);

	sf::Event e;
	while (_window.pollEvent(e))
	{
		if (e.type == sf::Event::Closed)
		{
			(args.getApplication())->stop();
		}
	}

	_window.display();
}

void me::RendererSystem::searchCamera(SystemArgs& args)
{

	auto ans = args.getRegistry().query<CameraComponent>();
	

}

