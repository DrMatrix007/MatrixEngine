using SFML.Graphics;
using SFML.System;
using SFML.Window;
using System;
using System.IO;
using System.Runtime.InteropServices;
using MatrixEngine.Physics;
using MatrixEngine.Renderers;
using MatrixEngine.Testing;
using MatrixEngine.Framework.Operations;

namespace MatrixEngine.Framework {
    public sealed class App {
        public PhysicsEngine physicsEngine { get; private set; }

        public KeyHandler keyHandler { get; private set; }

        public OperationManager asyncOperationManager { get; private set; }

        public readonly string AppName;
        private readonly bool isDebug;
        public Scene scene;

        public Camera camera;

        public SpriteRenderer spriteRenderer { get; private set; }

        public RenderWindow window { get; private set; }

        private Clock timeClock = new Clock();

        private Clock deltaTimeClock = new Clock();
        public CanvasRenderer canvasRenderer { get; private set; }

        private Time _deltaTime { get; set; }

        public float deltaTime => _deltaTime.AsSeconds();

        public float time => timeClock.ElapsedTime.AsSeconds();

        private TestingWindow testingWindow { get; set; }

        public void AddToDebug<T>(T obj) where T : class {
            if (isDebug) {
                testingWindow.Add(obj);
            }
        }


        public App(string appName, bool isDebug, Scene scene) {
            var screen_size = VideoMode.DesktopMode;
            AppName = appName;
            this.isDebug = isDebug;
            this.scene = scene;
            if (isDebug) {
                window = new RenderWindow(
                    new VideoMode(screen_size.Width - 10, (uint) (screen_size.Height * ((float) 4 / 5) - 100 - 40)),
                    AppName);
                window.Position = new Vector2i();
            }
            else {
                window = new RenderWindow(new VideoMode(800, 600), AppName);
            }
            // window.SetIcon();

            window.SetKeyRepeatEnabled(true);


            window.Closed += (s, e) => { ((Window) s)?.Close(); };

            window.KeyPressed += Window_KeyPressed;
            window.KeyReleased += Window_KeyReleased;

            keyHandler = new KeyHandler();
            camera = new Camera(this);
            spriteRenderer = new SpriteRenderer(this);
            physicsEngine = new PhysicsEngine(this);
            canvasRenderer = new CanvasRenderer(this);
            asyncOperationManager = new OperationManager(this);
            if (isDebug) {
                testingWindow = new TestingWindow((4, 2));
            }
        }

        public Vector2f windowSize => camera.size;

        private void Window_KeyReleased(object sender, KeyEventArgs e) {
            keyHandler.ReleasedKey(e.Code);
        }

        private void Window_KeyPressed(object sender, KeyEventArgs e) {
            keyHandler.PressedKey(e.Code);
        }


        public void Run() {
            timeClock.Restart();

            deltaTimeClock.Restart();


            scene.app = this;


            var background = new Color(20, 93, 160);

            window.SetFramerateLimit(144);

            while (window.IsOpen) {
                window.Clear(background);
                window.SetView(new View(camera.position, camera.size));


                window.DispatchEvents();

                spriteRenderer.Render();

                canvasRenderer.Render();




                scene.Update();

                window.SetView(new View(camera.position, camera.size));


                physicsEngine.Update();



                asyncOperationManager.Update();



                if (isDebug) {
                    testingWindow.Update();
                }


                //window.Draw(
                //        new Vertex[] {
                //        new Vertex(camera.rect.position+new Vector2f(5,5)),
                //        new Vertex(camera.rect.position+new Vector2f(5,-5+camera.size.Y))
                //}, PrimitiveType.Lines);


                window.Display();

                if (!window.IsOpen) {
                    break;
                }


                _deltaTime = deltaTimeClock.Restart();
            }
        }
    }
}