using MatrixEngine.Physics;
using MatrixEngine.System;
using SFML.System;
using System;

namespace MatrixEngine.GameObjects.Components.PhysicsComponents {
    [RequireComponent(typeof(ColliderComponent))]
    public class RigidBodyComponent : Component {

        private Vector2f _vel = new Vector2f(0, 0);
            
        public Vector2f velocity
        {
            get { return _vel; }
            set {
                //Environment.StackTrace.Log();

                _vel = value; }
        }

        public Vector2f gravity = new Vector2f(0, 0);

        public float velocityDrag = 0.1f;

        public RigidBodyComponent() {
            isStatic = false;
        }

        public RigidBodyComponent(Vector2f gravity, float velocityDrag, bool isStatic) {
            this.gravity = gravity;
            this.velocityDrag = velocityDrag;
            this.isStatic = isStatic;
        }


        public RigidBodyComponent(bool isStatic) {
            this.isStatic = isStatic;
        }


        public bool isStatic = false;

        public override void Start() {
        }
        
        public override void Update() {
            if(colliderComponent.colliderType == ColliderComponent.ColliderType.Tilemap) {
                isStatic = true;
            }

            if (isStatic) {
                app.rigidBodyManager.AddColliderToFrame(this.colliderComponent);
            } else {
                app.rigidBodyManager.AddRigidbodyToFrame(this);
            }

        }
    }
}
