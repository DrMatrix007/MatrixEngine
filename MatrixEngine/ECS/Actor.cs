using System;
using System.Collections.Generic;
using System.Linq;
using MatrixEngine.ECS.Behaviors;

namespace MatrixEngine.ECS
{
    public class Actor : IDisposable
    {
        public readonly Transform Transform = new Transform();

        private Scene _scene;

        public Scene Scene => _scene ?? throw new NullReferenceException($"Scene is empty for {this}");

        internal void SetScene(Scene scene)
        {
            this._scene = scene;
        }

        public Actor(IEnumerable<Behavior> behaviors)
        {
            foreach (var behavior in behaviors)
            {
                AddBehavior(behavior);
            }
        }

        private Dictionary<Type, Behavior> behaviors = new Dictionary<Type, Behavior>();

        public void AddBehavior(Behavior behavior)
        {
            behavior.GetType().Log();
            behavior.SetActor(this);
            behaviors[behavior.GetType()] = behavior;
        }

        public T GetBehavior<T>() where T : Behavior
        {
            return GetBehavior(typeof(T)) as T;
        }

        public Behavior GetBehavior(Type t)
        {
            return behaviors[t];
        }

        public void Dispose()
        {
        }

        public void Start()
        {
            foreach (var component in behaviors.ToArray())
            {
                component.Value.Start();
            }
        }

        public void Update()
        {
            foreach (var component in behaviors.ToArray())
            {
                component.Value.Update();
            }
        }
    }
}