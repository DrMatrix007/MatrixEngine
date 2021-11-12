using MatrixEngine.ECS.Behaviors.Physics;
using MatrixEngine.MatrixMath;
using SFML.System;
using System;
using System.Buffers;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.ECS.Plugins
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
            var scene = GetScene();

            var engine = GetEngine();

            DynamicRigidbodies.Clear();
            StaticRigidbodies.Clear();

            DynamicRigidbodies = scene.GetAllBehaviors<DynamicRigidbodyBehavior>().ToList();

            StaticRigidbodies = scene.GetAllBehaviorsWithPolymorphism<StaticRigidbodyBehavior>().ToList();

            foreach (var item in DynamicRigidbodies)
            {
                UpdateRigidbody(item, StaticRigidbodies);
            }
        }

        private void UpdateRigidbody(DynamicRigidbodyBehavior nonstatic,
            IReadOnlyCollection<StaticRigidbodyBehavior> staticRigidbodies)
        {
            var engine = GetEngine();

            var trans = nonstatic.GetTransform();

            var g = nonstatic.Gravity * engine.DeltaTimeAsSeconds;

            nonstatic.Velocity += g * engine.DeltaTimeAsSeconds;

            var options = new List<float>(staticRigidbodies.Count);

            var startXRect = nonstatic.RectBehavior.GetRect();
            trans.Position += nonstatic.Velocity.OnlyWithX() * engine.DeltaTimeAsSeconds;
            var endXRect = nonstatic.RectBehavior.GetRect();


            options.AddRange(staticRigidbodies.Select(item =>
                item.GetCollidingFix(startXRect, endXRect, Utils.Direction.X)));
            if (options.Count != 0)
            {
                var xValue = options.Aggregate((a, b) => a.Abs() < b.Abs() ? a : b);
                nonstatic.Transform.Position -= new Vector2f(xValue, 0);
            }

            options.Clear();

            var startYRect = nonstatic.RectBehavior.GetRect();
            trans.Position += nonstatic.Velocity.OnlyWithY() * engine.DeltaTimeAsSeconds;
            var endYRect = nonstatic.RectBehavior.GetRect();


            options.AddRange(staticRigidbodies.Select(item =>
                item.GetCollidingFix(startYRect, endYRect, Utils.Direction.Y)));
            if (options.Count != 0)
            {
                var yValue = options.Aggregate((a, b) => a.Abs() < b.Abs() ? a : b);
                nonstatic.Transform.Position -= new Vector2f(0, yValue);
            }

            options.Clear();
        }
    }
}