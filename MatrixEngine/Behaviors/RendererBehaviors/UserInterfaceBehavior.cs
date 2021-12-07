using SFML.Graphics;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Behaviors.RendererBehaviors
{
    public abstract class UserInterfaceBehavior : Behavior
    {
        public AnchorBehavior AnchorBehavior { get; private set; }



        protected override void OnStart()
        {
        }

        protected override void OnUpdate()
        {
            AnchorBehavior = GetBehavior<AnchorBehavior>();
        }
        public abstract void Render(RenderTarget target);
    }
}