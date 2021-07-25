using MatrixEngine.GameObjects.Components;
using MatrixEngine.Scenes;
using SFML.System;
using System;
using System.Collections.Generic;
using System.Linq;

namespace MatrixEngine.GameObjects {
    public sealed class GameObject {


        public Vector2f position = new Vector2f(0, 0);

        private Dictionary<Type, Component> components;

        public GameObject gameObject
        {
            get {
                return this;
            }
        }

        public void SetComponent<T>() where T : Component, new() {
            SetComponent(new T());
        }
        public void SetComponent(Type type) {
            Component c = (Component)Activator.CreateInstance(type);
            SetComponent(c);


        }
        private void PureSetComponent(Component component) {

            if (component.gameObject != null) {
                System.Utils.LogError($"{component} is already stored by a gameobject!!!");
            }

            component.SetupGameobject(this);
            var t = component.GetType();
            components[t] = component;
        }
        public void SetComponent(Component component) {
            //Debug.Log($"Added {component.GetType()}");
            var requireds = component.GetType().GetCustomAttributes(typeof(RequireComponent), true);
            foreach (RequireComponent item in requireds) {
                if (GetComponent(item.type) == null) {

                    SetComponent(item.type);

                }
            }
            PureSetComponent(component);
        }

        public void SetComponents(IEnumerable<Component> comps) {

            foreach (var component in comps) {
                if (component.gameObject != null) {
                    System.Utils.LogError($"{component} is already stored by a gameobject!!!");
                }
                component.SetupGameobject(this);
                var t = component.GetType();
                components[t] = component;
            }

            foreach (var component in comps) {

                var requireds = component.GetType().GetCustomAttributes(typeof(RequireComponent), true);


                if (requireds.Length == 0) {
                    continue;
                }


                foreach (RequireComponent item in requireds) {

                    if (GetComponent(item.type) == null) {
                        SetComponent(item.type);
                    }
                }

            }



        }

        public Scene scene
        {
            get;
            private set;
        }

        internal void SetupScene(Scene scene) {
            this.scene = scene;
        }

        public T GetComponent<T>() where T : Component {
            try {
                return (T)components[typeof(T)];
            } catch (Exception) { }

            return default;
        }
        public Component GetComponent(Type t) {
            if (components.ContainsKey(t)) {

                return components[t];
            } else {
                return default;
            }

        }

        public GameObject() {
            components = new Dictionary<Type, Component>();
        }
        public GameObject(IEnumerable<Component> components) : this() {
            SetComponents(components);
        }
        public GameObject(Vector2f pos) : this() {
            position = pos;
        }

        public GameObject(Vector2f pos, IEnumerable<Component> components) : this(components) {
            position = pos;
        }
        public GameObject(Component component) : this() {
            SetComponent(component);
        }

        public void Update() {
            var l = components.Values.ToList();
            foreach (var component in l) {
                if (!component.didStart) {
                    component.didStart = true;
                    component.Start();
                }
                component.Update();

            }
        }
    }
}
