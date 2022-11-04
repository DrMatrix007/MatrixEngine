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

#include "../utils/utils.h"

namespace me::ecs
{

	class component_vec
	{
	private:
		std::map<entity, std::unique_ptr<component>> components;

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
	};

}

template <typename T>
T *me::ecs::component_vec::get(const entity &e)
{
	auto i = components.find(e);
	if (i == components.end())
	{
		return nullptr;
	}
	return dynamic_cast<T *>(i->second.get());
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
	const T *ptr;
	locker<component_vec> *v = this->get<T>();
	if (v)
	{
		auto [g, m] = v->read();
		for (const auto &[e, c] : m)
		{
			ptr = dynamic_cast<T *>(c.get());
			if (ptr)
			{
				threads.push_back(std::thread(f,e, ptr));
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
