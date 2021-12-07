using System;
using SFML.Window;

namespace MatrixEngine.Behaviors
{
    public abstract class Behavior : IDisposable
    {
        private bool _hasStarted = false;

        private Actor _actor;

        public InputHandler GetInputHandler() => GetActor().GetScene().GetEngine().InputHandler;

        public Engine GetEngine() => GetActor().GetScene().GetEngine();

        public Actor GetActor() => _actor ?? throw new NullReferenceException($"GetActor is null in {this}");


        public Scene GetScene() => GetActor().GetScene();

        public T AddBehavior<T>(T t) where T : Behavior => GetActor().AddBehavior(t);
        public Behavior AddBehavior(Behavior t) => GetActor().AddBehavior(t);

        public bool HaveBehavior(Type t) => GetActor().HaveBehavior(t);

        public bool HaveBehavior<T>() => GetActor().HaveBehavior<T>();



        internal void SetActor(Actor a)
        {
            _actor = a;
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