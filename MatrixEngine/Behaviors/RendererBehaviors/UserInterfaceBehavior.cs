using SFML.Graphics;
using SFML.System;
using SFML.Window;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Behaviors.RendererBehaviors
{
    public abstract class UserInterfaceBehavior : Behavior
    {

        public UserInterfaceBehavior(int layer)
        {
            Layer = layer;
        }

        public AnchorBehavior AnchorBehavior { get; private set; }
        public int Layer;

        public abstract bool IsOverlapping(Vector2f pos);

        public bool IsActive = true;

        //public virtual void OnClick(Mouse.Button button)
        //{

        //}

        public EventHandler<Mouse.Button> OnClick = new EventHandler<Mouse.Button>((a, b) => { });


        public EventHandler<Mouse.Button> OnContinuesClick = new EventHandler<Mouse.Button>((a, b) => { });

        public EventHandler OnHover = new EventHandler((a, b) => { });
        protected override void OnStart()
        {
        }

        protected override void OnUpdate()
        {
            AnchorBehavior = GetBehavior<AnchorBehavior>()?? throw new BehaviorNotFoundException(typeof(AnchorBehavior));
        }
        public abstract void Render(RenderTarget target);

    }
}