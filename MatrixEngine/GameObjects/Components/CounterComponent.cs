using MatrixEngine.Framework;
using SFML.Graphics;

namespace MatrixEngine.GameObjects.Components {
    public sealed class CounterComponent : Component {
        int c = 0;
        public override void Start() {
            Framework.Utils.Log("Start Counting!");
        }

        public override void Update() {
            Framework.Utils.Log($"Count: {c}");
            c++;
        }
    }
}
