#include "RendererSystem.h"
#include "../Application.h"

using namespace me;

me::RendererSystem::RendererSystem(unsigned int width, unsigned int height, const std::string& name): _window({width,height},name)
{}

inline Registry::QueryResult<UniqueLocker<TransformComponent>*, UniqueLocker<RendererComponent>*> me::RendererSystem::getQuery(Registry& reg)
{
	auto ans = System<Read<TransformComponent>, Read<RendererComponent>>::getQuery(reg);
	ans.orderBy([](std::tuple< UniqueLocker<TransformComponent>*, UniqueLocker<RendererComponent>*> a,std::tuple< UniqueLocker<TransformComponent>*, UniqueLocker<RendererComponent>*> b)
	{
		return std::get<0>(a)->read()->getLayer() < std::get<0>(b)->read()->getLayer();
	});
	return ans;
}

void me::RendererSystem::onUpdate(SystemArgs& args, ReadGuard<TransformComponent> t, ReadGuard<RendererComponent> r)
{
	r->draw(this->_window, sf::RenderStates{t->getTransform()});
}

void me::RendererSystem::onLateUpdate(SystemArgs& args)
{
	me::System<Read<TransformComponent>, Read<RendererComponent>>::onLateUpdate(args);

	sf::Event e;
	while (_window.pollEvent(e))
	{
		if (e.type == sf::Event::Closed)
		{
			(*args.getApplication())->stop();
		}
	}

	_window.display();
}

