using MatrixEngine.MatrixMath;
using MatrixEngine.Utils;
using SFML.System;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Behaviors.PhysicsBehaviors
{
    public class DynamicRigidbodyBehavior : Behavior
    {

        public VerticalDirections VerticalCollisionDirection { get;internal set; }
        public HorizontalDirections HorizontalCollisionDirection { get; internal set; }

        public Vector2f Velocity;

        public Vector2f Gravity;

        public Vector2f Friction;

        public RectBehavior RectBehavior { get; private set; }
        public bool IsTrigger = false;

        public EventHandler<DynamicRigidbodyBehavior> OnCollisionTrigger = new EventHandler<DynamicRigidbodyBehavior>((a, b) => { });

        public DynamicRigidbodyBehavior(Vector2f gravity, Vector2f friction,bool IsTrigger = false)
        {
            Velocity = new Vector2f();
            Gravity = gravity;
            Friction = friction;
            this.IsTrigger = IsTrigger;
        }

        public override void Dispose()
        {
        }

        protected override void OnStart()
        {
        }

        protected override void OnUpdate()
        {
            //Logging.Assert(HaveBehavior<RectBehavior>());
            RectBehavior = GetBehavior<RectBehavior>() ?? throw new BehaviorNotFoundException(typeof(RectBehavior));
        }
    }
}
