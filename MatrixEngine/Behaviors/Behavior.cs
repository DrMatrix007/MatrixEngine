using System;
using SFML.Window;

namespace MatrixEngine.ECS.Behaviors
{
    public abstract class Behavior : IDisposable
    {
        private bool _hasStarted = false;

        private Actor _actor;

        public InputHandler GetInputHandler() => GetActor().GetScene().GetEngine().InputHandler;

        public Engine GetEngine() => GetActor().GetScene().GetEngine();

        public Actor GetActor() => _actor ?? throw new NullReferenceException($"GetActor is null in {this}");

        public Transform GetTransform() => GetActor().Transform;

        public Scene GetScene() => GetActor().GetScene();

        public T AddBehavior<T>(T t) where T:Behavior => GetActor().AddBehavior<T>(t);
        public Behavior AddBehavior(Behavior t) => GetActor().AddBehavior(t);

        public bool HaveBehavior(Type t) => GetActor().HaveBehavior(t);

        public bool HaveBehavior<T>() => GetActor().HaveBehavior<T>();

        public Transform Transform { get;private set; }

        internal void SetActor(Actor a)
        {
            _actor = a;
            Transform = GetTransform();
        }

        public T GetBehavior<T>() where T : Behavior
        {
            return GetActor().GetBehavior<T>();
        }

        public void Start()
        {
            if (_hasStarted) return;

            OnStart();
            _hasStarted = true;
        }

        public void Update()
        {
            OnUpdate();
        }

        protected abstract void OnStart();

        protected abstract void OnUpdate();

        public abstract void Dispose();

        public void Destory()
        {
            GetActor().Destroy(this);
        }


    }
}