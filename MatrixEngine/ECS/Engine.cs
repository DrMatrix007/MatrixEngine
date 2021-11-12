using SFML.Graphics;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Microsoft.VisualBasic;
using SFML.System;
using SFML.Window;

namespace MatrixEngine.ECS
{
    public struct WindowSettings
    {
        public string Name;

        public Vector2u Size;
    }

    public class Engine
    {
        public readonly RenderWindow Window;

        public readonly InputHandler InputHandler;

        private Scene _scene;

        public Time DeltaTime { get; private set; }

        public float DeltaTimeAsSeconds { get; private set; }

        public Time Time { get; private set; }

        public Scene CurrentScene
        {
            get => _scene;
            set
            {
                _scene = value ?? throw new ArgumentNullException(nameof(value));
                _scene.SetEngine(this);
            }
        }

        public Engine(WindowSettings windowSettings, Scene scene = null)
        {
            Window = new RenderWindow(new VideoMode(windowSettings.Size.X, windowSettings.Size.Y),
                windowSettings.Name);
            Window.Closed += delegate (object sender, EventArgs args)
             {
                 ((Window)sender)?.Close();
             };

            Window.SetKeyRepeatEnabled(false);

            CurrentScene = scene ?? new Scene();

            InputHandler = new InputHandler();

            Window.KeyPressed += InputHandler.WindowKeyPressed;
            Window.KeyReleased += InputHandler.WindowKeyReleased;
            Window.MouseWheelScrolled += InputHandler.Window_MouseWheelScrolled;

            Window.SetActive(false);

        }

        public void Run()
        {
            var dc = new Clock();
            var tc = new Clock();
            
            
            Window.ResetGLStates();

            Window.SetActive(true);
            while (Window.IsOpen)
            {

                Window.Clear(Color.Cyan);
                Window.DispatchEvents();

                CurrentScene.Update();

                InputHandler.Update();

                Window.Display();
                DeltaTime = dc.Restart();
                DeltaTimeAsSeconds = DeltaTime.AsSeconds();
                Time = tc.ElapsedTime;

            }
            CurrentScene.Dispose();
            Window.Dispose();
        }
    }
}