using MatrixGDK.GameObjects.Components.PhysicsComponents;
using MatrixGDK.Physics;
using MatrixGDK.Scenes;
using MatrixGDK.System;
using SFML.System;

namespace MatrixGDK.GameObjects.Components {
    public abstract class Component {

        public GameObject gameObject
        {
            private set;
            get;
        }

        public Vector2f position
        {
            get {
                return gameObject.position;
            }
            set {
                gameObject.position = value;
            }
        }

        public Scene scene
        {
            get {
                return gameObject.scene;
            }
        }
        public System.App app
        {
            get {
                return scene.app;
            }
        }
        public KeyHandler keyHandler
        {
            get {
                return app.keyHandler;
            }
        }

        public RigidBodyComponent rigidBodyComponent
        {
            get {
                return GetComponent<RigidBodyComponent>();
            }
        }
        public ColliderComponent colliderComponent
        {
            get {
                return GetComponent<ColliderComponent>();
            }
        }

        public TransformComponent transform
        {
            get => gameObject.transform;
        }




        public T GetComponent<T>() where T : Component {
            return gameObject.GetComponent<T>();
        }

        internal bool didStart
        {
            get;
            set;


        } = false;

        public Component() {
        }
        internal void SetupGameobject(GameObject gameObject) {
            this.gameObject = gameObject;
        }




        abstract public void Start();
        abstract public void Update();

        public virtual void LateUpdate() {

        }

        public virtual void Setup() {
        }
    }
}
