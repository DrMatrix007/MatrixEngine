#ifndef MATRIX_ENGINE_REGISTRY
#define MATRIX_ENGINE_REGISTRY

#include <map>
#include <memory>
#include <typeindex>
#include <tuple>
#include <functional>
#include "Entity.h"

namespace me
{
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

		template<typename Data,typename Deleter>
		static inline auto getGuard(const Locker<Data,Deleter>& data)
		{
			return std::move(data.read());
		}
	};

	template<typename T>
	struct Write
	{
		using Guard = Guard<T*>;
		using Type = T;

		template<typename Data,typename Deleter>
		static inline auto getGuard(Locker<Data,Deleter>& data)
		{
			return data.write();
		}
	};



	class Registry
	{
	public:
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
		inline void query(void(*f)(typename Ts::Guard&...) )
		{
			auto logout = me::logout.write();
			auto vecsGuards = std::make_tuple((this->read_vec<typename Ts::Type>())...);
			if (checkNotNulls(vecsGuards))
			{
				auto vecs = me::apply([](auto&... data)
				{
					return std::make_tuple<const ComponentVec<typename Ts::Type>*...>(( data.getPointer())...);
				},vecsGuards);
				auto& first = *std::get<0>(vecs);
				for (auto& i : first)
				{
					const Entity& e= i.first;

					auto lockers = me::apply([&e](auto&... data)
					{
						return std::make_tuple(data->get(e)...);
					}, vecs);
					if (checkNotNulls(lockers))
					{
						auto guards = me::apply([](auto&... data)
						{
							return std::make_tuple(Ts::getGuard(*data)...);
						}, lockers);
						me::apply(f,guards);

					}

					//getComponents<Ts...>(a, vecs);

				}
			}
		}

	private:
		std::map<std::type_index, std::unique_ptr<UniqueLocker<IComponentVec>>> _data;

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



}

#endif // !MATRIX_ENGINE_REGISTRY
