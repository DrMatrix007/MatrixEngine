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


        public override void Update() {
            counter++;
            if (counter > 200) {
                gameObject.SetComponent<DebugComponent>();
            }
        }
    }
}
