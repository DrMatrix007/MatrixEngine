﻿using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Plugins
{
    public abstract class Plugin : IDisposable
    {
        private bool _hasStarted;

        private Scene _scene;

        public Scene GetScene() => _scene ?? throw new NullReferenceException($"GetScene is empty for {this}");

        internal void SetScene(Scene scene)
        {
            _scene = scene;
        }
        public Engine GetEngine()
        {
            return GetScene().GetEngine();
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