using MatrixEngine.Physics;
using MatrixEngine.Framework.MathM;
using SFML.System;

namespace MatrixEngine.Framework {

    public class Camera {
        private readonly App app;

        public Camera(App app) {
            this.app = app;
        }

        public Vector2f position = new();

        public Vector2f Size
        {
            get => new(2.0f.Pow(zoom) /* here is the freaking multuply*/ * ((float)app.Window.Size.X / app.Window.Size.Y).Sqrt() * 100, 2.0f.Pow(zoom) * 100 / ((float)app.Window.Size.X / app.Window.Size.Y).Sqrt());
        }

        public Rect Rect
        {
            get {
                var s = Size;
                return new Rect(position.X - s.X / 2, position.Y - s.Y / 2, s.X, s.Y);
            }
        }

        public float zoom = -2;
    }
}