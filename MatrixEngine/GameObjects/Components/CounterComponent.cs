using MatrixEngine.Utilities;
using SFML.Graphics;

namespace MatrixEngine.GameObjects.Components {
    public sealed class CounterComponent : Component {
        int c = 0;
        public override void Start() {
            Utils.Log("Start Counting!");
        }

        public override void Update() {
            Utils.Log($"Count: {c}");
            c++;
        }
    }
}
