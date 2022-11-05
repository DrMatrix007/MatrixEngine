#ifndef MATRIX_ENGINE_DUPLICATES_TEMPLATE
#define MATRIX_ENGINE_DUPLICATES_TEMPLATE

namespace me::utils
{
    template <typename T, typename T2>
    struct is_same
    {
        static constexpr bool value = false;
    };

    template <typename T>
    struct is_same<T, T>
    {
        static constexpr bool value = true;
    };

    template <typename... T>
    struct contains_dup_template
    {
    };

    template <typename T1, typename T2, typename... T>
    struct contains_dup_template<T1, T2, T...>
    {
        static constexpr bool contains = is_same<T1, T2>::value && contains_dup_template<T1, T...>::unique;
    };
    template <typename T1, typename T2>
    struct contains_dup_template<T1, T2>
    {
        static constexpr bool contains = is_same<T1, T2>::value;
    };

    template <typename T1>
    struct contains_dup_template<T1>
    {
        static constexpr bool contains = false;
    };


    template <typename... T>
    struct unique_template
    {
    };

    template <typename T1, typename... T>
    struct unique_template<T1, T...>
    {
        static constexpr bool unique = (!(contains_dup_template<T1, T...>::contains)) && unique_template<T...>::unique;
    };
    template<>
    struct unique_template<>
    {
        static constexpr bool unique = true;
    };

    template<template<typename...T> typename base,typename T1,typename T2>
    struct connect_templates {
        // using type = base<T...>;
    };
    template<template<typename...T> typename base,typename... T1,typename ...T2>
    struct connect_templates<base,base<T1...>,base<T2...>>{ 
        using type = base<T1...,T2...>;
    };

    template<typename ...T>
    class A{};
    template<typename ...T>
    using B = A<T...>;
    connect_templates<B,B<int>,B<float>>::type a;



}

#endif