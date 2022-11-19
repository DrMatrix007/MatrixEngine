#include "ISystem.h"
#include "Registry.h"
void me::ISystem::update(const SystemArgs& args)
{
	onUpdate(args);
}

me::SystemArgs::SystemArgs(Registry* r,Application* a) : _reg(r),_app(a)
{}

me::Registry& me::SystemArgs::getRegistry() const
{

	return *_reg;
}

me::Application& me::SystemArgs::getApplication() const
{
	return *_app;
}

void me::ISystem::lateUpdate(const SystemArgs& args)
{
	onLateUpdate(args);
}
