#include "Registry.h"

void me::Registry::update(Application* a)
{
	SystemArgs args = {this,a};

	for (auto& i : _systems)
	{
		i->update(args);
	}

	for (auto& i : _systems)
	{
		i->lateUpdate(args);
	}
}
