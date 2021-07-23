using SFML.Graphics;

namespace MatrixEngine.GameObjects.Components.RenderComponents {
    public abstract class RendererComponent : Component {

        public int layer = 0;

        public override void Update() {
            app.renderer.addToDrawQueue(this);
        }
        public abstract void Render(RenderTarget drawable);
    }
}
