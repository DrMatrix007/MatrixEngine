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
        public PhysicsEngine PhysicsEngine { get; private set; }

        public InputHandler InputHandler { get; private set; }

        public OperationManager OperationManager { get; private set; }

        public readonly string AppName;
        private readonly bool isDebug;
        public Scene scene;

        public readonly Camera camera;

        public SpriteRenderer SpriteRenderer { get; private set; }

        public RenderWindow Window { get; private set; }

        private readonly Clock timeClock = new();

        private readonly Clock deltaTimeClock = new();
        public CanvasRenderer CanvasRenderer { get; private set; }

        private Time DeltaTime { get; set; }

        public float DeltaTimeAsSeconds => DeltaTime.AsSeconds();

        public float TimeAsSeconds => timeClock.ElapsedTime.AsSeconds();

        private TestingWindow TestingWindow { get; set; }

        public void AddToDebug<T>(T obj) where T : class {
            if (isDebug) {
                TestingWindow.Add(obj);
            }
        }

        public App(string appName, bool isDebug, Scene scene) {
            var screen_size = VideoMode.DesktopMode;
            AppName = appName;
            this.isDebug = isDebug;
            this.scene = scene;
            if (isDebug) {
                Window = new RenderWindow(
                    new VideoMode(screen_size.Width - 10, (uint)(screen_size.Height * ((float)4 / 5) - 100 - 40)),
                    AppName) {
                    Position = new Vector2i()
                };
            } else {
                Window = new RenderWindow(new VideoMode(800, 600), AppName);
            }

            Window.SetKeyRepeatEnabled(false);

            Window.Closed += (s, e) => { ((Window)s)?.Close(); };

            Window.KeyPressed += Window_KeyPressed;
            Window.KeyReleased += Window_KeyReleased;

            InputHandler = new InputHandler(this);
            camera = new Camera(this);
            SpriteRenderer = new SpriteRenderer(this);
            PhysicsEngine = new PhysicsEngine(this);
            CanvasRenderer = new CanvasRenderer(this);
            OperationManager = new OperationManager();
            if (isDebug) {
                TestingWindow = new TestingWindow((4, 2));
            }
        }

        public Vector2f CameraSize => camera.Size;

        public Vector2i WindowSize => (Vector2i)Window.Size;

        private void Window_KeyReleased(object sender, KeyEventArgs e) {
            InputHandler.ReleasedKey(e.Code);
        }

        private void Window_KeyPressed(object sender, KeyEventArgs e) {
            InputHandler.PressedKey(e.Code);
        }

        public void Run() {
            timeClock.Restart();

            deltaTimeClock.Restart();

            scene.app = this;

            var background = new Color(20, 93, 160);

            //Window.SetFramerateLimit(144);

            while (Window.IsOpen) {
                Window.Clear(background);
                Window.SetView(new View(camera.position, camera.Size));

                Window.DispatchEvents();

                SpriteRenderer.Render();
                


                scene.Update();

                CanvasRenderer.Render();

                //Window.SetView(new View(camera.position, camera.Size));

                OperationManager.Update();

                PhysicsEngine.Update();

                if (isDebug) {
                    TestingWindow.Update();
                }

                //window.Draw(
                //        new Vertex[] {
                //        new Vertex(camera.rect.position+new Vector2f(5,5)),
                //        new Vertex(camera.rect.position+new Vector2f(5,-5+camera.size.Y))
                //}, PrimitiveType.Lines);

                InputHandler.Update();

                Window.Display();

                if (!Window.IsOpen) {
                    break;
                }

                DeltaTime = deltaTimeClock.Restart();
            }
        }
    }
}