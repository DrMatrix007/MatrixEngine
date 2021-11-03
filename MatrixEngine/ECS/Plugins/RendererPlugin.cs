using System;
using System.Collections.Generic;
using System.Configuration.Assemblies;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.ECS.Behaviors;
using SFML.Graphics;
using SFML.System;
using SFML.Window;

namespace MatrixEngine.ECS.Plugins
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
            var window = Scene.GetApp().Window;

            var ratio = ((float)window.Size.X) / window.Size.Y;
            var size = new Vector2f((Camera.Area * ratio).Sqrt(), (Camera.Area / ratio).Sqrt());
            window.SetView(new View(Camera.Position, size));
            foreach (var behavior in Scene.GetAllBehaviorsWithPolymorphism<RendererBehavior>())
            {
                behavior.Render(window);
            }
        }
    }
}