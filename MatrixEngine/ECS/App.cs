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

    public class App
    {
        public readonly RenderWindow Window;

        private Scene _scene;

        public Scene CurrentScene
        {
            get => _scene;
            set
            {
                _scene = value ?? throw new ArgumentNullException(nameof(value));
                _scene.SetApp(this);
            }
        }

        public App(WindowSettings windowSettings, Scene scene = null)
        {
            Window = new RenderWindow(new VideoMode(windowSettings.Size.X, windowSettings.Size.Y),
                windowSettings.Name);
            Window.Closed += delegate (object sender, EventArgs args)
             {
                 ((Window)sender)?.Close();
             };

            CurrentScene = scene ?? new Scene();
        }

        public void Run()
        {
            while (Window.IsOpen)
            {
                Window.Clear(Color.Cyan);
                Window.DispatchEvents();

                CurrentScene.Update();

                Window.Display();
            }
            CurrentScene.Dispose();
            Window.Dispose();
        }
    }
}