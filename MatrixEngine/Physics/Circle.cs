using SFML.System;

namespace MatrixEngine.Physics {
    public class Circle {
        public Vector2f position;
        public float r;

        public float X => position.X;

        public float Y => position.Y;

        public Circle(Vector2f position, float r) {
            this.position = position;
            this.r = r;
        }
        
        
    }
}