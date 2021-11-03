using System;
using SFML.Window;

namespace MatrixEngine.ECS.Behaviors
{
    public abstract class Behavior : IDisposable
    {
        private bool _hasStarted = false;

        private Actor _actor;

        public KeyHandler GetKeyHandler() => GetActor().GetScene().GetApp().KeyHandler;

        public App GetApp() => GetActor().GetScene().GetApp();

        public Actor GetActor() => _actor ?? throw new NullReferenceException($"GetActor is null in {this}");

        public Transform GetTransform() => GetActor().Transform;

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