using MatrixEngine.System;
using SFML.System;
using SFML.Window;

namespace MatrixEngine.GameObjects.Components.PhysicsComponents {
    [RequireComponent(typeof(RigidBodyComponent))]
    public class SimplePlayerControllerComponent : Component {



        public float speed = 2;

        public override void Start() {
            
            app.window.MouseWheelScrolled += (s, e) => { app.camera.zoom += (float) e.Delta / 10; };


            app.AddToDebug(rigidBodyComponent);

        }

        public override void Update() {
            var x = (float)rigidBodyComponent.velocity.X / speed;
            var y = (float)rigidBodyComponent.velocity.Y / speed;
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
            rigidBodyComponent.velocity = new Vector2f(x, y) * speed;

            //var sr = GetComponent<SpriteRendererComponent>();
        }
    }
}
