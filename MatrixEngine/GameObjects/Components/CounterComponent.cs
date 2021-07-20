using SFML.Graphics;

namespace MatrixEngine.GameObjects.Components {
    public sealed class CounterComponent : Component {
        int c = 0;
        public override void Start() {
            Debug.Log("Start Counting!");
        }

        public override void Update(RenderWindow window) {
            Debug.Log($"Count: {c}");
            c++;
        }
    }
}
