using System;

namespace MatrixEngine.ECS.Behaviors
{
    public abstract class Behavior : IDisposable
    {
        private bool _hasStarted = false;

        private Actor _actor;

        public Actor Actor => _actor ?? throw new NullReferenceException($"Actor is null in {this}");

        public Transform Transform => Actor.Transform;

        internal void SetActor(Actor a)
        {
            _actor = a;
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
    }
}