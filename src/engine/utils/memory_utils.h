#ifndef MATRIXENGINE_MEMORY_UTILS
#define MATRIXENGINE_MEMORY_UTILS

#include <shared_mutex>
#include <mutex>
#include <tuple>

// inline void show_memory_leaks()
// {
// 	printf("Memory leaks: %d\n", _CrtCheckMemory());
// }

template <typename... T, size_t... I>
bool check_null_helper(std::tuple<T...> &t, std::index_sequence<I...>)
{
	return ((bool)std::get<I>(t) && ...);
}
template <typename... Ts>
bool is_not_nulls(std::tuple<Ts...> &t)
{
	return check_null_helper(t, std::make_index_sequence<sizeof...(Ts)>());
}

template <typename Guard, typename T>
class guard
{
private:
	T val;
	Guard g;
	inline guard(std::shared_mutex &m, T v) : g(m), val(v)
	{
	}
	template <typename A>
	friend class locker;

public:

	inline T get() {
		return val;
	}
	T operator->()
	{
		return val;
	}
	auto &operator*()
	{
		return *val;
	}
};

template <typename T>
class locker
{
public:
	inline locker()
	{
	}
	inline locker(T data) : value(data)
	{
	}

	// inline T &lock()
	// {
	// 	mutex.lock();
	// 	return value;
	// }
	// inline void unlock()
	// {
	// 	mutex.unlock();
	// }

	inline guard<std::shared_lock<std::shared_mutex>, const T*> read()
	{
		return guard<std::shared_lock<std::shared_mutex>, const T*>(mutex, &this->value);
	}
	inline guard<std::unique_lock<std::shared_mutex>, T*> write()
	{
		return guard<std::unique_lock<std::shared_mutex>, T*>(mutex, &this->value);
	}
	T value;
	std::shared_mutex mutex;
};
template <typename T,typename Other = T>
class locker_ref
{
public:
	inline locker_ref(locker<Other>& l): mutex(l.mutex),data(l.value) {

	}
	inline locker_ref(std::shared_mutex &m, T *d) : mutex(m), data(d)
	{
	}

	inline locker_ref(std::shared_mutex &m) : mutex(m)
	{
	}

	inline guard<std::shared_lock<std::shared_mutex>, const T *> read()
	{
		return guard<std::shared_lock<std::shared_mutex>, const T *>(mutex, this->value);
	}
	inline guard<std::unique_lock<std::shared_mutex>, T *> write()
	{
		return guard<std::unique_lock<std::shared_mutex>, T *>(mutex, this->value);
	}

private:
	T *data;
	std::shared_mutex &mutex;
};

#endif // !MATRIXENGINE_MEMORY_UTILS