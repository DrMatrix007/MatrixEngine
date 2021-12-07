using System;
using System.Buffers;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.Behaviors.PhysicsBehaviors;
using MatrixEngine.MatrixMath;
using MatrixEngine.Plugins;
using MatrixEngine.Utils;
using SFML.System;
using SFML.Window;


namespace MatrixEngine.Behaviors
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
            var trans =GetBehavior<RectBehavior>();
            var app = GetEngine();
            var k = app.InputHandler;
            float x = 0;
            float y = 0;
            if (k.IsPressed(Keyboard.Key.D))
            {
                x += 1;
            }
            if (k.IsPressed(Keyboard.Key.K))
            {
                y -= 0.01f;
            }

            if (k.IsPressed(Keyboard.Key.A))
            {
                x -= 1;
            }

            if (k.IsPressed(Keyboard.Key.W))
            {
                y -= 1;
            }

            if (k.IsPressed(Keyboard.Key.S))
            {
                y += 1;
            }


            var add = new Vector2f(x, y) * speed;
            x = add.X;
            y = add.Y;
            GetBehavior<DynamicRigidbodyBehavior>().Velocity.X = x;

            if (y != 0)
            {
                GetBehavior<DynamicRigidbodyBehavior>().Velocity.Y = y;
            }


            //Task.Run(
            //    () =>
            //        (1 / app.DeltaTime.AsSeconds()).Log()
            //);

            var renderer = GetScene().GetPlugin<RendererPlugin>();


            renderer.Camera.Area = 2.Pow(k.ScrollY);

            renderer.Camera.Position = trans.Position;

            //Console.SetCursorPosition(0, Console.CursorTop - 2);
            //$"FPS: {1 / GetEngine().DeltaTimeAsSeconds}\r".Log();
            //$"Position: {trans.Position}; Velocity:{GetBehavior<DynamicRigidbodyBehavior>().Velocity}".Log();
        }

        public override void Dispose()
        {
        }
    }
}