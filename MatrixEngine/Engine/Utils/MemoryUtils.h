#ifndef MATRIX_ENGINE_MEM_UTILS
#define MATRIX_ENGINE_MEM_UTILS
#include <memory>
#include <shared_mutex>
#include "Locker.h"

namespace me
{
	//struct NoDelete
	//{
	//	template <typename T>
	//	inline void operator() (T const&) const noexcept {}
	//};
	////TODO: read how std unque ptr is moved.

	//template<typename T>
	//using ReadGuard = Guard<const T*>;

	//template<typename T>
	//using WriteGuard = Guard<T*>;

	//template<typename T, typename Old>
	//Guard<T> castGuard(Old&& guard)
	//{
	//	return { dynamic_cast<T>(guard._ref), guard._mutex };
	//}

	//extern me::Locker<std::ostream*,NoDelete> cout;

	//template<typename T, typename Deleter = std::default_delete<T>>
	//using UniqueLocker = Locker<T, Deleter>;



	template<typename T>
	inline bool checkNotNulls(T* _data)
	{
		return (bool)_data;
	}
	//template<typename T>
	//inline bool checkNotNulls(const Guard<T>& _data)
	//{
	//	return (bool)_data.getPointer();
	//}



	template<size_t I, typename...Ts>
	inline typename std::enable_if <I >= sizeof...(Ts), bool>::type checkNotNullTuple(const std::tuple<Ts...>& data)
	{
		return true;
	}

	template<size_t I = 0, typename...Ts>
	inline typename std::enable_if < I<sizeof...(Ts), bool>::type checkNotNullTuple(const std::tuple<Ts...>& data)
	{
		return checkNotNulls(std::get<I>(data)) && checkNotNullTuple<I + 1, Ts...>(data);
	}


	template<typename T>
	inline bool checkNotNulls(T a)
	{
		return (bool)a;
	}

	template<typename T, typename... Ts>
	inline bool checkNotNulls(T a, Ts... _data)
	{
		return a && checkNotNulls(_data...);
	}


	template<typename T>
	inline T deref(T t)
	{
		return t;
	}
	template<typename T>
	inline T& deref(T* t)
	{
		return *t;
	}

	template<typename ...T>
	inline auto deref(std::tuple<T...>& t)
	{
		return derefImpl(t, std::make_index_sequence<sizeof...(T)>{});
	}
	template<typename ...T,size_t... Is>
	inline auto derefImpl(std::tuple<T...>& t,std::index_sequence<Is...>)
	{
		return std::make_tuple(deref(std::get<Is>(t))...);
	}



}

#endif // !MATRIX_ENGINE_MEM_UTILS
