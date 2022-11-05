#ifndef MATRIXENGINE_MEMORY_UTILS
#define MATRIXENGINE_MEMORY_UTILS

#include <shared_mutex>
#include <mutex>


// inline void show_memory_leaks()
// {
// 	printf("Memory leaks: %d\n", _CrtCheckMemory());
// }

template <typename T>
class locker
{
public:
	inline locker()
	{
	}
	inline locker(T data): value(data)
	{
	}
	/*locker(const locker&& other) : value(other.value), r(other.r.get())
	{

	};*/

	inline T &lock()
	{
		mutex.lock();
		return value;
	}
	inline void unlock()
	{
		mutex.unlock();
	}
	inline std::pair<std::shared_lock<std::shared_mutex>,const T&> read()
	{
		return std::pair<std::shared_lock<std::shared_mutex>, T &>(mutex, this->value);
	}
	inline std::pair<std::unique_lock<std::shared_mutex>, T &> write()
	{
		return std::pair<std::unique_lock<std::shared_mutex>, T &>(mutex, this->value);
	}
	T value;
	std::shared_mutex mutex;
};

#endif // !MATRIXENGINE_MEMORY_UTILS