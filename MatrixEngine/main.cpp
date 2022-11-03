#include <iostream>
#include "engine/matrix_engine.h"

using namespace me;


class MyApplication : public Application
{
public:
	ecs::registry reg;
};

class T : public ecs::component
{
	virtual void on_update(const ecs::component_args& args){};
	virtual void on_init() {};
	virtual void on_destroy() {};
};


std::unique_ptr<Application> create_main_app() {

	using namespace ecs;
	MyApplication* app = new MyApplication{};
	auto e = entity{};
	auto e1 = entity{};
	//auto v = me::ecs::component_vec{};
	//
	//v.set(T{}, e);
	auto& reg = app->reg;
	
	reg.set(e, T{});

	app->reg.operate<T>([](entity e, T* p)
	{
		
	});

	return std::unique_ptr<Application>(app);
}