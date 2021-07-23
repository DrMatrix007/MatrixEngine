using MatrixEngine.GameObjects.Components.RenderComponents;

namespace MatrixEngine.GameObjects.Components.UIComponents {
    public abstract class UIRendererComponent : RendererComponent {


        public override void Start() {
        }

        public override void Update() {
            app.canvasRenderer.Add(this);

        }
    }
}
