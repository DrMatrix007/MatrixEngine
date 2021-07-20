using SFML.Graphics;
using SFML.Window;
using SFML.System;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.Scenes;
using MatrixEngine.GameObjects;
using MatrixEngine.GameObjects.Components;

namespace MatrixEngine.App {
    public sealed class App {


        public KeyHandler keyHandler;

        public readonly string AppName;

        public Scene scene;

        public App(string appName,Scene scene) {
            this.scene = scene;
            window = new RenderWindow(new VideoMode(800, 600), appName);
            window.SetKeyRepeatEnabled(true);

            window.Closed += (s, e) => {
                ((Window)s).Close();
            };

            window.KeyPressed += Window_KeyPressed;
            window.KeyReleased += Window_KeyReleased;
            AppName = appName;

            keyHandler = new KeyHandler();
        }

        private void Window_KeyReleased(object sender, KeyEventArgs e) {
            keyHandler.ReleasedKey(e.Code);
        }

        private void Window_KeyPressed(object sender, KeyEventArgs e) {
            keyHandler.PressedKey(e.Code);
        }

        private RenderWindow window;

        public void Run() {



            while (window.IsOpen) {
                window.SetView(new View(new Vector2f(0, 0), new Vector2f(window.Size.X, window.Size.Y)));

                scene.Update(window);

                scene.app = this;

                window.DispatchEvents();

                window.Clear(Color.Black);

                



                window.Display();

            }


        }


    }
}
