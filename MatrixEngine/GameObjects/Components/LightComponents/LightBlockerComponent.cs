using MatrixEngine.GameObjects.Components.PhysicsComponents;
using MatrixEngine.System;
using NotImplementedException = System.NotImplementedException;

namespace MatrixEngine.GameObjects.Components.LightComponents {
    [RequireComponent(typeof(ColliderComponent))]
    public class LightBlockerComponent : Component {

        
        public override void Start() {
        }

        public override void Update() {
            app.lightRenderer.AddToBlockerComponents(this);
        }
    }
}