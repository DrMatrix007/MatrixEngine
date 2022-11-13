#ifndef MATRIX_ENGINE_FUCNTIONS_UTILS
#define MATRIX_ENGINE_FUCNTIONS_UTILS

#include <tuple>

namespace me
{
	//template<typename T, typename ...Args>
	//inline auto apply(const T& f, Args... args)
	//{
	//	return f(args...);
	//}
	template<typename T, typename ...Args>
	inline auto apply(const T& f, std::tuple<Args...>&  args)
	{
		return applyHelper(f, std::move(args), std::make_index_sequence<sizeof...(Args)>{});
	}
	template<typename T, typename ...Args, size_t... Is>
	inline auto applyHelper(const T& f, std::tuple<Args...>&& args, std::index_sequence<Is...>)
	{
		return f(std::get<Is>(args)...);
	}

}


#endif // !MATRIX_ENGINE_FUCNTIONS_UTILS
