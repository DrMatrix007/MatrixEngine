using MatrixEngine.Framework.MathM;
using SFML.System;
using System;

namespace MatrixEngine.GameObjects.Components.PhysicsComponents {

    [RequireComponent(typeof(ColliderComponent))]
    public class RigidBodyComponent : Component {

        public bool TouchRight
        {
            get;
            internal set;
        }

        public bool TouchLeft
        {
            get;
            internal set;
        }

        public bool TouchUp
        {
            get;
            internal set;
        }

        public bool TouchDown
        {
            get;
            set;
        }

        internal void ClearTouches() {
            TouchDown = false;
            TouchUp = false;
            TouchRight = false;
            TouchLeft = false;
        }

        private Vector2f _vel = new(0, 0);

        public Vector2f Velocity
        {
            get => _vel;
            set => _vel = value;
        }

        public Vector2f gravity = new(0, 0);

        public Vector2f velocityDrag = new(0, 0);

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
            if (ColliderComponent.colliderType == ColliderComponent.ColliderType.Tilemap) {
                isStatic = true;
            }

            if (isStatic) {
                App.PhysicsEngine.AddColliderToFrame(this.ColliderComponent);
            } else {
                App.PhysicsEngine.AddRigidbodyToFrame(this);
            }
        }

        public override string ToString() {
            return $"rigidbody: \nVelocity: {Velocity.Round(2)}, \nPosition: {Position.Round(2)}";
        }
    }
}