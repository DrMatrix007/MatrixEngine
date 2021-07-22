using MatrixEngine.MathM;
using MatrixEngine.Physics;
using MatrixEngine.Scenes;
using SFML.Graphics;
using SFML.System;
using SFML.Window;

namespace MatrixEngine.App {
    public sealed class App {

        public PhysicsEngine rigidBodyManager
        {
            get;
            private set;
        }

        public KeyHandler keyHandler
        {
            get;
            private set;
        }

        public readonly string AppName;

        public Scene scene;

        public Camera camera;

        public SpriteRenderer spriteRenderer
        {
            get;
            private set;
        }

        public RenderWindow window
        {
            get;
            private set;
        }

        private Clock timeClock = new Clock();

        private Clock deltaTimeClock = new Clock();

        private Time _deltaTime
        {
            get;
            set;
        }
        public float deltaTime
        {
            get => _deltaTime.AsSeconds();
        }
        public float time
        {
            get => timeClock.ElapsedTime.AsSeconds();
        }


        public App(string appName, Scene scene) {
            AppName = appName;

            this.scene = scene;
            window = new RenderWindow(new VideoMode(800, 600), AppName);
            window.SetKeyRepeatEnabled(true);

            window.Closed += (s, e) => {
                ((Window)s).Close();
            };

            window.KeyPressed += Window_KeyPressed;
            window.KeyReleased += Window_KeyReleased;

            keyHandler = new KeyHandler();
            spriteRenderer = new SpriteRenderer();
            camera = new Camera();
            rigidBodyManager = new PhysicsEngine(this);
        }



        private void Window_KeyReleased(object sender, KeyEventArgs e) {
            keyHandler.ReleasedKey(e.Code);
        }

        private void Window_KeyPressed(object sender, KeyEventArgs e) {
            keyHandler.PressedKey(e.Code);
        }


        public void Run() {

            timeClock.Restart();

            deltaTimeClock.Restart();

            window.MouseWheelMoved += (s, e) => {

                camera.zoom += (float)e.Delta/10;
 
            };

            while (window.IsOpen) {
                var size = window.Size;
                window.SetView(new View(camera.position, new Vector2f((2.0f).Pow(camera.zoom) * ((float)size.X / size.Y).Sqrt() * 100, (2.0f).Pow(camera.zoom) * 100 / ((float)size.X / size.Y).Sqrt())));

                scene.app = this;

                window.DispatchEvents();


                scene.Update();

                rigidBodyManager.Update();

                window.Clear(Color.Black);



                spriteRenderer.Render();

                window.Display();

                _deltaTime = deltaTimeClock.Restart();

            }


        }


    }
}
