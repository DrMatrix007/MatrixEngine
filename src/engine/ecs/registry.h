#ifndef MATRIX_ENGINE_REGISTRY
#define MATRIX_ENGINE_REGISTRY

#include <map>
#include <typeindex>
#include <memory>
#include <thread>
#include <mutex>
#include <functional>

#include "entity.h"
#include "component.h"
#include "system.h"
#include "query.h"
#include "../utils/utils.h"

namespace me::ecs
{

	class component_vec
	{
	private:
		std::map<entity, std::unique_ptr<locker<std::unique_ptr<component>>>> components;

	public:
		template <typename T>
		locker<std::unique_ptr<component>>* get(const entity &) const;
		template <typename T>
		locker<std::unique_ptr<component>>* get(const entity &);

		template <typename T>
		void set(std::unique_ptr<T>, const entity &);
		template <typename T>
		void set(T, const entity &);

		inline std::map<entity, std::unique_ptr<locker<std::unique_ptr<component>>>>::iterator begin()
		{
			return components.begin();
		}
		inline std::map<entity, std::unique_ptr<locker<std::unique_ptr<component>>>>::iterator end()
		{
			return components.end();
		}

		inline std::map<entity, std::unique_ptr<locker<std::unique_ptr<component>>>>::const_iterator begin() const
		{
			return components.cbegin();
		}
		inline std::map<entity, std::unique_ptr<locker<std::unique_ptr<component>>>>::const_iterator end() const
		{
			return components.cend();
		}
	};
	namespace queries
	{

		template <typename T>
		class query_vec_result
		{
		};

		template <template <typename> typename base, typename T>
		struct query_vec_result<base<T>>
		{
			using guard_type = base<T>::guard_type;

			inline query_vec_result(std::shared_mutex &g, component_vec &c) : l(g), v(c){};
			guard_type l;
			component_vec &v;
		};

		template <typename... T>
		struct function_tempalte
		{
			using type = std::function<void(T...)>;
			using parameters = std::tuple<T...>;
		};

		template <typename T>
		struct read
		{
			using param_type = const T *;
			using type = T;
			using guard_type = guard<param_type>;

			template <typename Val,typename Og>
			static inline auto get_guard(locker<Og> &l)
			{
				return l.template read<Val>();
			}
		};

		template <typename T>
		struct write
		{
			using param_type = T *;

			using type = T;
			using guard_type = guard<param_type>;

			

			template <typename Val,typename Og>
			static inline auto get_guard(locker<Og> &l)
			{
				return l.template write<Val>();
			}
		};

		template <typename... T>
		class query
		{
		};

		template <typename T>
		struct infer_type
		{
			using type = T::param_type;
			using guard_type = T::guard_type;
		};

		template <typename T1, typename... T>
		struct query<T1, T...>
		{
			using parameter_type = infer_type<T1>::guard_type;
			using all = utils::connect_templates<function_tempalte, function_tempalte<parameter_type>, typename query<T...>::all>::type;
			// using vecs_types = utils::connect_templates<std::tuple, std::tuple<std::unique_ptr<query_vec_result<T1>>>, typename query<T...>::vecs_types>::type;

			using func = all::type;

			func f;

			inline void run_query(all::parameters t)
			{
				_run_query(std::move(t), std::make_index_sequence<sizeof...(T) + 1>());
			}

			inline query(func fun) : f(fun)
			{
			}

		private:
			template <size_t... I>
			inline void _run_query(all::parameters&& ts, std::index_sequence<I...> is)
			{
				f(std::move(std::get<I>(ts))...);
			}
		};

		template <>
		struct query<>
		{
			using all = function_tempalte<>;
			using vecs_types = std::tuple<>;
		};

	}
	class registry
	{
	private:
		std::map<std::type_index, std::unique_ptr<locker<component_vec>>> vecs;
		std::map<std::type_index, std::unique_ptr<base_system>> systems;

	public:
		template <typename T, typename F>
		me::thread_pool write_components(const F &);

		template <typename T, typename F>
		me::thread_pool read_component(const F &);

		template <typename T, typename F>
		me::thread_pool write_components_async(const F &);

		template <typename T, typename F>
		me::thread_pool read_component_async(const F &);

		template <typename T>
		inline locker<component_vec> *get();

		template <typename T>
		locker<std::unique_ptr<component>>* get(const entity &);

		template <typename T>
		void set(const entity &, T);

		template <typename T1, typename... Ts>
		me::thread_pool query_sync(queries::query<T1, Ts...>::func f);

		template <typename T1, typename... Ts>
		me::thread_pool query_async(queries::query<T1, Ts...>::func f);
	};

	template <typename T>
	locker<std::unique_ptr<component>>* me::ecs::component_vec::get(const entity &e) const
	{
		auto i = components.find(e);
		if (i == components.end())
		{
			return nullptr;
		}
		else
		{
			return i->second.get();
		}
	}
	template <typename T>
	locker<std::unique_ptr< component>>* me::ecs::component_vec::get(const entity &e)
	{
		T *data;
		auto i = components.find(e);
		if (i == components.end())
		{
			return nullptr;
		}
		return i->second.get();
	}

	template <typename T>
	void me::ecs::component_vec::set(std::unique_ptr<T> t, const entity &e)
	{
		auto n = std::unique_ptr<component>(dynamic_cast<component *>(t.release()));
		if (n.get())
		{
			components.emplace(e, std::move(n));
		}
	}
	template <typename T>
	void me::ecs::component_vec::set(T t, const entity &e)
	{
		set(std::make_unique<T>(t), e);
	}

	template <typename T, typename F>
	inline me::thread_pool me::ecs::registry::write_components(const F &f)
	{

		return std::thread([this, &f]()
						   {
		T* ptr;
		locker<component_vec>* v = this->get<T>();
		if (v)
		{
			auto m = v->write();
			for (const auto& [e, c] : *m)
			{
				ptr = dynamic_cast<T*>(c.get());
				if (ptr)
				{
					f(e, ptr);
				}
			}

		} });
	}

	template <typename T, typename F>
	me::thread_pool me::ecs::registry::read_component(const F &f)
	{
		return std::thread([this, &f]()
						   {
		const T* ptr;
		locker<component_vec>* v = this->get<T>();
		if (v)
		{
			auto m = v->read();
			for (const auto& [e, c] : *m)
			{
				ptr = dynamic_cast<const T*>(c.get());
				if (ptr)
				{
					f(e, ptr);
				}
			}
		} });
	}

	template <typename T, typename F>
	inline me::thread_pool me::ecs::registry::write_components_async(const F &f)
	{
		std::vector<std::thread> threads;
		T *ptr;
		locker<component_vec> *v = this->get<T>();
		if (v)
		{
			auto m = v->read();
			for (const auto &[e, c] : *m)
			{
				ptr = dynamic_cast<T *>(c.get());
				if (ptr)
				{
					threads.push_back(std::thread(f, e, ptr));
				}
			}
		}
		return threads;
	}

	template <typename T, typename F>
	me::thread_pool me::ecs::registry::read_component_async(const F &f)
	{
		std::vector<std::thread> threads;
		const T *ptr;
		locker<component_vec> *v = this->get<T>();
		if (v)
		{
			auto m = v->read();
			for (const auto &[e, c] : *m)
			{
				ptr = dynamic_cast<const T *>(c.get());
				if (ptr)
				{
					f(e, ptr);
				}
			}
		}
		return threads;
	}

	template <typename T>
	inline locker<me::ecs::component_vec> *me::ecs::registry::get()
	{
		auto i = vecs.find(typeid(T));
		if (i == vecs.end())
		{
			return nullptr;
		}
		else
		{
			return i->second.get();
		}
	}
	template <typename T>
	locker<std::unique_ptr<component>>* me::ecs::registry::get(const entity &e)
	{
		auto ptr = get<T>();
		if( ptr) {
			return ptr->read()->template get<T>(e);
		}
		return nullptr;
	}

	template <typename T>
	inline void me::ecs::registry::set(const entity &e, T t)
	{
		locker<component_vec> *v = nullptr;
		auto i = vecs.find(typeid(T));
		if (i == vecs.end())
		{
			vecs.emplace(typeid(T), new locker<component_vec>());
			v = vecs.find(typeid(T))->second.get();
		}
		else
		{
			v = i->second.get();
		}
		auto data = v->write();
		data->set(t, e);
	}
	// template <typename... Ts, typename... Cv>
	// std::tuple<typename Ts::guard_type...> get_lockers(const entity &e, std::tuple<Cv...> &data)
	// {
	// 	return get_lockers_helper<Ts...>(e, data, std::make_index_sequence<sizeof...(Cv)>());
	// }

	// template <typename... Ts, typename... Cv, size_t... Is>
	// std::tuple<typename Ts::guard_type...> get_lockers_helper(const entity &e, std::tuple<Cv...> &data, std::index_sequence<Is...>)
	// {
	// 	auto data1 = std::make_tuple(((std::get<Is>(data).get()))...);
	// 	auto data2 = std::make_tuple(((std::get<Is>(data1)==nullptr)?nullptr:std::get<Is>(data1)->template get<typename Ts::type>(e))...);
	// 	auto data3 = std::make_tuple(((std::get<Is>(data2).get()==nullptr)?nullptr:std::make_unique(Ts::template get_guard<typename Ts::type,locker_ref,component>( *std::get<Is>(data2).get())))...);
	// 	// return std::make_tuple<locker_ref<typename Ts::type,component>...>(std::move(*(std::get<Is>(data1)).get())...);
	// 	return data3;
	// }

	template <typename... Ts, typename... Cv>
	std::tuple<std::unique_ptr<typename Ts::guard_type>...> get_guards(std::tuple<Cv*...> &data)
	{
		return get_guards_helper<Ts...>(data, std::make_index_sequence<sizeof...(Cv)>());
	}

	template <typename... Ts, typename... Cv, size_t... Is>
	std::tuple<std::unique_ptr<typename Ts::guard_type>...> get_guards_helper(std::tuple<Cv*...> &data, std::index_sequence<Is...>)
	{
		return std::make_tuple(((std::get<Is>(data) == nullptr ? nullptr : std::make_unique<typename Ts::guard_type>(Ts::template get_guard<typename Ts::type>(*std::get<Is>(data)))))...);
	}

	template <typename... Ts>
	std::tuple<typename Ts::guard_type...> deref_guards(std::tuple<std::unique_ptr<typename Ts::guard_type>...> &&data)
	{
		return deref_guards_helper<Ts...>(std::move(data), std::make_index_sequence<sizeof...(Ts)>());
	}

	template <typename... Ts, size_t... Is>
	std::tuple<typename Ts::guard_type...> deref_guards_helper(std::tuple<std::unique_ptr<typename Ts::guard_type>...> &&data, std::index_sequence<Is...>)
	{
		return std::make_tuple<typename Ts::guard_type...>(std::move((*std::get<Is>(data)))...);
	}

	template <typename T1, typename... Ts>
	inline me::thread_pool me::ecs::registry::query_sync(queries::query<T1, Ts...>::func f)
	{
		return std::thread([this, &f]()
						   {

		queries::query<T1, Ts...> q(f);
		// auto guards = std::make_tuple((*this->get<typename T1::type>()).read(), (*this->get<typename Ts::type>()).read()...);

		auto p = this->get<typename T1::type>();
		component_vec &v = p->value;

		for (auto &[e, c] : v)
		{
			auto values = std::make_tuple(this->get<typename T1::type>()->value.template get<typename T1::type>(e), (this->get<typename Ts::type>()->value.template get<typename Ts::type>(e))...);
			auto guards = get_guards<T1,Ts...>(values);
			if (is_not_nulls(guards))
			{
				q.run_query(deref_guards<T1,Ts...>(std::move(guards)));
			}
		} });
	};

	template <typename T1, typename... Ts>
	inline me::thread_pool me::ecs::registry::query_async(queries::query<T1, Ts...>::func f)
	{
		me::thread_pool pool;

		queries::query<T1, Ts...> q(f);
		// auto guards = std::make_tuple((*this->get<typename T1::type>()).read(), (*this->get<typename Ts::type>()).read()...);

		auto p = this->get<typename T1::type>();
		component_vec &v = p->value;

		for (auto &[e, c] : v)
		{
			auto values = std::make_tuple(this->get<typename T1::type>(e), (this->get<typename Ts::type>(e))...);
			int a = values;
			std::tuple<std::unique_ptr<typename T1::guard_type>,std::unique_ptr<typename Ts::guard_type>...> guards = get_guards<T1, Ts...>(values);

			if (is_not_nulls(guards))
			{
				pool.push(std::thread([guards = std::move(guards), &q]() mutable
									  { q.run_query(std::move(deref_guards<T1, Ts...>(std::move(guards)))); }));
			}
		}
		return pool;
	};
}
;
#endif // !MATRIX_ENGINE_REGISTRY
