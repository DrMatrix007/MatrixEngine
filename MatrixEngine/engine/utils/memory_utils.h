#ifndef MATRIXENGINE_MEMORY_UTILS
#define MATRIXENGINE_MEMORY_UTILS
#include<stdio.h>

#include <mutex>

#define _CRTDBG_MAP_ALLOC
#include <stdlib.h>
#include <crtdbg.h>

inline void show_memory_leaks()
{
	printf("Memory leaks: %d\n", _CrtCheckMemory());
}



template<typename T>
class locker
{
public:
	inline locker()
	{}
	inline locker(T data)
	{
		value = data;
	}
	/*locker(const locker&& other) : value(other.value), r(other.r.get())
	{

	};*/


	inline T& lock()
	{
		r.lock();
		return value;
	}
	inline void unlock()
	{
		r.unlock();
	}
	inline std::tuple<std::lock_guard<std::mutex>, T&> guard()
	{
		
		return std::make_tuple<std::lock_guard<std::mutex>, T&>(std::lock_guard(r), this->r);
	}

private:
	T value;
	std::mutex r;
};


#endif // !MATRIXENGINE_MEMORY_UTILS