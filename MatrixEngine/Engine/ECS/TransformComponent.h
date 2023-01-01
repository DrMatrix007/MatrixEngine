#ifndef MATRIX_ENGINE_TRANSFORM_COMPONENT
#define MATRIX_ENGINE_TRANSFORM_COMPONENT

#include "SFML/Graphics.hpp"

namespace me
{
	class TransformComponent : public sf::Transformable
	{
	public:
		const static TransformComponent ZERO;
		void setLayer(const size_t& l);
		const size_t&	getLayer() const;
	private:
		size_t _layer;
	};

}


#endif // !MATRIX_ENGINE_TRANSFORM_COMPONENT