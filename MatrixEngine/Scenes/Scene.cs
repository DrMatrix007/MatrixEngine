using MatrixEngine.GameObjects;
using MatrixEngine.App;
using SFML.Graphics;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Scenes {
    public class Scene {

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
        public App.App app
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
        public void Update(RenderWindow window) {
            var list = gameObjects.ToList();
            foreach (var item in list) {
                item.Update(window);
            }
            Debug.Log(list.Count.ToString());
        }

    }
}
