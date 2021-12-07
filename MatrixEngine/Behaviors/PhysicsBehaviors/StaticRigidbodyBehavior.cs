using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.Behaviors;
using MatrixEngine.MatrixMath;
using MatrixEngine.Utils;

namespace MatrixEngine.Behaviors.PhysicsBehaviors
{
    public abstract class StaticRigidbodyBehavior : Behavior
    {
        public abstract float GetCollidingFix(Rect dynamicStartRect, Rect dynamicEndRect, Direction dir);
    }
}
