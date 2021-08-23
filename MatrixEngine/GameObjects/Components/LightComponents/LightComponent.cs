using MatrixEngine.Renderers;
using NotImplementedException = System.NotImplementedException;

namespace MatrixEngine.GameObjects.Components.LightComponents {
    public abstract class LightComponent:  Component {

        internal abstract LightRenderer.LightType lightType {
            get;
        }
        
        public override void Start() {
        }

        public override void Update() {
            app.lightRenderer.AddToLightComponents(this);

        }
    }
}