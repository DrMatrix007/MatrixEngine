#ifndef MATRIX_ENGINE_FUCNTIONS_UTILS
#define MATRIX_ENGINE_FUCNTIONS_UTILS

#include <tuple>

namespace me
{
	////template<typename T, typename ...Args>
	////inline auto apply(const T& f, Args... args)
	////{
	////	return f(args...);
	////}
	//template<typename T, typename ...Args>
	//inline auto apply(const T& f, std::tuple<Args...>&& args)
	//{
	//	return applyHelper(f, std::move(args), std::make_index_sequence<sizeof...(Args)>{});
	//}

	////template<typename T, typename ...Args>
	////inline auto apply(const T& f, std::tuple<Args...>&& args)
	////{
	////	return applyHelper(f, std::move(args), std::make_index_sequence<sizeof...(Args)>{});
	////}
	//template<typename T, typename ...Args, size_t... Is>
	//inline auto applyHelper(const T& f,std::tuple<Args...>&& args, std::index_sequence<Is...>)
	//{
	//	return f(std::get<Is>(args)...);
	//}

	template<typename Function, typename Tuple, size_t ... I>
	auto applyImpl(Function f, Tuple& t, std::index_sequence<I ...>)
	{
		return f((std::get<I>(t))...);
	}

	template<typename Function, typename ...Ts>
	auto apply(Function f, std::tuple<Ts...>& t)
	{
		return applyImpl(f, t, std::make_index_sequence<sizeof...(Ts)>{});
	}


	/*template<typename F, typename...T>
	inline auto apply(const F& f, std::tuple<T...>)
	{
	
	}*/

}


#endif // !MATRIX_ENGINE_FUCNTIONS_UTILS
