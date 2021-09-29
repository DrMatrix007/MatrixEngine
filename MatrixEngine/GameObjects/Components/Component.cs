using MatrixEngine.GameObjects.Components.PhysicsComponents;
using MatrixEngine.Framework;
using MatrixEngine.Physics;
using SFML.System;
using MatrixEngine.Framework.Operations;

namespace MatrixEngine.GameObjects.Components {

    public abstract class Component {

        public OperationManager OperationManager
        {
            get => App.OperationManager;
        }

        public PhysicsEngine PhysicsEngine => App.PhysicsEngine;

        public GameObject GameObject
        {
            private set;
            get;
        }

        public Vector2f Position
        {
            get => GameObject.Position;

            set {
                GameObject.Position = value;
            }
        }

        public Scene Scene
        {
            get {
                return GameObject.Scene;
            }
        }

        public Framework.App App
        {
            get {
                return Scene.app;
            }
        }

        public InputHandler InputHandler
        {
            get {
                return App.InputHandler;
            }
        }

        public RigidBodyComponent RigidBodyComponent
        {
            get {
                return GetComponent<RigidBodyComponent>();
            }
        }

        public ColliderComponent ColliderComponent
        {
            get {
                return GetComponent<ColliderComponent>();
            }
        }

        public TransformComponent Transform
        {
            get => GameObject.Transform;
        }

        public T GetComponent<T>() where T : Component {
            return GameObject.GetComponent<T>();
        }

        public T SetComponent<T>() where T : Component, new() {
            return GameObject.SetComponent<T>();
        }

        public T SetComponent<T>(T c) where T : Component {
            return (T)GameObject.SetComponent(c);
        }

        internal bool DidStart
        {
            get;
            set;
        } = false;

        public Component() {
        }

        internal void SetupGameobject(GameObject gameObject) {
            this.GameObject = gameObject;
        }

        abstract public void Start();

        abstract public void Update();

        public virtual void LateUpdate() {
        }

        public virtual void Setup() {
        }

        public void Destroy() {
            GameObject.DestroyComponent(this);
        }
    }
}