using MatrixEngine.GameObjects;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace MatrixEngine.Scenes {
    public class Scene : IEnumerable<GameObject> {

        public Scene scene
        {
            get {
                return scene;
            }
        }

        private List<GameObject> gameObjects;

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
            this.gameObjects = new List<GameObject>();
        }
        public Scene(IEnumerable<GameObject> gameObjects) {
            this.gameObjects = new List<GameObject>();
            var l = gameObjects.ToList();
            foreach (var item in l) {
                AddGameObject(item);
            }

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
        }

        public IEnumerator<GameObject> GetEnumerator() {
            return gameObjects.GetEnumerator();
        }

        IEnumerator IEnumerable.GetEnumerator() {
            return GetEnumerator();
        }
    }
}
