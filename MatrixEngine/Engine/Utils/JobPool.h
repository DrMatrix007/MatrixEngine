#ifndef MATRIX_ENGINE_JOBPOOL_POOL
#define MATRIX_ENGINE_JOBPOOL_POOL

#include <vector>
#include <thread>
#include <functional>

#include "ThreadPool.h"

namespace me
{

	class IJobPool
	{
	public:


		virtual ThreadPool async_threads() abstract;
		virtual ThreadPool async_thread() abstract;
		virtual void sync() abstract;
	};


}

#endif // !MATRIX_ENGINE_THREAD_POOL
