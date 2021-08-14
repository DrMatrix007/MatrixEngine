using MatrixEngine.System;
using SFML.System;

namespace MatrixEngine.GameObjects.Components.PhysicsComponents {
    [RequireComponent(typeof(ColliderComponent))]
    public class RigidBodyComponent : Component {

        private Vector2f _vel = new Vector2f(0, 0);

        public Vector2f velocity
        {
            get => _vel;
            set => _vel = value;
        }

        public Vector2f gravity = new Vector2f(0, 5);

        public Vector2f velocityDrag = new Vector2f(0.7f, 0f);

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


        public bool isStatic = false;

        public override void Start() {
        }

        public override void Update() {
            if (colliderComponent.colliderType == ColliderComponent.ColliderType.Tilemap) {
                isStatic = true;
            }

            if (isStatic) {
                app.rigidBodyManager.AddColliderToFrame(this.colliderComponent);
            } else {
                app.rigidBodyManager.AddRigidbodyToFrame(this);
            }

        }
        public override string ToString() {
            return $"rigidbody: \nVelocity: {velocity.Round(2)}, \nPosition: {position.Round(2)}";
        }
    }
}
