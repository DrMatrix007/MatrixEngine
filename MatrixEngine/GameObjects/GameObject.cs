using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.GameObjects.Components;
using MatrixEngine.Scenes;
using SFML.Graphics;

namespace MatrixEngine.GameObjects {
    public sealed class GameObject {

        private Dictionary<Type,Component> components;

        public GameObject gameObject
        {
            get {
                return this;
            }
        }

        public void CreateComponent<T>() where T : Component,new() {
            SetComponent(new T());
        }
        public void SetComponent(Component component)  {
            //Debug.Log($"Added {component.GetType()}");
            component.SetupGameobject(this);
            components[component.GetType()] = component;
        }

        public Scene scene
        {
            get;
            private set;
        }

        internal void SetupScene(Scene scene) {
           this.scene = scene;
        }

        public T GetComponent<T>() where T:Component {
            try {
                return (T)components[typeof(T)];
            } catch (Exception ex) { }
            
            return default;
        }

        public GameObject() {
            components = new Dictionary<Type, Component>();
        }
        public GameObject(IEnumerable<Component> components) : this() {
            foreach (var item in components) {
                try {
                    SetComponent(item);
                } catch (Exception ex) {
                }
            }
        }
        public GameObject(Component component) : this() {
            SetComponent(component);
        }

        public void Update(RenderWindow window) {
            foreach (var component in components.Values) {
                if (!component.didStart) {
                    component.didStart = true;
                    component.Start();
                }
                component.Update(window);

            }
        }
    }
}
