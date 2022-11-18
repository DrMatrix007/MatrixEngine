#include "ThreadPool.h"

me::ThreadPool::ThreadPool(std::thread a)
{
	_threads.push_back(std::move(a));
}

me::ThreadPool::ThreadPool(std::vector<std::thread>& v) : _threads(std::move(v))
{}

void me::ThreadPool::pushThread(std::thread t)
{
	_threads.push_back(std::move(t));
}

void me::ThreadPool::push(std::function<void()> t)
{
	pushThread(std::thread(std::move(t)));
}

void me::ThreadPool::join()
{
	for (auto& t : _threads)
	{
		if (t.joinable())
		{
			t.join();
		}
	}
}

std::vector<std::thread>& me::ThreadPool::getVec()
{
	return _threads;
}
