using MatrixEngine.GameObjects.Components.PhysicsComponents;
using SFML.Graphics;

namespace MatrixEngine.GameObjects.Components.RenderComponents {

    public abstract class RendererComponent : Component {

        


        public int layer = 0;

        public override void Start() {

        }

        public override void Update() {
            app.renderer.AddToDrawQueue(this);
        }
        public abstract void Render(RenderTarget target);
    }
}
