using SFML.Graphics;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.GameObjects.Components {
    public class DebugComponent : Component {



        int counter = 0;

        public override void Start() {
            //Debug.Log("Start Debug");
        }


        public override void Update(RenderWindow window) {
            counter++;
            //Debug.Log(counter.ToString());
            if (counter > 200) {
                gameObject.CreateComponent<DebugComponent>();
                //gameObject.scene.AddGameObject(new GameObject(new DebugComponent()));
            }
        }
    }
}
