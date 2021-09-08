using MatrixEngine.Content;
using MatrixEngine.Framework;
using SFML.Audio;
using SFML.System;
using SFML.Window;

namespace MatrixEngine.GameObjects.Components.PhysicsComponents {
    [RequireComponent(typeof(RigidBodyComponent))]
    public class SimplePlayerControllerComponent : Component {

        private Sound sound;

        public float speed = 2;


        private float x;
        private float y;
        private bool isx;
        private bool isy;

        public SimplePlayerControllerComponent(float speed) {
            this.speed = speed;
        }

        public override void Start() {
            sound = AudioManager.GetAudio("tracks/testJump.wav");
            app.window.MouseWheelScrolled += (s, e) => { app.camera.zoom += (float) e.Delta / 10; };


            app.AddToDebug(rigidBodyComponent);

        }

        public override void Update() {
            x   = (float)rigidBodyComponent.velocity.X ;
            y  = (float)rigidBodyComponent.velocity.Y;
            isx = false;
            isy = false;
            if (keyHandler.isPressed(Keyboard.Key.A)) {
                x = -speed;
                isx = true;
            }
            if (keyHandler.isPressed(Keyboard.Key.D)) {
                x = speed;
                isx = true;
                

            }
            if (keyHandler.isPressedDown(Keyboard.Key.W)) {
                sound.Play();
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
            rigidBodyComponent.velocity = new Vector2f(x, y);

            //var sr = GetComponent<SpriteRendererComponent>();
        }
    }
}
