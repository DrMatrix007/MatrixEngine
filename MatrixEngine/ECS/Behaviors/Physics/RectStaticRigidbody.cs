using System;
using MatrixEngine.MatrixMath;
using MatrixEngine.Utils;

namespace MatrixEngine.ECS.Behaviors.Physics
{
    public class RectStaticRigidbody : StaticRigidbodyBehavior
    {
        private RectBehavior RectBehavior;

        public override float GetCollidingFix(Rect dynamicStartRect, Rect dynamicEndRect, Direction dir)
        {
            var myRect = RectBehavior.GetRect();

            if (!dynamicEndRect.IsColliding(myRect))
            {
                return 0;
            }

            switch (dir)
            {
                case Direction.X:
                    if (dynamicEndRect.cX < myRect.cX)
                    {
                        return dynamicEndRect.max.X - myRect.X;
                    }

                    return dynamicEndRect.X - myRect.max.X;
                case Direction.Y:
                    if (dynamicEndRect.cY < myRect.cY)
                    {
                        return dynamicEndRect.max.Y - myRect.Y;
                    }

                    return dynamicEndRect.Y- myRect.max.Y;


                default:
                    throw new ArgumentOutOfRangeException(nameof(dir), dir, null);
            }
        }

        protected override void OnStart()
        {
            RectBehavior = GetBehavior<RectBehavior>() ?? AddBehavior<RectBehavior>(new RectBehavior());
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