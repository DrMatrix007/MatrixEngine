#ifndef MATRIX_ENGINE_QUERY
#define MATRIX_ENGINE_QUERY

#include <functional>
#include <shared_mutex>
#include <tuple>
#include <memory>

#include "../utils/utils.h"


namespace me::ecs {

    class component_vec;


    // template<typename ...T>
    // struct query_result {};


    // template<typename T1,typename ...T>
    // struct query_result<T1,T...> {
    //     using lock_type = infer_type<T1>::guard_type;
    //     using all = utils::connect_templates<std::tuple,std::tuple<query_vec_result<T1>>,typename query_result<T...>::all>::type;

    //     inline query_result(all d): data(d) {};

    //     all data;


    // };
    // template<>
    // struct query_result<> {
    //     using all = std::tuple<>;
    // };


}



#endif