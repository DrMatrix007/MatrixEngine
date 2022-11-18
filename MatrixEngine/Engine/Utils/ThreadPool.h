#ifndef MATRIX_ENGINE_THREAD_POOL
#define MATRIX_ENGINE_THREAD_POOL

#include <vector>
#include <thread>
#include <functional>

namespace me
{


	class ThreadPool
	{
	public:
		ThreadPool() = default;
		ThreadPool(std::thread a);
		ThreadPool(std::vector<std::thread>& v);
		void pushThread(std::thread t);
		void push(std::function<void()> t);
		void join();
		std::vector<std::thread>& getVec();
	private:
		std::vector<std::thread> _threads;
	};


}

#endif // !MATRIX_ENGINE_THREAD_POOL
