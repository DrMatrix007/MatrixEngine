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

        public SimplePlayerControllerComponent(float speed) {
            this.speed = speed;
        }

        public override void Start() {
            sound = AudioManager.GetAudio("tracks/testJump.wav");
            App.Window.MouseWheelScrolled += (s, e) => { App.camera.zoom += (float)e.Delta / 10; };

            App.AddToDebug(RigidBodyComponent);
        }

        public override void Update() {
            x = (float)RigidBodyComponent.Velocity.X;
            y = (float)RigidBodyComponent.Velocity.Y;
            if (KeyHandler.IsPressed(Keyboard.Key.A)) {
                x = -speed;
            }
            if (KeyHandler.IsPressed(Keyboard.Key.D)) {
                x = speed;
            }
            if (KeyHandler.IsPressedDown(Keyboard.Key.W)) {
                sound.Play();
            }
            if (KeyHandler.IsPressed(Keyboard.Key.W)) {
                y = -speed;
            }
            if (KeyHandler.IsPressed(Keyboard.Key.S)) {
                y = speed;
            }

            //add = new Vector2f(add.X == 0 ? rigidBodyComponent.velocity.X : add.X, add.Y == 0 ? rigidBodyComponent.velocity.Y : add.Y);
            //rigidBodyComponent.velocity = new SFML.System.Vector2f(isx?x:rigidBodyComponent.velocity.X,isy?y:rigidBodyComponent.velocity.Y) * speed * 10;
            RigidBodyComponent.Velocity = new Vector2f(x, y);

            //var sr = GetComponent<SpriteRendererComponent>();
        }
    }
}