using System;
using System.Collections.Generic;
using System.Linq;
using MatrixEngine.Behaviors;
using MatrixEngine.Utils;
using SFML.System;

namespace MatrixEngine
{
    public class Actor : IDisposable
    {
        //public readonly Transform Transform = new Transform();

        private Scene _scene;

        public Scene GetScene() => _scene ?? throw new NullReferenceException($"GetScene is empty for {this}");

        internal void SetScene(Scene scene)
        {
            this._scene = scene;
        }

        public bool HaveBehavior(Type t)
        {
            return behaviors.ContainsKey(t);
        }

        public bool HaveBehavior<T>()
        {
            return HaveBehavior(typeof(T));
        }


        public Actor(IEnumerable<Behavior> behaviors)
        {
            foreach (var behavior in behaviors)
            {
                AddBehavior(behavior);
            }
        }
        public Actor(params Behavior[] behaviors) : this(behaviors as IEnumerable<Behavior>)
        {

        }


        private Dictionary<Type, Behavior> behaviors = new Dictionary<Type, Behavior>();

        public Behavior AddBehavior(Behavior behavior)
        {
            if (behavior == null)
            {
                throw new ArgumentNullException(nameof(behavior));
            }
            behavior.GetType().Log();
            behavior.SetActor(this);
            behaviors[behavior.GetType()] = behavior;
            return behavior;
        }

        public T AddBehavior<T>(T b) where T : Behavior
        {
            return (T)AddBehavior(b as Behavior);
        }

        public T GetBehavior<T>() where T : Behavior
        {
            return (T)GetBehavior(typeof(T));
        }

        public void Destroy(Behavior behavior)
        {
            behaviors.Remove(behavior.GetType());
        }

        public void Destroy()
        {
            GetScene().Destroy(this);
        }

        public Behavior GetBehavior(Type t)
        {
            return behaviors.GetValueOrDefault(t);
            //return !behaviors.ContainsKey(t) ? null : behaviors[t];
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