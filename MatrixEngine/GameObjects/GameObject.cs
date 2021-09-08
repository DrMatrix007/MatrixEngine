using MatrixEngine.Framework;
using MatrixEngine.GameObjects.Components;
using MatrixEngine.Physics;
using MatrixEngine.Utilities;
using SFML.System;
using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace MatrixEngine.GameObjects {
    public sealed class GameObject : IEnumerable<Component> {


        public Vector2f position
        {
            get => transform.position;
            set => transform.position = value;
        }
        public Vector2f scale
        {
            get => transform.scale;
            set => transform.scale = value;
        }
        public Rect rect
        {
            get => transform.rect;
        }

        private Dictionary<Type, Component> components;

        public TransformComponent transform
        {
            get;
            private set;
        }

        public GameObject gameObject
        {
            get => this;
        }

        public T SetComponent<T>() where T : Component, new() {
            return (T)SetComponent(new T());
        }
        public Component SetComponent(Type type) {
            Component c = (Component)Activator.CreateInstance(type);
            return SetComponent(c);


        }
        private Component PureSetComponent(Component component) {

            if (component.gameObject != null) {
                Utils.LogError($"{component} is already stored by a gameobject!!!");
            }

            component.SetupGameobject(this);
            var t = component.GetType();
            components[t] = component;

            return component;
        }
        public Component SetComponent(Component component) {
            //Debug.Log($"Added {component.GetType()}");
            var requireds = component.GetType().GetCustomAttributes(typeof(RequireComponent), true);
            foreach (RequireComponent item in requireds) {
                if (GetComponent(item.type) == null) {

                    SetComponent(item.type);

                }
            }
            return PureSetComponent(component);
        }

        public void SetComponents(IEnumerable<Component> comps) {

            foreach (var component in comps) {
                if (component.gameObject != null) {
                    Utils.LogError($"{component} is already stored by a gameobject!!!");
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
            transform = new TransformComponent();
        }
        public GameObject(IEnumerable<Component> components) : this() {
            SetComponents(components);
        }

        public GameObject(params Component[] components) : this(components as IEnumerable<Component>) {

        }
        public GameObject(Vector2f pos, params Component[] components) : this(components) {
            position = pos;
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

        public void Setup() {
            foreach (var component in this.ToArray()) {
                if (!component.didStart) {
                    component.Setup();
                }
            }
        }

        public void Start() {

            foreach (var component in this.ToArray()) {
                if (!component.didStart) {
                    component.didStart = true;
                    component.Start();
                }
            }
        }

        public void Update() {

            foreach (var component in this.ToArray()) {
                component.Update();
            }


        }
        public void LateUpdate() {
            foreach (var item in this.ToArray()) {
                item.LateUpdate();
            }
        }

        public IEnumerator<Component> GetEnumerator() {
            var l = components.Values.ToList();
            foreach (var item in l) {
                yield return item;
            }
        }

        IEnumerator IEnumerable.GetEnumerator() {
            return GetEnumerator();
        }

        public void DestroyComponent(Component component) {
            components.Remove(component.GetType());
        }

        public void Destroy() {
            scene.DestroyGameObject(this);
        }
    }
}
