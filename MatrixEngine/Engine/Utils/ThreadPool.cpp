#include "ThreadPool.h"

void me::ThreadPool::pushThread(std::thread t)
{
	_threads.emplace_back(std::move(t));
}

void me::ThreadPool::push(std::function<void()> t)
{
	_threads.emplace_back(std::move(t));
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

void me::ThreadPool::clear()
{
	_threads.clear();
}

std::vector<std::thread>& me::ThreadPool::getVec()
{
	return _threads;
}
