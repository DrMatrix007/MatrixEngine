using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.ECS
{
    public abstract class Plugin : IDisposable
    {
        private bool _hasStarted;

        private Scene _scene;

        public Scene Scene => _scene ?? throw new NullReferenceException($"Scene is empty for {this}");

        internal void SetScene(Scene scene)
        {
            this._scene = scene;
        }

        public void Dispose()
        {
        }

        protected abstract void OnUpdate();

        protected abstract void OnStart();

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
    }
}