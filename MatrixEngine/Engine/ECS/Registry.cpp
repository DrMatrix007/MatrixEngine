#include "Registry.h"

void me::Registry::update(Application* a)
{
	SystemArgs args = {this,a};

	for (auto& i : _systems)
	{
		auto s = i->write();
		(*s).update(args);
	}

	for (auto& i : _systems)
	{
		auto s = i->write();
		(*s).lateUpdate(args);
	}
}
