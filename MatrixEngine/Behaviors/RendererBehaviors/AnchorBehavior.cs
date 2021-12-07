using SFML.System;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Behaviors.RendererBehaviors
{
    public class AnchorBehavior : Behavior
    {
        public Vector2f Position = new Vector2f();

        public Vector2f Size = new Vector2f();

        public AnchorBehavior(Vector2f position, Vector2f size)
        {
            Position = position;
            Size = size;
        }

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
