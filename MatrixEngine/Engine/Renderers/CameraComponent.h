#ifndef MATRIX_ENGINE_CAMERA_COMPONENT
#define MATRIX_ENGINE_CAMERA_COMPONENT

#include <SFML/Graphics.hpp>

namespace me
{

	class CameraComponent
	{
	public:
		CameraComponent(float = 10.0f);

		const float& getSize() const;

		const bool& getIsMain() const;
		void setIsMain(bool);

	private:
		bool _isMain;
		float _size;
	};
}



#endif // !MATRIX_ENGINE_CAMERA_COMPONENT
