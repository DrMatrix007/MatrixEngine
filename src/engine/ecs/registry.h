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
		std::map<entity, std::unique_ptr<component>> components;
		std::map<std::type_index, std::unique_ptr<base_system>> systems;

	public:
		template <typename T>
		T *get(const entity &);

		template <typename T>
		T *set(std::unique_ptr<T>, const entity &);
		template <typename T>
		T *set(T, const entity &);

		inline std::map<entity, std::unique_ptr<component>>::iterator begin()
		{
			return components.begin();
		}
		inline std::map<entity, std::unique_ptr<component>>::iterator end()
		{
			return components.end();
		}

		inline std::map<entity, std::unique_ptr<component>>::const_iterator begin() const
		{
			return components.cbegin();
		}
		inline std::map<entity, std::unique_ptr<component>>::const_iterator end() const
		{
			return components.cend();
		}
	};
	namespace query
	{

		template <typename T>
		class query_vec_result
		{
		};

		template <template <typename> typename base, typename T>
		class query_vec_result<base<T>>
		{
			using guard_type = base<T>::guard_type;

			inline query_vec_result(guard_type g, component_vec &c) : l(g), v(c){};
			guard_type l;
			component_vec &v;
		};

		template <typename... T>
		struct function_tempalte
		{
			using type = std::function<void(T...)>;
		};

		template <typename T>
		struct read
		{
			template <typename A>
			using format_type = const A;
			using param_type = format_type<T *>;
			using type = T;
			using guard_type = std::shared_lock<std::shared_mutex>;

			static inline query_vec_result<read<T>> get_guard(locker<component_vec> &l)
			{

				return query_vec_result<read<T>>(guard_type(l.mutex), l.value);
			}
		};

		template <typename T>
		struct write
		{
			template <typename A>
			using format_type = A;
			using param_type = format_type<T *>;

			using type = T;
			using guard_type = std::unique_lock<std::shared_mutex>;

			static inline query_vec_result<write<T>> get_guard(locker<component_vec> &l)
			{
				auto [a, b] = l.write();
				return query_vec_result<write<T>>(a, b);
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
			using parameter_type = infer_type<T1>::type;
			using all = utils::connect_templates<function_tempalte, function_tempalte<parameter_type>, typename query<T...>::all>::type;
			using vecs_types = utils::connect_templates<std::tuple, std::tuple<std::unique_ptr<query_vec_result<T1>>>, typename query<T...>::vecs_types>::type;

			using func = all::type;

			func f;

			inline query(func fun) : f(fun)
			{
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

	public:
		template <typename T, typename F>
		std::thread write_components(const F &);

		template <typename T, typename F>
		std::thread read_component(const F &);

		template <typename T, typename F>
		std::vector<std::thread> write_components_async(const F &);

		template <typename T, typename F>
		std::vector<std::thread> read_component_async(const F &);

		template <typename T>
		locker<component_vec> *get();

		template <typename T>
		T *set(const entity &, T);

		template <typename T1, typename T2, typename... Ts>
		query::query<Ts...>::vecs_types get_vecs()
		{
			return std::tuple_cat(get_vec<T1>(), get_vecs<T2, Ts...>());
		}
		template <typename T1>
		query::query<T1>::vecs_types get_vecs()
		{
			return std::make_tuple(get_vec<T1>());
		}

		template <typename base>
		std::unique_ptr<query::query_vec_result<base>> get_vec()
		{
			auto ptr = get<typename base::type>();
			if (ptr)
			{
				return std::make_unique<query::query_vec_result<base>>(base::get_guard(*ptr));
			}
			return nullptr;
		}
	};
};

template <typename T>
T *me::ecs::component_vec::get(const entity &e)
{
	auto i = components.find(e);
	if (i == components.end())
	{
		return nullptr;
	}
	else
	{
		return dynamic_cast<T *>(i->second.get());
	}
}
template <typename T>
T *me::ecs::component_vec::set(std::unique_ptr<T> t, const entity &e)
{
	T *og = t.release();
	auto c = dynamic_cast<component *>(og);
	if (c)
	{
		components[e] = std::unique_ptr<component>(c);
	}
	return c ? og : nullptr;
}
template <typename T>
T *me::ecs::component_vec::set(T t, const entity &e)
{
	return set(std::make_unique<T>(t), e);
}

template <typename T, typename F>
inline std::thread me::ecs::registry::write_components(const F &f)
{

	return std::thread([this, &f]()
					   {
		T* ptr;
		locker<component_vec>* v = this->get<T>();
		if (v)
		{
			auto [g,m] = v->write();
			for (const auto& [e, c] : m)
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
std::thread me::ecs::registry::read_component(const F &f)
{
	return std::thread([this, &f]()
					   {
		const T* ptr;
		locker<component_vec>* v = this->get<T>();
		if (v)
		{
			auto [g,m] = v->read();
			for (const auto& [e, c] : m)
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
inline std::vector<std::thread> me::ecs::registry::write_components_async(const F &f)
{
	std::vector<std::thread> threads;
	T *ptr;
	locker<component_vec> *v = this->get<T>();
	if (v)
	{
		auto [g, m] = v->read();
		for (const auto &[e, c] : m)
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
std::vector<std::thread> me::ecs::registry::read_component_async(const F &f)
{
	std::vector<std::thread> threads;
	const T *ptr;
	locker<component_vec> *v = this->get<T>();
	if (v)
	{
		auto [g, m] = v->read();
		for (const auto &[e, c] : m)
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
inline T *me::ecs::registry::set(const entity &e, T t)
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
	auto [guard, data] = v->write();
	return data.set(t, e);
	return nullptr;
}

#endif // !MATRIX_ENGINE_REGISTRY
