using MatrixEngine.GameObjects.Components;
using MatrixEngine.GameObjects.Components.TilemapComponents;
using MatrixEngine.StateManagment;
using MatrixEngine.System;
using MatrixEngine.System.MathM;
using SFML.System;
using SFML.Window;

namespace MatrixEngineTests {
    internal class TestResizeTilemapComponent : Component {

        private readonly Provider<TilemapComponent> provider;

        public TestResizeTilemapComponent(Provider<TilemapComponent> prov) {
            this.provider = prov;
        }

        public override void Start() {
            
        }

        public override void Update() {
            
            var t = provider.Get();
            if (app.keyHandler.isPressed(Keyboard.Key.Add)) {
                t.transform.scale += new Vector2f(1, 1) * app.deltaTime;
            }
            if (app.keyHandler.isPressed(Keyboard.Key.Subtract)) {
                t.transform.scale -= new Vector2f(1, 1) * app.deltaTime;
            }
            if(t.transform.scale.X.Abs()<0.1f|| t.transform.scale.Y.Abs() < 0.1f) {
                t.transform.scale = new Vector2f(1, 1);
            }

        }
    }
}
