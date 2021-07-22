using MatrixEngine.MathM;
using SFML.System;
using SFML.Window;
using System;

namespace MatrixEngine.GameObjects.Components {
    [RequireComponent(typeof(RigidBodyComponent))]
    public class SimplePlayerControllerComponent : Component {


        RigidBodyComponent rigidBodyComponent;

        public float speed = 1;

        public override void Start() {
            rigidBodyComponent = GetComponent<RigidBodyComponent>();
        }

        public override void Update() {
            var add = new Vector2f(0, 0);

            if (keyHandler.isPressed(Keyboard.Key.A)) {
                add.X = -speed;
            }
            if (keyHandler.isPressed(Keyboard.Key.D)) {
                add.X = speed;
            }
            if (keyHandler.isPressed(Keyboard.Key.W)) {
                add.Y = -speed;
            }
            if (keyHandler.isPressed(Keyboard.Key.S)) {
                add.Y = speed;
            }
            if (rigidBodyComponent.velocity.Length() > speed) {
                var vec = rigidBodyComponent.velocity;
                vec.X = Math.Clamp(vec.X, -speed * 100, speed * 100);
                vec.Y = Math.Clamp(vec.Y, -speed * 100, speed * 100);
                rigidBodyComponent.velocity = vec;
            }

            rigidBodyComponent.velocity += add.Normalize() * app.deltaTime * 100;






            //var sr = GetComponent<SpriteRendererComponent>();
        }
    }
}
