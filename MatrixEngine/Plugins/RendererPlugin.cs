using System;
using System.Collections.Generic;
using System.Configuration.Assemblies;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.Behaviors.RendererBehaviors;
using MatrixEngine.MatrixMath;
using MatrixEngine.Utils;
using SFML.Graphics;
using SFML.System;
using SFML.Window;

namespace MatrixEngine.Plugins
{

    public struct Camera
    {
        public Camera(float area = 100) : this(area, new Vector2f())
        {
        }

        public Camera(float area, Vector2f vector2)
        {
            Area = area;
            Position = vector2;
        }

        public float Area;

        public Vector2f Position;
    }

    public class RendererPlugin : Plugin
    {
        public Camera Camera = new Camera(100);

        protected override void OnStart()
        {
            //GetScene.GetApp.Window.Resized += Window_Resized;
        }

        //private void Window_Resized(object sender, SFML.Window.SizeEventArgs e)
        //{
        //    var window = sender as Window;

        //    UpdateCamera();

        //}

        private void UpdateCamera()
        {
        }

        protected override void OnUpdate()
        {

            RenderWindow window;
            float ratio;
            Vector2f size;

            Vector2f mousePos;

            View newView;
            Engine engine;
            
            window = GetScene().GetEngine().Window;

            engine = GetEngine();

            ratio = (float)window.Size.X / window.Size.Y;
            size = new Vector2f((Camera.Area * ratio).Sqrt(), (Camera.Area / ratio).Sqrt());
            window.SetView(new View(Camera.Position, size));
            foreach (var behavior in GetScene().GetAllBehaviorsWithPolymorphism<RendererBehavior>())
            {
                behavior.Render(window);
            }

            
            newView = new View(new Vector2f(window.Size.X/2, window.Size.Y/2),(Vector2f) window.Size);
            window.SetView(newView);

            mousePos = window.MapPixelToCoords(Mouse.GetPosition(window));

            foreach (var behavior in GetScene().GetAllBehaviorsWithPolymorphism<UserInterfaceBehavior>().Where(a=>a.IsActive))
            {
                behavior.Render(window);
                if (behavior.IsOverlapping(mousePos))
                {
                    foreach (var click in engine.InputHandler.GetAllPressedDownMouseButtons())
                    {
                        behavior.OnClick.Invoke(this, click);
                    }
                    foreach (var click in engine.InputHandler.GetAllPressedMouseButtons())
                    {
                        behavior.OnContinuesClick.Invoke(this, click);
                    }
                    behavior.OnHover.Invoke(this, EventArgs.Empty);
                }
            }

            window.SetView(new View(Camera.Position, size));

            newView.Dispose();
            //window.GetView().Size.Log();

            


        }
    }
}