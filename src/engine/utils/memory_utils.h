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
	return ((bool)std::get<I>(t).get() && ...);
}
template <typename... Ts>
bool is_not_nulls(std::tuple<Ts...> &t)
{
	return check_null_helper(t, std::make_index_sequence<sizeof...(Ts)>());
}

template <typename T>
class guard;

template <typename T>
class guard<T *>
{

private:
	T *val;
	std::unique_lock<std::shared_mutex> g;
	template <typename A>
	friend class locker;

public:
	inline guard(const std::shared_mutex &m, T *v) : g(m), val(v)
	{
	}
	inline guard( std::shared_mutex &m, T *v) : g(m), val(v)
	{
	}
	inline T *get()
	{
		return val;
	}
	T *operator->()
	{
		return val;
	}
	auto &operator*()
	{
		return *val;
	}
};

template <typename T>
class guard<const T *>
{

private:
	const T *val;
	std::shared_lock<std::shared_mutex> g;
	template <typename A>
	friend class locker;

public:
	inline guard(const std::shared_mutex &m, T *v) : g(m), val(v)
	{
	}

	inline guard(std::shared_mutex &m, T *v) : g(m), val(v)
	{
	}
	inline const T *get()
	{
		return val;
	}
	const T *operator->()
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
	inline locker(T data) : value(std::move(data))
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
	template<typename New = T>

	inline guard<const New *> read() const
	{
		return guard<const New *>(mutex, &this->value);
	}

	template<typename New = T>
	inline guard<const New *> read()
	{
		return guard<const New *>(mutex,&this->value);
	}
	template<typename New = T>

	inline guard<New *> write()
	{
		return guard<New *>(mutex, &this->value);
	}
	T value;
	mutable std::shared_mutex mutex;
};


#endif // !MATRIXENGINE_MEMORY_UTILS