using MatrixEngine.MathM;
using SFML.System;
using SFML.Window;
using System;

namespace MatrixEngine.GameObjects.Components.PhysicsComponents {
    [RequireComponent(typeof(RigidBodyComponent))]
    public class SimplePlayerControllerComponent : Component {



        public float speed = 1;

        public override void Start() {
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


            rigidBodyComponent.velocity += add.Normalize();





            //var sr = GetComponent<SpriteRendererComponent>();
        }
    }
}
