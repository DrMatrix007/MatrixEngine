using SFML.Graphics;

namespace MatrixEngine.GameObjects.Components.RenderComponents {
    [RequireComponent(typeof(RectComponent))]

    public abstract class RendererComponent : Component {
        public RectComponent rectComponent
        {
            get;
            protected set;
        }
        


        public int layer = 0;

        public override void Start() {
            rectComponent = GetComponent<RectComponent>();
        }

        public override void Update() {
            app.renderer.AddToDrawQueue(this);
        }
        public abstract void Render(RenderTarget target);
    }
}
