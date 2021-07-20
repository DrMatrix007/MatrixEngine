using MatrixEngine.Scenes;
using SFML.Graphics;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.GameObjects.Components {
    public abstract class Component {
        
        public GameObject gameObject
        {
            private set;
            get;
        }
        public Scene scene
        {
            get {
                return gameObject.scene;
            }
        }


        internal bool didStart
        {
            get;
            set;


        } = false;

        public Component() {
        }
        internal void SetupGameobject(GameObject gameObject) {
            this.gameObject = gameObject;
        }
        



        abstract public void Start();
        abstract public void Update(RenderWindow window);
    }
}
