using System;
using System.Buffers;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.ECS.Behaviors;
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
            var k = app.KeyHandler;

            if (k.IsPressed(Keyboard.Key.D))
            {
                trans.Position += new Vector2f(speed, 0) * app.DeltaTime.AsSeconds();
            }
            if (k.IsPressed(Keyboard.Key.A))
            {
                trans.Position += new Vector2f(-speed, 0) * app.DeltaTime.AsSeconds();
            }
            if (k.IsPressed(Keyboard.Key.W))
            {
                trans.Position += new Vector2f(0, -speed) * app.DeltaTime.AsSeconds();
            }
            if (k.IsPressed(Keyboard.Key.S))
            {
                trans.Position += new Vector2f(0, speed) * app.DeltaTime.AsSeconds();
            }

            Task.Run(
                () =>
                    (1 / app.DeltaTime.AsSeconds()).Log()
            );
        }

        public override void Dispose()
        {
        }
    }
}