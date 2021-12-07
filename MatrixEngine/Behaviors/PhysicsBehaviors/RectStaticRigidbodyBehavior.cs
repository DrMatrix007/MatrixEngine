using System;
using MatrixEngine.MatrixMath;
using MatrixEngine.Utils;

namespace MatrixEngine.Behaviors.PhysicsBehaviors
{
    public class RectStaticRigidbodyBehavior : StaticRigidbodyBehavior
    {
        private RectBehavior RectBehavior;

        public override float GetCollidingFix(Rect dynamicStartRect, Rect dynamicEndRect, Direction dir)
        {
            return Physics.GetCollisionFix(dynamicStartRect, dynamicEndRect, RectBehavior.Rect, dir);
        }

        protected override void OnStart()
        {
            RectBehavior = GetBehavior<RectBehavior>() ?? AddBehavior(new RectBehavior());
        }

        protected override void OnUpdate()
        {
            if (!HaveBehavior<RectBehavior>())
            {
                throw new BehaviorNotFoundException(typeof(RectBehavior));
            }
        }

        public override void Dispose()
        {
        }
    }
}