using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.ECS;
using MatrixEngine.ECS.Behaviors;
using MatrixEngine.ECS.Plugins;
using MatrixEngine.Utils;
using SFML.Graphics;
using SFML.System;

namespace MatrixEngineTests
{
    public static partial class Program
    {
        public static void Main1(string[] args)
        {
            var app = new Engine(new WindowSettings() { Name = "Tests", Size = new Vector2u(1000, 500) });

            app.CurrentScene = new Scene();

            app.CurrentScene.AddActor(new Actor(new Behavior[]
            {
                new SpriteRendererBehavior(new Texture("object.png"), 16),
                new TestBehavior(),
            }));
            app.CurrentScene.AddActor(new Actor(new Behavior[]
            {
                new SpriteRendererBehavior(new Texture("object.png"), 16),
            }));

            var t = new TilemapBehavior();

            var tex = new Texture("object.png");

            var ran = new MatrixRandom(100);

            var s = 20;
            var d = 100;

            var perlin = new PerlinNoise2D(s, d, MatrixRange.ZeroToOne, 0);

            Logging.LogTime(perlin.Generate);

            for (int i = 0; i < s * d; i++)
            {
                for (int j = 0; j < s * d; j++)
                {
                    if (perlin.floats[i, j] > 0.5f || j * i == 0 || j == s * d - 1 || i == s * d - 1)
                    {
                        t.SetTile(new Vector2i(i, j), new Tile(tex));
                    }
                }

                i.Log();
            }

            app.CurrentScene.AddActor(new Actor(new Behavior[]
            {
                t,
                new TilemapRendererBehavior(16)
            }));

            app.CurrentScene.AddPlugin(new RendererPlugin());

            app.Run();
        }
    }
}