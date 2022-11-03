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
		template<typename T>
		T* get(const entity&);

		template<typename T>
		T* set(std::unique_ptr<T>, const entity&);
		template<typename T>
		T* set(T, const entity&);

		inline std::map<entity, std::unique_ptr<component>>::iterator begin()
		{
			return components.begin();
		}
		inline std::map<entity, std::unique_ptr<component>>::iterator end()
		{
			return components.end();
		}

	};


	class registry
	{
	private:
		std::map<std::type_index, std::unique_ptr<locker<component_vec>>> vecs;

	public:
		template<typename T, typename F>
		void operate(F);
		template<typename T>
		locker<component_vec>* get();

		template<typename T>
		T* set(const entity&, T);

	};



}

template<typename T>
T* me::ecs::component_vec::get(const entity& e)
{
	auto i = components.find(e);
	if (i == components.end())
	{
		return nullptr;
	}
	return dynamic_cast<T*>(i->second.get());
}
template<typename T>
T* me::ecs::component_vec::set(std::unique_ptr<T> t, const entity& e)
{
	T* og = t.release();
	auto c = dynamic_cast<component*>(og);
	if (c)
	{
		components[e] = std::unique_ptr<component>(c);
	}
	return c ? og : nullptr;
}
template<typename T>
T* me::ecs::component_vec::set(T t, const entity& e)
{
	return set(std::make_unique<T>(t), e);
}



template<typename T, typename F>
inline void me::ecs::registry::operate(F f)
{

	std::thread([this, &f]()
	{
		T* ptr;
		locker<component_vec>* v = this->get<T>();
		if (v)
		{
			component_vec& m = v->lock();
			for (auto& [e, c] : m)
			{
				ptr = dynamic_cast<T*>(c.get());
				if (ptr)
				{
					f(e, ptr);
				}
			}

		}
	}).join();
}

template<typename T>
inline locker<me::ecs::component_vec>* me::ecs::registry::get()
{
	auto i = vecs.find(typeid(T));
	if (i == vecs.end())

	{

		//vecs.insert(typeid(T),new locker<component_vec>());
		//return vecs.find(typeid(T))->second.get();
		return nullptr;
	}
	else
	{
		return i->second.get();
	}


}

template<typename T>
inline T* me::ecs::registry::set(const entity& e, T t)
{
	locker<component_vec>* v = nullptr;
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
	auto [guard, data] = v->guard();
	return data.set(t, e);
}




#endif // !MATRIX_ENGINE_REGISTRY
