#ifndef MATRIX_ENGINE_SYSTEM
#define MATRIX_ENGINE_SYSTEM

#include "ISystem.h"
#include "Registry.h"

namespace me
{


	template<typename ...T>
	class System : public me::ISystem
	{
	protected:
		static constexpr auto orders = std::make_index_sequence<sizeof...(T)>{};

		virtual inline Registry::QueryResult<T*...> getQuery(Registry& reg)
		{
			return reg.query<T...>();
		}

		virtual inline void onUpdate(SystemArgs& args) override
		{

			auto result = getQuery(args.getRegistry());
			for (auto& i : result)
			{
				onUpdateImpl(args, i, orders);
			}

		}


		template<size_t... TsOrders>
		inline void onUpdateImpl(SystemArgs& args, std::tuple<const Entity, T*...> data, std::index_sequence<TsOrders...>)
		{
			onUpdate(args, std::get<0>(data), (*std::get<TsOrders + 1>(data))...);
		}

		virtual void onUpdate(SystemArgs&,const Entity, T&...) abstract;

	};

	/*template<template<typename> typename ...DataAccess, typename ...T>
	class System<DataAccess<T>...> : public me::ISystem
	{
	protected:




	};
	template<typename ...T>
	class MultiThreadedSyncSystem {};


	template<template<typename> typename ...DataAccess, typename ...T>
	class MultiThreadedSyncSystem<DataAccess<T>...> : public me::System<DataAccess<T>...>
	{
	public:
		virtual inline void onUpdate(SystemArgs& args) override
		{

			auto result = this->getQuery(args.getRegistry());
			_pool.getVec().push_back(std::move(std::thread(std::function<void(decltype(result))>([&](auto result)
			{
				for (auto& i : result)
				{
					this->onUpdateImpl(args, i, this->orders);
				}
			}), std::move(result))));

		}
		virtual inline void onLateUpdate(SystemArgs& args) override
		{
			_pool.join();
			_pool.clear();
		}

	private:
		ThreadPool _pool;
	};

	template<typename ...T>
	class MultiThreadedAsyncSystem {};


	template<template<typename> typename ...DataAccess, typename ...T>
	class MultiThreadedAsyncSystem<DataAccess<T>...> : public me::System<DataAccess<T>...>
	{
	public:
		virtual inline void onUpdate(SystemArgs& args) override
		{

			auto result = this->getQuery(args.getRegistry());
			for (auto& i : result)
			{

				_pool.getVec().push_back(std::move(std::thread(std::function<void(std::tuple<UniqueLocker<T>*...>)>([&](auto t)
				{
					this->onUpdateImpl(args, t, this->orders);
				}), i)));

			}
		}
		virtual inline void onLateUpdate(SystemArgs& args) override
		{
			_pool.join();
			_pool.clear();
		}

	private:
		ThreadPool _pool;
	};*/

}



#endif // !MATRIX_ENGINE_ISYSTEM

