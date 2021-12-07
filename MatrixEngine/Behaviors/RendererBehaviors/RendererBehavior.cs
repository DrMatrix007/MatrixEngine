using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using SFML.Graphics;

namespace MatrixEngine.Behaviors.RendererBehaviors
{
    public abstract class RendererBehavior : Behavior
    {
        protected override void OnStart()
        {
        }

        protected override void OnUpdate()
        {
        }

        public abstract void Render(RenderTarget target);
    }
}