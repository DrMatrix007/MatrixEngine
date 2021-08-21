using MatrixEngine.GameObjects.Components;

namespace MatrixEngineTests {
    public class CameraControllerComponent : Component {
        public CameraControllerComponent() : base() {
            
        
        }
        public override void Start() {
        }

        public override void Update() {
            app.camera.position = position+transform.fullRect.size/2;
        }
    }
}
