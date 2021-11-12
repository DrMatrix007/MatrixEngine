using MatrixEngine.MatrixMath;
using SFML.System;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.ECS.Behaviors
{
    public class RectBehavior : Behavior
    {

        private Rect _rect;

        public Rect GetRect() =>new Rect(Transform.Position,Transform.Scale.Multiply(Size));
        public Vector2f Size;

        public override void Dispose()
        {
        }

        protected override void OnStart()
        {
        }

        protected override void OnUpdate()
        {
        }
    }
}
