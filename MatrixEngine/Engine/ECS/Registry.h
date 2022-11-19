#ifndef MATRIX_ENGINE_REGISTRY
#define MATRIX_ENGINE_REGISTRY

#include <map>
#include <memory>
#include <typeindex>
#include <tuple>
#include <iterator>
#include <functional>

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


		UniqueLocker<T>* get(const Entity& e) const;
		UniqueLocker<T>* set(const Entity& e, T t);

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
		std::map<Entity, std::unique_ptr<UniqueLocker<T>>> _data;

		friend class Registry;

	};

	template<typename T>
	struct Read
	{
		using Guard = Guard<const T*>;
		using Type = T;

		template<typename Data, typename Deleter>
		static inline auto getGuard(const Locker<Data, Deleter>& data)
		{
			return std::move(data.read());
		}
	};

	template<typename T>
	struct Write
	{
		using Guard = Guard<T*>;
		using Type = T;

		template<typename Data, typename Deleter>
		static inline auto getGuard(Locker<Data, Deleter>& data)
		{
			return data.write();
		}
	};



	class Registry
	{
	private:
		template<typename ...T>
		class QueryResult
		{
			using Func = std::function<void(T&...)>;
		public:

			inline auto begin();

			inline auto end();

			inline void push(std::tuple<T...>& g);

			/*	virtual ThreadPool async_threads() override
				{
					ThreadPool ans;

					for (size_t i = 0; i < _data.size(); i++)
					{
						auto& t = _data[i];
						auto f = std::function<void()>([&t, f{ this->_func }]()
						{
							me::apply(f, t);
						});
						ans.push(f);
					}
					return ans;
				}
				virtual ThreadPool async_thread() override
				{
					ThreadPool ans;
					std::vector<std::tuple<T...>>& data = _data;
					ans.getVec().emplace_back([&data, f{ this->_func }](){

						for (auto& it : data)
						{

							me::apply(f, it);

						}
					});
					return ans;
				}
				virtual void sync() override
				{
					for (auto& it : _data)
					{
						me::apply(_func, it);
					}
				}*/


		private:
			std::vector<std::tuple<T...>> _data;

			// Inherited via IJobPool
		};
	public:
		
		template<typename T>
		void pushSystem(T t);
		
		template<typename T>
		UniqueLocker<T>* set(const Entity& e, T t);
		
		template<typename T>
		UniqueLocker<T>* get(const Entity& e) const;
		
		template<typename T>
		UniqueLocker<IComponentVec>* get() const;
		
		template<typename T>
		ReadGuard<ComponentVec<T>> read_vec();
		
		template<typename T>
		WriteGuard<ComponentVec<T>> write_vec();
		
		template<typename ...Ts>
		inline QueryResult<UniqueLocker<typename Ts::Type>*...> query();

		void update(Application*);

	private:
		std::map<std::type_index, std::unique_ptr<UniqueLocker<IComponentVec>>> _data;
		std::vector<std::unique_ptr<UniqueLocker<ISystem>>> _systems;
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
	inline UniqueLocker<T>* ComponentVec<T>::get(const Entity& e) const
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
	inline UniqueLocker<T>* ComponentVec<T>::set(const Entity& e, T t)
	{
		_data[e] = std::make_unique<UniqueLocker<T>>(t);
		return _data[e].get();
	}
	template<typename T>
	void ComponentVec<T>::remove(const Entity& e)
	{
		_data.erase(e);
	}
	template<typename T>
	inline void Registry::pushSystem(T t)
	{
		auto lock = new UniqueLocker<ISystem>(dynamic_cast<ISystem*>(new T(std::move(t))));
		_systems.emplace_back(lock);
	}
	template<typename T>
	inline UniqueLocker<T>* Registry::set(const Entity& e, T t)
	{
		auto it = _data.find(typeid(T));
		if (it == _data.end())
		{
			_data.emplace(typeid(T), std::make_unique<UniqueLocker<IComponentVec>>(new ComponentVec<T>()));
		}
		auto guard = _data[typeid(T)].get()->write();
		auto ptr = dynamic_cast<ComponentVec<T>*>(guard.getPointer());
		if (!ptr)
		{
			throw std::runtime_error("cannot understand component vec");
		}
		return ptr->set(e, t);
	}

	template<typename T>
	inline UniqueLocker<T>* Registry::get(const Entity& e) const
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
	inline UniqueLocker<IComponentVec>* Registry::get() const
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
	inline ReadGuard<ComponentVec<T>> Registry::read_vec()
	{
		auto v = get<T>();
		if (v)
		{
			return castGuard<const ComponentVec<T>*>(v->read());
		}
		else
		{
			return { nullptr,nullptr };
		}
	}

	template<typename ...Ts>
	inline Registry::QueryResult<UniqueLocker<typename Ts::Type>*...> Registry::query()
	{
		QueryResult<UniqueLocker<typename Ts::Type>*...> ans;

		auto vecsGuards = std::make_tuple((this->read_vec<typename Ts::Type>())...);
		if (checkNotNulls(vecsGuards))
		{
			auto vecs = me::apply([](auto&... data)
			{
				return std::make_tuple<const ComponentVec<typename Ts::Type>*...>((data.getPointer())...);
			}, vecsGuards);
			auto& first = *std::get<0>(vecs);
			for (auto& i : first)
			{
				const Entity& e = i.first;

				auto lockers = me::apply([&e](auto&... data)
				{
					return std::make_tuple(data->get(e)...);
				}, vecs);
				if (checkNotNulls(lockers))
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
	inline void Registry::QueryResult<T...>::push(std::tuple<T...>& g)
	{
		_data.push_back(std::move(g));
	}

}

#endif // !MATRIX_ENGINE_REGISTRY
