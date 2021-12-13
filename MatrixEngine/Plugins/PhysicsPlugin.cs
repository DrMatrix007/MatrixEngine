using MatrixEngine.Behaviors.PhysicsBehaviors;
using MatrixEngine.MatrixMath;
using MatrixEngine.Utils;
using SFML.System;
using System;
using System.Buffers;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Plugins
{
    public class PhysicsPlugin : Plugin
    {
        private List<DynamicRigidbodyBehavior> DynamicRigidbodies = new List<DynamicRigidbodyBehavior>();
        private List<StaticRigidbodyBehavior> StaticRigidbodies = new List<StaticRigidbodyBehavior>();


        protected override void OnStart()
        {
        }

        protected override void OnUpdate()
        {
            var time = Stopwatch.StartNew();


            var scene = GetScene();



            DynamicRigidbodies.Clear();
            StaticRigidbodies.Clear();

            DynamicRigidbodies = scene.GetAllBehaviors<DynamicRigidbodyBehavior>().ToList();

            StaticRigidbodies = scene.GetAllBehaviorsWithPolymorphism<StaticRigidbodyBehavior>().ToList();

            foreach (var item in DynamicRigidbodies)
            {
                UpdateRigidbody(item, StaticRigidbodies);
            }
            foreach (var item in DynamicRigidbodies)
            {
                foreach (var itemToTest in DynamicRigidbodies.Where(e=>e!=item && e.IsTrigger))
                {
                    if (item.RectBehavior.Rect.IsColliding(itemToTest.RectBehavior.Rect))
                    {
                        item.OnCollisionTrigger.Invoke(this, itemToTest);
                    }
                }
            }
            //time.Elapsed.TotalSeconds.Log();
            time.Stop();
        }

        private void UpdateRigidbody(DynamicRigidbodyBehavior nonstatic,
            IReadOnlyCollection<StaticRigidbodyBehavior> staticRigidbodies)
        {
            var engine = GetEngine();

            var trans = nonstatic.RectBehavior;

            var options = new List<float>(staticRigidbodies.Count);

            var startXRect = nonstatic.RectBehavior.Rect;
            trans.Rect.Position += nonstatic.Velocity.OnlyWithX() * engine.DeltaTimeAsSeconds;
            var endXRect = nonstatic.RectBehavior.Rect;


            options.AddRange(staticRigidbodies.Select(item =>
                item.GetCollidingFix(startXRect, endXRect, Direction.X)));
            if (options.Count != 0)
            {
                var xValue = options.Aggregate((a, b) => a.Abs() > b.Abs() ? a : b);
                trans.Position -= new Vector2f(xValue, 0);
                if (xValue != 0)
                {
                    nonstatic.Velocity.X = 0;
                    nonstatic.HorizontalCollisionDirection = xValue < 0 ? HorizontalDirections.Left : HorizontalDirections.Right;
                }
            }

            options.Clear();

            var startYRect = nonstatic.RectBehavior.Rect;
            trans.Position += nonstatic.Velocity.OnlyWithY() * engine.DeltaTimeAsSeconds;


            var endYRect = nonstatic.RectBehavior.Rect;


            options.AddRange(staticRigidbodies.Select(item =>
                item.GetCollidingFix(startYRect, endYRect, Direction.Y)));
            if (options.Count != 0)
            {
                var yValue = options.Aggregate((a, b) => a.Abs() > b.Abs() ? a : b);
                trans.Position -= new Vector2f(0, yValue);
                if (yValue != 0)
                {
                    nonstatic.Velocity.Y = 0;
                    nonstatic.VerticalCollisionDirection = yValue < 0 ? VerticalDirections.Up : VerticalDirections.Down;

                }
            }

            var v = nonstatic.Velocity;
            v.X -= engine.DeltaTimeAsSeconds * v.X.Sign() * nonstatic.Friction.X;
            if (v.X.Sign() != nonstatic.Velocity.X.Sign())
            {
                v.X = 0;
            }
            v.Y -= engine.DeltaTimeAsSeconds * v.Y.Sign() * nonstatic.Friction.Y;
            if (v.Y.Sign() != nonstatic.Velocity.Y.Sign())
            {
                v.Y = 0;
            }
            nonstatic.Velocity = v;

            var g = nonstatic.Gravity * engine.DeltaTimeAsSeconds;

            nonstatic.Velocity += g;



            options.Clear();
        }
    }
}