using System.Collections;
using System.Collections.Generic;
using System.Linq;
using MatrixEngine.GameObjects;
using MatrixEngine.UI;
using NotImplementedException = System.NotImplementedException;

namespace MatrixEngine.System {
    public class Scene : IEnumerable<GameObject> {


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
        public System.App app
        {
            get;
            internal set;
        }

        public Scene() {
            gameObjects = new List<GameObject>();
            uiObjects = new List<UIObject>();
        }
        public Scene(IEnumerable<GameObject> gameObjects) {
            this.gameObjects = new List<GameObject>();
            uiObjects = new List<UIObject>();

            var l = gameObjects.ToList();
            foreach (var item in l) {
                AddGameObject(item);
            }


        }

        public Scene(IEnumerable<GameObject> gameobjects, IEnumerable<UIObject> objects) : this(gameobjects) {
            uiObjects.AddRange(objects);
        }
        
        public virtual void Start() {

        }
        public void Update() {
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
