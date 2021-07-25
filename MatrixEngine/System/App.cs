using MatrixEngine.Physics;
using MatrixEngine.Renderers;
using MatrixEngine.Scenes;
using MatrixEngine.System.AsyncOperations;
using SFML.Graphics;
using SFML.System;
using SFML.Window;
using System;

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

        public AsyncOperationManager asyncOperationManager
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

            //window.SetFramerateLimit(144);


            window.Closed += (s, e) => {
                ((Window)s).Close();
            };

            window.KeyPressed += Window_KeyPressed;
            window.KeyReleased += Window_KeyReleased;

            keyHandler = new KeyHandler();
            camera = new Camera(this);
            renderer = new Renderer(this);
            rigidBodyManager = new PhysicsEngine(this);
            canvasRenderer = new CanvasRenderer(this);
            asyncOperationManager = new AsyncOperationManager(this);
        }

        public Vector2f windowSize
        {
            get => camera.size;
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

            window.MouseWheelScrolled += (s, e) => {

                camera.zoom += (float)e.Delta / 10;

            };

            scene.app = this;

            //window.SetFramerateLimit(145);


            while (window.IsOpen) {




                window.DispatchEvents();


                scene.Update();

                asyncOperationManager.Update();

                window.SetView(new View(camera.position, camera.size));

                rigidBodyManager.Update();


                window.Clear(Color.Black);

                Utils.GetTimeInSeconds(() => {

                    renderer.Render();
                    canvasRenderer.Render();

                });

                //window.Draw(
                //        new Vertex[] {
                //        new Vertex(camera.rect.position+new Vector2f(5,5)),
                //        new Vertex(camera.rect.position+new Vector2f(5,-5+camera.size.Y))
                //}, PrimitiveType.Lines);




                window.Display();

                _deltaTime = deltaTimeClock.Restart();

            }


        }


    }
}
