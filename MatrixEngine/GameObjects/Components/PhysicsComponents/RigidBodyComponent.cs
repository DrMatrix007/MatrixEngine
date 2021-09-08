using MatrixEngine.Framework;
using MatrixEngine.Framework.MathM;
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

        public Vector2f gravity = new Vector2f(0, 0);

        public Vector2f velocityDrag = new Vector2f(0,0);

        public RigidBodyComponent() {
            isStatic = true;
        }

        public RigidBodyComponent(Vector2f gravity, Vector2f velocityDrag) {
            this.gravity = gravity;
            this.velocityDrag = velocityDrag;
            this.isStatic = false;
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
                app.physicsEngine.AddColliderToFrame(this.colliderComponent);
            } else {
                app.physicsEngine.AddRigidbodyToFrame(this);
            }

        }
        public override string ToString() {
            return $"rigidbody: \nVelocity: {velocity.Round(2)}, \nPosition: {position.Round(2)}";
        }
    }
}
