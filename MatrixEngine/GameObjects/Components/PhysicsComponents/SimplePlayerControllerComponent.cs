using MatrixEngine.System;
using SFML.System;
using SFML.Window;

namespace MatrixEngine.GameObjects.Components.PhysicsComponents {
    [RequireComponent(typeof(RigidBodyComponent))]
    public class SimplePlayerControllerComponent : Component {



        public float speed = 1;

        public override void Start() {
        }

        public override void Update() {
            var x = (float)0;
            var y = (float)0;
            var isx = false;
            var isy = false;
            if (keyHandler.isPressed(Keyboard.Key.A)) {
                x = -speed;
                isx = true;
            }
            if (keyHandler.isPressed(Keyboard.Key.D)) {
                x = speed;
                isx = true;

            }
            if (keyHandler.isPressed(Keyboard.Key.W)) {
                y = -speed;
                isy = true;
            }
            if (keyHandler.isPressed(Keyboard.Key.S)) {
                y = speed;
                isy = true;

            }
            //add = new Vector2f(add.X == 0 ? rigidBodyComponent.velocity.X : add.X, add.Y == 0 ? rigidBodyComponent.velocity.Y : add.Y);
            //rigidBodyComponent.velocity = new SFML.System.Vector2f(isx?x:rigidBodyComponent.velocity.X,isy?y:rigidBodyComponent.velocity.Y) * speed * 10;
            rigidBodyComponent.velocity = new Vector2f(x,y)*speed*10;




            //var sr = GetComponent<SpriteRendererComponent>();
        }
    }
}
