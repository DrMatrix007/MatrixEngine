using MatrixEngine.Physics;
using SFML.System;

namespace MatrixEngine.GameObjects.Components {
    [RequireComponent(typeof(RectComponent))]
    public class RigidBodyComponent : Component {

        public Vector2f velocity = new Vector2f(0, 0);

        public Vector2f gravity = new Vector2f(0, 0);

        public Vector2f velocityDrag = new Vector2f(0.001f, 0.001f);

        public RigidBodyComponent() {
            isStatic = false;
        }

        public RigidBodyComponent(Vector2f gravity, Vector2f velocityDrag, bool isStatic) {
            this.gravity = gravity;
            this.velocityDrag = velocityDrag;
            this.isStatic = isStatic;
        }


        public RigidBodyComponent(bool isStatic) {
            this.isStatic = isStatic;
        }

        private RectComponent rectComponent;
        public bool isStatic = false;

        public override void Start() {
            rectComponent = GetComponent<RectComponent>();
        }
        public Rect rect
        {
            get {
                return rectComponent.rect;
            }
        }
        public override void Update() {


            app.rigidBodyManager.AddToFrameComputing(this);
        }
    }
}
