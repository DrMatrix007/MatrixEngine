#include "engine/Engine.h"
#include <iostream>
#include <chrono>
#include <thread>
#include <memory>

#define RAND_BOOL (rand()%2)

std::unique_ptr<me::Application> createMainApp()
{
	srand((unsigned int)time(nullptr));

	using namespace me;


	me::UniqueLocker<int> a;
	Registry reg;

	for (size_t i = 0; i < 5; i++)
	{
		Entity e;
		reg.set(e, 10);
		reg.set(e, 10.f);
	}
	a.write();
	reg.query<Read<float>, Write<int>>([](ReadGuard<float>& a, WriteGuard<int>& b)
	{
		std::cout << "nice.\n";
	}).async_thread().join();

	return nullptr;
}