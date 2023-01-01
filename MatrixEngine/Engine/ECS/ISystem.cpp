#include "ISystem.h"
#include "Registry.h"
void me::ISystem::update(SystemArgs& args)
{
	onUpdate(args);
}

me::SystemArgs::SystemArgs(Registry* r,Application* a) : _reg(r),_app(a)
{}

me::Registry& me::SystemArgs::getRegistry() const
{

	return *_reg;
}

me::Application* const me::SystemArgs::getApplication()
{
	return _app;
}


void me::ISystem::lateUpdate(SystemArgs& args)
{
	onLateUpdate(args);
}
