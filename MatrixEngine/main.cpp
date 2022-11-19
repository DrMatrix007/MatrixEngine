#include "engine/Engine.h"
#include <iostream>
#include <chrono>
#include <thread>
#include <memory>

using me::Read;
using me::Write;


class MyApplication : public me::Application
{

};

class MySystem : public me::MultiThreadedAsyncSystem<Read<int>, Write<float>>
{
public:
	MySystem(int a) : c(a)
	{}
	int c = 0;
private:
	// Inherited via System
	virtual void onUpdate(const me::SystemArgs& args, me::ReadGuard<int> a, me::WriteGuard<float> b) override
	{
		*b *= *a;
		if (*b > 100)
		{
			auto cout = me::cout.write();
			**cout << *a << " " << *b << "  " << c << std::endl;
			args.getApplication().stop();
		}

	}
};

std::unique_ptr<me::Application> createMainApp()
{
	srand((unsigned int)time(nullptr));

	using namespace me;

	auto app = std::make_unique<MyApplication>();

	Registry& reg = app->getRegistry();

	reg.pushSystem(MySystem{ 1 });
	reg.pushSystem(MySystem{ 0 });

	for (int i = 0; i <= 5; i++)
	{
		Entity e;
		reg.set(e, 1.0f);
		reg.set(e, 5);
	}
	//[](ReadGuard<float>& a, WriteGuard<int>& b)
	//{
	//	auto cout = me::cout.write();
	//	**cout << *b << '\n';
	//}

	reg.query<Read<float>, Write<int>>();

	return app;
}