using System;
using System.Buffers;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.ECS.Behaviors;
using MatrixEngine.ECS.Plugins;
using SFML.System;
using SFML.Window;

namespace MatrixEngine.ECS
{
    public class TestBehavior : Behavior
    {
        public const float speed = 10;

        protected override void OnStart()
        {
            Console.WriteLine("EZ start");
        }

        protected override void OnUpdate()
        {
            var trans = GetTransform();
            var app = GetApp();
            var k = app.InputHandler;
            var add = new Vector2f(0, 0);
            if (k.IsPressed(Keyboard.Key.D))
            {
                add += new Vector2f(1, 0);
            }
            if (k.IsPressed(Keyboard.Key.A))
            {
                add += new Vector2f(-1, 0);
            }
            if (k.IsPressed(Keyboard.Key.W))
            {
                add += new Vector2f(0, -1);
            }
            if (k.IsPressed(Keyboard.Key.S))
            {
                add += new Vector2f(0, 1);
            }

            if (!add.IsZeroZero())
            {
                add = add.Normalized() * speed * app.DeltaTime.AsSeconds();
            }
            trans.Position += add;
            //Task.Run(
            //    () =>
            //        (1 / app.DeltaTime.AsSeconds()).Log()
            //);

            var renderer = GetScene().GetPlugin<RendererPlugin>();

            renderer.Camera.Area = 2.Pow(k.ScrollY);

            renderer.Camera.Position = trans.Position;
        }

        public override void Dispose()
        {
        }
    }
}