﻿using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.Behaviors;
using MatrixEngine.Plugins;

namespace MatrixEngine
{
    public class Scene : IDisposable
    {
        internal void SetEngine(Engine app)
        {
            _app = app;
        }

        private Engine _app;

        public Engine GetEngine() => _app ?? throw new NullReferenceException($"GetEngine is null in{this}");

        private List<Actor> _actors;
        private Dictionary<Type, Plugin> _plugins;

        public Scene(List<Actor> entities = null, List<Plugin> plugins = null)
        {
            _actors = entities ?? new List<Actor>();

            _plugins = new Dictionary<Type, Plugin>();
            foreach (Plugin item in plugins ?? new List<Plugin>())
            {
                AddPlugin(item);
            }
        }

        public void Dispose()
        {
        }

        public void AddActor(Actor actor)
        {
            actor.SetScene(this);
            _actors.Add(actor);
        }

        public void AddPlugin(Plugin plugin)
        {
            if (!_plugins.ContainsKey(plugin.GetType()))
            {
                plugin.SetScene(this);
                _plugins.Add(plugin.GetType(), plugin);
            }
        }

        public IEnumerable<T> GetAllBehaviors<T>() where T : Behavior
        {
            return _actors.Select(actor => actor.GetBehavior<T>()).Where(t => t != null);
        }

        public IEnumerable<T> GetAllBehaviorsWithPolymorphism<T>() where T : Behavior
        {
            var types = typeof(T).Assembly.GetTypes().Where(t => t.IsSubclassOf(typeof(T)));

            //foreach (var actor in _actors)
            //{
            //    foreach (var type in types)
            //    {
            //        var b = actor.GetBehavior(type);
            //        if (b != null)
            //        {
            //            yield return (T)b;
            //        }
            //    }
            //}
            foreach (var b in from actor in _actors
                              from type in types
                              select actor.GetBehavior(type)
                into b
                              where b != null
                              select b)
            {
                yield return (T)b;
            }
        }

        public void Destroy(Actor actor)
        {
            _actors.Remove(actor);
        }

        public void Update()
        {
            var actors = _actors.ToArray();
            var plugins = _plugins.ToArray();
            foreach (var actor in actors)
            {
                actor.Start();
            }

            foreach (var actor in actors)
            {
                actor.Update();
            }

            foreach (var plugin in plugins)
            {
                plugin.Value.Start();
            }

            foreach (var plugin in plugins)
            {
                plugin.Value.Update();
            }
        }

        public T GetPlugin<T>() where T : Plugin
        {
            return GetPlugin(typeof(T)) as T ?? throw new NullReferenceException($"Plugin {typeof(T).Name} does not exist");
        }

        public Plugin GetPlugin(Type t)
        {
            return _plugins.GetValueOrDefault(t);
        }
    }
}