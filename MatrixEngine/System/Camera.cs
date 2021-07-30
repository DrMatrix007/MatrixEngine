using MatrixEngine.Physics;
using SFML.System;


namespace MatrixEngine.System {
    public class Camera {

        App app;


        public Camera(App app) {
            this.app = app;
        }

        public Vector2f position = new Vector2f();

        public Vector2f size
        {
            get => new Vector2f(2.0f.Pow(zoom) * ((float)app.window.Size.X / app.window.Size.Y).Sqrt() * 100, 2.0f.Pow(zoom) * 100 / ((float)app.window.Size.X / app.window.Size.Y).Sqrt());
        }

        public Rect rect
        {
            get {
                var s = size;
                return new Rect(position.X - s.X / 2, position.Y - s.Y / 2,s.X,s.Y);
            }
        }

        public float zoom = -2;

    }
}
