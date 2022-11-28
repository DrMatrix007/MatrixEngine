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

		private:
			std::vector<std::tuple<Entity, T...>> _data;

			// Inherited via IJobPool
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

		void update(Application*);

	private:
		std::map<std::type_index, std::unique_ptr<IComponentVec>> _data;
		std::vector<std::unique_ptr<ISystem>> _systems;
		//template<typename ...Ts>
		//inline auto getComponents(const Entity& e,const std::tuple<ComponentVec<typename Ts::Type>...>& data)
		//{
		//	return getComponentsHelper(e, data, std::make_index_sequence<sizeof...(Ts)>{});
		//}
		//template<typename ...Ts,size_t...Is>
		//inline auto getComponentsHelper(const Entity& e,const std::tuple<ComponentVec<typename Ts::Type>...>& data,std::index_sequence<Is...>)
		//{
		//	return std::make_tuple((std::get<Is>(data).get(e))...);
		//}

		//template<typename ...Ts>
		//inline auto getVecFromGuards(std::tuple<Ts...>& data)
		//{
		//	return getVecsFromGuardsHelper(data, std::make_index_sequence<sizeof...(Ts)>{});
		//}
		//template<typename ...Ts, size_t...Is>
		//inline auto getVecsFromGuardsHelper(std::tuple<Ts...>& data, std::index_sequence<Is...>)
		//{
		//	return std::make_tuple((*std::get<Is>(data))...);
		//}


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
			auto g = (it->second.get()->read());
			const ComponentVec<T>* ptr = dynamic_cast<const ComponentVec<T>*>(g.getPointer());
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
