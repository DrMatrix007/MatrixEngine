using MatrixEngine.MathM;
using MatrixEngine.Physics;
using MatrixEngine.Renderer;
using MatrixEngine.Scenes;
using SFML.Graphics;
using SFML.System;
using SFML.Window;

namespace MatrixEngine.System {
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

        public Renderer renderer
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
        public CanvasRenderer canvasRenderer
        {
            get;
            private set;
        }

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
            camera = new Camera();
            renderer = new Renderer(this);
            rigidBodyManager = new PhysicsEngine(this);
            canvasRenderer = new CanvasRenderer(this);
        }

        public Vector2f windowSize
        {
            get;
            private set;
        } = new Vector2f(100, 100);

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

                camera.zoom += (float)e.Delta / 10;

            };

            scene.app = this;


            while (window.IsOpen) {
                var size = window.Size;

                windowSize = new Vector2f(2.0f.Pow(camera.zoom) * ((float)size.X / size.Y).Sqrt() * 100, 2.0f.Pow(camera.zoom) * 100 / ((float)size.X / size.Y).Sqrt());


                window.DispatchEvents();


                scene.Update();

                window.SetView(new View(camera.position, windowSize));

                rigidBodyManager.Update();

                window.Clear(Color.Black);



                renderer.Render();

                canvasRenderer.Render();


                window.Display();

                _deltaTime = deltaTimeClock.Restart();

            }


        }


    }
}
