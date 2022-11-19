#ifndef MATRIX_ENGINE_MEM_UTILS
#define MATRIX_ENGINE_MEM_UTILS
#include <memory>
#include <shared_mutex>
#include "Locker.h"

namespace me
{
	struct NoDelete
	{
		template <typename T>
		inline void operator() (T const&) const noexcept {}
	};
	//TODO: read how std unque ptr is moved.

	template<typename T>
	using ReadGuard = Guard<const T*>;

	template<typename T>
	using WriteGuard = Guard<T*>;

	template<typename T, typename Old>
	Guard<T> castGuard(Old&& guard)
	{
		return { dynamic_cast<T>(guard._ref), guard._mutex };
	}

	extern me::Locker<std::ostream*,NoDelete> cout;

	template<typename T, typename Deleter = std::default_delete<T>>
	using UniqueLocker = Locker<T, Deleter>;



	template<typename T>
	inline bool checkNotNulls(T* _data)
	{
		return (bool)_data;
	}
	template<typename T>
	inline bool checkNotNulls(const Guard<T>& _data)
	{
		return (bool)_data.getPointer();
	}



	template<typename T, typename... Ts>
	inline bool checkNotNulls(T a, Ts... _data)
	{
		return checkNotNulls(a) && checkNotNulls(_data...);
	}
	template<size_t I=0,typename...Ts>
	inline typename std::enable_if < I<sizeof...(Ts), bool>::type checkNotNulls(const std::tuple<Ts...>& data)
	{
		return checkNotNulls(std::get<I>(data)) && checkNotNulls<I+1,Ts...>(data);
	}
	template<size_t I,typename...Ts>
	inline typename std::enable_if < I==sizeof...(Ts), bool>::type checkNotNulls(const std::tuple<Ts...>& data)
	{
		return true;
	}
	
}

#endif // !MATRIX_ENGINE_MEM_UTILS
