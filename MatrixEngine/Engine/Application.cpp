#include "Application.h"

me::Registry& me::Application::getRegistry()
{
	return _reg;
}

void me::Application::stop()
{
	_running = false;
}

void me::Application::run()
{
	while (_running)
	{
		_reg.update(this);
	}

}
