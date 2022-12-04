#ifndef MATRIX_ENGINE_REGISTRY
#define MATRIX_ENGINE_REGISTRY

#include <map>
#include <memory>
#include <typeindex>
#include <tuple>
#include <vector>
#include <iterator>
#include <functional>
#include <algorithm>

#include "ISystem.h"
#include "Entity.h"
#include "../Utils/Utils.h"

namespace me
{
	class Application;

	class IComponentVec
	{

	public:
		IComponentVec() = default;
		virtual void remove(const Entity&) abstract;
		virtual ~IComponentVec()
		{}
	};

	template<typename T>
	class ComponentVec : public IComponentVec
	{

	public:
		ComponentVec() = default;
		ComponentVec(ComponentVec&) = delete;
		void remove(const Entity&) override;


		T* get(const Entity& e) const;
		T* set(const Entity& e, T t);

		inline auto begin()
		{
			return _data.begin();
		}
		inline auto end()
		{
			return _data.end();
		}
		inline auto begin() const
		{
			return _data.cbegin();
		}
		inline auto end() const
		{
			return _data.cend();
		}

	private:
		std::map<Entity, std::unique_ptr<T>> _data;

		friend class Registry;

	};

	class IResource
	{
	public:
		virtual ~IResource() {};
	};

	template<typename T>
	class Resource : public IResource
	{
	public:

		inline Resource(std::unique_ptr<T> p) : _ptr(std::move(p))
		{

		}

		inline T* get() const
		{
			return _ptr.get();
		}
		inline T* get()
		{
			return _ptr.get();
		}
		inline T* operator->()
		{
			return get();
		}

	private:

		std::unique_ptr<T> _ptr;
	};


	class Registry
	{
	public:
		template<typename ...T>
		class QueryResult
		{
			using Func = std::function<void(const Entity, T&...)>;
		public:

			inline auto begin();

			inline auto end();

			inline void push(std::tuple<const Entity, T...>& g);

			template<typename F>
			inline void orderBy(const F& f)
			{
				std::sort(_data.begin(), _data.end(), f);
			}
			template<typename Func>
			inline void forEach(const Func& f)
			{
				for (auto& i : _data)
				{
					auto r = me::deref(i);
					me::apply(f, r);
				}
			}

		private:
			std::vector<std::tuple<Entity, T...>> _data;
		};

	public:

		template<typename T>
		void pushSystem(std::unique_ptr<T>&& t);



		template<typename T>
		T* set(const Entity& e, T t);

		template<typename T>
		T* get(const Entity& e) const;

		template<typename T>
		IComponentVec* get() const;

		template<typename T>
		ComponentVec<T>* get_vec();


		template<typename ...Ts>
		inline QueryResult<Ts*...> query();

		template<typename T>
		T* getResource();

		template<typename T>
		void setResource(std::unique_ptr<T> data);

		template<typename T>
		void removeResource();



		void update(Application*);

	private:
		std::map<std::type_index, std::unique_ptr<IComponentVec>> _data;
		std::map<std::type_index, std::unique_ptr<IResource>> _resources;
		std::vector<std::unique_ptr<ISystem>> _systems;
	};
	template<typename T>
	inline T* ComponentVec<T>::get(const Entity& e) const
	{
		auto it = _data.find(e);
		if (it == _data.end())
		{
			return nullptr;
		}
		else
		{
			return it->second.get();
		}

	}
	template<typename T>
	inline T* ComponentVec<T>::set(const Entity& e, T t)
	{
		_data[e] = std::make_unique<T>(t);
		return _data[e].get();
	}
	template<typename T>
	void ComponentVec<T>::remove(const Entity& e)
	{
		_data.erase(e);
	}
	template<typename T>
	inline void Registry::pushSystem(std::unique_ptr<T>&& t)
	{
		_systems.emplace_back(std::unique_ptr<ISystem>(t.release()));
	}
	template<typename T>
	inline T* Registry::set(const Entity& e, T t)
	{
		auto it = _data.find(typeid(T));
		if (it == _data.end())
		{
			_data.emplace(typeid(T), (new ComponentVec<T>()));
		}

		auto ptr = dynamic_cast<ComponentVec<T>*>(_data[typeid(T)].get());
		if (!ptr)
		{
			throw std::runtime_error("cannot understand component vec");
		}
		return ptr->set(e, t);
	}

	template<typename T>
	inline T* Registry::get(const Entity& e) const
	{
		auto it = _data.find(typeid(T));
		if (it != _data.end())
		{
			const ComponentVec<T>* ptr = dynamic_cast<const ComponentVec<T>*>((it->second.get()));
			if (ptr == nullptr)
			{
				throw std::runtime_error("cannot understand component vec");
			}
			return ptr->get(e);
		}
		return nullptr;
	}

	template<typename T>
	inline IComponentVec* Registry::get() const
	{
		auto it = _data.find(typeid(T));
		if (it == _data.end())
		{
			return nullptr;
		}
		else
		{
			return it->second.get();
		}
	}

	template<typename T>
	inline ComponentVec<T>* Registry::get_vec()
	{
		ComponentVec<T>* v = dynamic_cast<ComponentVec<T>*>(get<T>());
		if (v)
		{
			return v;
		}
		else
		{
			return nullptr;
		}
	}

	template<typename ...Ts>
	inline Registry::QueryResult<Ts*...> Registry::query()
	{
		QueryResult< Ts*...> ans;

		auto vecs = std::make_tuple((this->get_vec<Ts>())...);
		if (checkNotNullTuple(vecs))
		{
			auto& first = *std::get<0>(vecs);
			for (auto& i : first)
			{
				const Entity e = i.first;

				std::tuple<const Entity, Ts*...> lockers = me::apply([&e](auto&... data)
				{
					return std::make_tuple<const Entity, Ts*...>(Entity(e), data->get(e)...);
				}, vecs);
				if (me::apply([](auto&, auto&... data)
				{
					return checkNotNulls(data...);
				}, lockers))
				{
					//auto guards = me::apply([](auto&... data)
					//{
					//	return std::make_tuple(Ts::getGuard(*data)...);
					//}, lockers);


					ans.push(lockers);

				}
			}
		}
		return ans;
	}

	template<typename T>
	inline T* Registry::getResource()
	{
		Resource<T>* ptr = nullptr;
		auto it = _resources.find(typeid(T));
		if (it != _resources.end())
		{
			ptr = dynamic_cast<decltype(ptr)>(it->second.get());
			if (ptr)
			{
				return ptr->get();
			}
		}
		return nullptr;
	}
	;
	template<typename T>
	inline void Registry::setResource(std::unique_ptr<T> data)
	{
		_resources.emplace(typeid(T), new Resource<T>(std::move(data)));
	}

	template<typename T>
	inline void Registry::removeResource()
	{
		_resources.erase(typeid(T));
	}


	template<typename ...T>
	inline auto Registry::QueryResult<T...>::begin()
	{
		return _data.begin();
	}

	template<typename ...T>
	inline auto Registry::QueryResult<T...>::end()
	{
		return _data.end();
	}

	template<typename ...T>
	inline void Registry::QueryResult<T...>::push(std::tuple<const Entity, T...>& g)
	{
		_data.push_back(std::move(g));
	}

}

#endif // !MATRIX_ENGINE_REGISTRY
