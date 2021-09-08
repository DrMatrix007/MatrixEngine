using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Xml;
using MatrixEngine.GameObjects;
using MatrixEngine.UI;
using NotImplementedException = System.NotImplementedException;

namespace MatrixEngine.Framework {

    public class Scene : IEnumerable<GameObject> {
        private bool isStarted = false;

        private List<GameObject> gameObjects;

        private List<UIObject> uiObjects;

        public GameObject CreateGameObject() {
            var g = new GameObject();

            return g;
        }

        public GameObject AddGameObject(GameObject gameObject) {
            gameObject.SetupScene(this);
            gameObjects.Add(gameObject);

            return gameObject;
        }

        public Framework.App app
        {
            get;
            internal set;
        }

        public Scene(IEnumerable<GameObject> gameObjects = null, IEnumerable<UIObject> uiObjects = null) {
            this.gameObjects = new List<GameObject>();
            this.uiObjects = new List<UIObject>();
            gameObjects ??= new List<GameObject>();
            uiObjects ??= new List<UIObject>();

            // var l = gameObjects.ToList();
            foreach (var item in gameObjects) {
                AddGameObject(item);
            }

            foreach (UIObject uiObject in uiObjects) {
                AddUIObject(uiObject);
            }

            // uiObjects.AddRange(objects);
        }

        private void AddUIObject(UIObject uiObject) {
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

            var l = this.ToArray();

            foreach (var item in l) {
                item.Setup();
            }

            foreach (var item in l) {
                item.Start();
            }

            foreach (var item in l) {
                item.Update();
            }

            foreach (var item in l) {
                item.LateUpdate();
            }

            foreach (var uiObject in uiObjects) {
                app.canvasRenderer.Add(uiObject);
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