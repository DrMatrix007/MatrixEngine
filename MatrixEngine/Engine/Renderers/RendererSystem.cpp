#include "RendererSystem.h"

#include "../Application.h"
#include "CameraComponent.h"

using namespace me;


void me::RendererSystem::onUpdate(SystemArgs& args)
{
	auto _window = args.getRegistry().getResource<sf::RenderWindow>();
	if (_window)
	{
		auto& window = *_window;
		args.getRegistry().query<RendererComponent, TransformComponent>().forEach([&window](const Entity, RendererComponent& r, TransformComponent& t)
		{
			r.draw(window, sf::RenderStates{ t.getTransform() });

		});
	}

}

void me::RendererSystem::onLateUpdate(SystemArgs& args)
{
	auto window = args.getRegistry().getResource<sf::RenderWindow>();
	if (!window)
	{
		return;
	}
	auto& _window = *window;
	searchCamera(args);


	me::System::onLateUpdate(args);

	sf::Event e;
	while (_window.pollEvent(e))
	{
		if (e.type == sf::Event::Closed)
		{
			(args.getApplication())->stop();
		}
	}

	_window.display();
	_window.clear(sf::Color{ 0x69696969 });
}

void me::RendererSystem::searchCamera(SystemArgs& args)
{
	auto window = args.getRegistry().getResource<sf::RenderWindow>();
	if (!window)
	{
		return;
	}
	auto& _window = *window;
	auto cam = args.getRegistry().get<CameraComponent>(_cameraEntity);
	auto trans = args.getRegistry().get<TransformComponent>(_cameraEntity);
	if (!cam)
	{
		auto ans = args.getRegistry().query<CameraComponent>();
		for (auto& c : ans)
		{
			if (std::get<1>(c)->getIsMain())
			{
				_cameraEntity = std::get<0>(c);
			}
		}
	}
	else
	{

		updateView(_window ,*cam, trans ? *trans : TransformComponent::ZERO);
	}

}

void me::RendererSystem::updateView(sf::RenderWindow& win, const CameraComponent& c, const TransformComponent& t)
{
	sf::Vector2f size = (sf::Vector2f)win.getSize();
	_currentView.setCenter(t.getPosition());
	_currentView.setSize(size * sqrtf(c.getSize() / (size.x * size.y)));

	win.setView(_currentView);
}

