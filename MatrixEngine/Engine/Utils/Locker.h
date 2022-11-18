#ifndef MATRIX_ENGINE_LOCKER
#define MATRIX_ENGINE_LOCKER

#include <memory>
#include <mutex>
#include <shared_mutex>
#include <tuple>
namespace me
{

	/*class SharedRecursiveMutex : public std::shared_mutex
	{
	public:
		inline SharedRecursiveMutex() : _counter(0), _writer()
		{}

		void lock()
		{
			auto new_id = std::this_thread::get_id();
			if (_writer == new_id)
			{
				_counter++;
			}
			else
			{
				shared_mutex::lock();
				_writer = new_id;
				_counter = 1;
			}
		}

		void unlock()
		{
			if (_counter > 1)
			{
				_counter--;
			}
			else
			{
				_writer = std::thread::id();
				_counter = 0;
				shared_mutex::unlock();

			}
		}

	private:
		std::atomic<std::thread::id> _writer;
		size_t _counter;
	};*/


	template<typename T, typename Deleter>
	class Locker;

	template<typename T>
	class Guard {};

	template<typename T, typename Old>
	Guard<T> castGuard(Old&& guard);



	template<typename T>
	class Guard<const T*>
	{
	public:
		using RefType = const T&;
		inline const T* operator->() const
		{
			return getPointer();
		}
		inline const T& operator*() const
		{
			return *getPointer();
		}

		inline Guard(const T* ptr, std::shared_mutex* const mutex) : _ref(ptr), _mutex(mutex)
		{
			lock();
		}
		inline Guard(Guard&& other) noexcept : _ref(other._ref), _mutex(other._mutex)
		{
			other.unlock();
			other._mutex = nullptr;
			other._ref = nullptr;
			lock();
		}
		inline ~Guard()
		{
			unlock();
		}
		inline const T* getPointer() const
		{
			return _ref;
		}
		

	private:
		const T* _ref;
		//std::shared_mutex* const _mutex_ref;
		std::shared_mutex* _mutex;
		template<typename T, typename Deleter>
		friend class Locker;


		template<typename T, typename Old>
		friend Guard<T> castGuard(Old&& guard);

		void lock()
		{
			if (_mutex && _ref)
			{
				_mutex->lock_shared();
			}
		}
		void unlock()
		{
			if (_mutex && _ref)
			{
				_mutex->unlock_shared();
			}
		}
	};

	template <typename T>
	class Guard<T*>
	{
	public:
		using RefType = T&;

		inline T* operator->() const
		{

			return getPointer();
		}
		inline T& operator*() const
		{
			return *getPointer();
		}
		inline T* getPointer() const
		{
			return _ref;
		}

		inline Guard(T* ptr, std::shared_mutex* const mutex) : _ref(ptr), _mutex(mutex)
		{
			lock();
		}
		inline Guard(Guard&& other) noexcept : _ref(other._ref),_mutex(other._mutex)
		{
			other.unlock();
			other._mutex = nullptr;
			other._ref = nullptr;
			lock();
		}
		~Guard()
		{
			unlock();
		}
	private:

		//std::shared_mutex* const _mutex_ref;
		std::shared_mutex* _mutex;
		T* _ref;

		template<typename T, typename Deleter>
		friend class Locker;


		template<typename T, typename Old>
		friend Guard<T> castGuard(Old&& guard);

		void lock()
		{
			if (_mutex && _ref)
			{
				_mutex->lock();
			}
		}
		void unlock()
		{
			if (_mutex && _ref)
			{
				_mutex->unlock();
			}
		}


	};

	template<typename T, typename Deleter = std::default_delete<T>>
	class Locker
	{
	public:
		inline Locker(T val) : _data(new T(std::move(val)))
		{}
		inline Locker(std::unique_ptr<T, Deleter> val) : _data(std::move(val))
		{}
		inline Locker(T* val) : _data(val)
		{}
		inline Locker() : _data(new T())
		{}
		Guard<const T*> read() const
		{
			return { this->_data.get(), &this->_mutex };
		}
		template<typename To = T>
		Guard<T*> write()
		{
			return { this->_data.get(), &this->_mutex };
		}

	private:

	protected:

		mutable std::shared_mutex _mutex;
		std::unique_ptr<T, Deleter> _data;
	};






}

#endif // !MATRIX_ENGINE_LOCKER
