using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.ECS.Plugins;
using MatrixEngine.MatrixMath;
using MatrixEngine.Utils;

namespace MatrixEngine.ECS.Behaviors.PhysicsBehaviors
{
    public abstract class StaticRigidbodyBehavior : Behavior
    {
        public abstract float GetCollidingFix(Rect dynamicStartRect,Rect dynamicEndRect,Direction dir);
    }
}
