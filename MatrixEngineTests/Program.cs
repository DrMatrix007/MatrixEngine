using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine;
using MatrixEngine.ECS;
using MatrixEngine.ECS.Behaviors;
using MatrixEngine.ECS.Plugins;
using SFML.Graphics;
using SFML.System;

namespace MatrixEngineTests
{
    public static class Program
    {
        public static void Main(string[] args)
        {
            var app = new App(new WindowSettings() { Name = "Tests", Size = new Vector2u(1000, 500) });

            app.CurrentScene = new Scene();

            app.CurrentScene.AddActor(new Actor(new Behavior[]
            {
                new SpriteRendererBehavior(new Texture("object.png"),16 ),
                new TestBehavior(),
            }));

            app.CurrentScene.AddActor(new Actor(new Behavior[]
            {
                new SpriteRendererBehavior(new Texture("object.png"),16 ),
            }));

            app.CurrentScene.AddPlugin(new RendererPlugin());

            app.Run();
        }
    }
}