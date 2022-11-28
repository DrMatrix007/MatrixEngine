#ifndef MATRIX_ENGINE_CAMERA_COMPONENT
#define MATRIX_ENGINE_CAMERA_COMPONENT

#include <SFML/Graphics.hpp>

namespace me
{

	class CameraComponent
	{
		CameraComponent(float = 10.0f);

		const float& getSize() const;

	private:
		float _size;
	};
}



#endif // !MATRIX_ENGINE_CAMERA_COMPONENT
