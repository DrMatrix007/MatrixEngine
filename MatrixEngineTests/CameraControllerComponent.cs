
using MatrixEngine.GameObjects.Components;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;


namespace MatrixEngineTests {
    [RequireComponent(typeof(RectComponent))]
    public class CameraControllerComponent : Component {
        public CameraControllerComponent() : base() {
            
        
        }
        private RectComponent rectComponent;
        public override void Start() {
            rectComponent = GetComponent<RectComponent>();
        }

        public override void Update() {
            app.camera.position = position+rectComponent.rect.size/2;
        }
    }
}
