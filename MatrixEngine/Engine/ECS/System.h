#ifndef MATRIX_ENGINE_SYSTEM
#define MATRIX_ENGINE_SYSTEM

#include "ISystem.h"
#include "Registry.h"

namespace me
{


	
	class System : public me::ISystem
	{
	protected:


		virtual inline void onUpdate(SystemArgs& args) override
		{

			

		}




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

