using MatrixGDK.System;
using SFML.Graphics;

namespace MatrixGDK.GameObjects.Components {
    public sealed class CounterComponent : Component {
        int c = 0;
        public override void Start() {
            System.Utils.Log("Start Counting!");
        }

        public override void Update() {
            System.Utils.Log($"Count: {c}");
            c++;
        }
    }
}
