using MatrixEngine.Framework;
using MatrixEngine.GameObjects;
using MatrixEngine.Scenes.Plugins;
using MatrixEngine.UI;
using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace MatrixEngine.Scenes {

    public class Scene : IEnumerable<GameObject> {
        private bool isStarted = false;

        private List<GameObject> gameObjects;

        private List<UIObject> uiObjects;

        private List<Plugin> plugins;

        public GameObject CreateGameObject() {
            var g = new GameObject();

            return g;
        }

        public GameObject AddGameObject(GameObject gameObject) {
            gameObject.SetupScene(this);
            gameObjects.Add(gameObject);

            return gameObject;
        }

        public App app
        {
            get;
            internal set;
        }

        public Scene(IEnumerable<GameObject> gameObjects = null, IEnumerable<UIObject> uiObjects = null, List<Plugin> plugins = null) {
            this.gameObjects = new List<GameObject>();
            this.uiObjects = new List<UIObject>();
            this.plugins = new List<Plugin>();
            gameObjects ??= new List<GameObject>();
            uiObjects ??= new List<UIObject>();
            plugins ??= new List<Plugin>();
            // var l = gameObjects.ToList();
            foreach (var item in gameObjects) {
                AddGameObject(item);
            }

            foreach (UIObject uiObject in uiObjects) {
                AddUIObject(uiObject);
            }

            foreach (var item in plugins) {
                AddPlugin(item);
            }

            // uiObjects.AddRange(objects);
        }

        public void AddPlugin(Plugin plugin) {
            if (plugin == null) {
                throw new ArgumentNullException(nameof(plugin));
            }
            plugin.SetupScene(this);
            plugins.Add(plugin);
        }

        public void AddUIObject(UIObject uiObject) {
            uiObject.SetupScene(this);
            uiObjects.Add(uiObject);
        }

        public virtual void Start() {
        }

        public void Update() {
            if (!isStarted) {
                Start();
                isStarted = true;
            }

            var l = this.Where((e) => e.IsActive).ToArray();

            var p = plugins.ToList();

            foreach (var item in l) {
                item.Setup();
            }

            foreach (var item in l) {
                item.Start();
            }

            foreach (var item in p) {
                if (!item.HasStarted) {
                    item.Start();
                }
                item.HasStarted = true;
            }

            foreach (var item in l) {
                item.Update();
            }

            foreach (var item in p) {
                item.Update();
            }

            foreach (var item in l) {
                item.LateUpdate();
            }

            foreach (var item in p) {
                item.LateUpdate();
            }

            foreach (var uiObject in uiObjects) {
                app.CanvasRenderer.Add(uiObject);
            }
        }

        public IEnumerator<GameObject> GetEnumerator() {
            return gameObjects.GetEnumerator();
        }

        IEnumerator IEnumerable.GetEnumerator() {
            return GetEnumerator();
        }

        public void DestroyGameObject(GameObject gameObject) {
            gameObjects.Remove(gameObject);
        }
    }
}