using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.Physics;

namespace MatrixEngine.GameObjects.Components.PhysicsComponents {
    public sealed class ColliderComponent : Component {

        public enum ColliderType {
            None,

            Rect,

            Tilemap,

        }

        public ColliderType colliderType;

        public ColliderComponent(ColliderType colliderType) {
            this.colliderType = colliderType;
        }

        public ColliderComponent() : this(ColliderType.Rect) {

        }
        public Rect rect
        {
            get => transform.fullRect;
        }

        public override void Start() {
        }

        public override void Update() {
        }
    }
}
