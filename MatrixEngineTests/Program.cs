using MatrixEngine.Content;
using MatrixEngine.Framework;
using MatrixEngine.GameObjects;
using MatrixEngine.GameObjects.Components;
using MatrixEngine.GameObjects.Components.PhysicsComponents;
using MatrixEngine.GameObjects.Components.RenderComponents;
using MatrixEngine.GameObjects.Components.TilemapComponents;
using MatrixEngine.Physics;
using MatrixEngine.StateManagment;
using MatrixEngine.UI;
using MatrixEngine.Utilities;
using SFML.Graphics;
using SFML.System;
using System;

namespace MatrixEngineTests {

    internal static class Program {
        private static void Main1(string[] args) {
            var y = 10;

            var r1 = new Rect(0, y, 1, 1);


            var r2 = new Rect(0, y + 1.0f, 1, 1);

            Console.WriteLine(r1.isColliding(r2));
        }


        private static void Main(string[] args) {

            FunctionProvider<string> fpsProv = new FunctionProvider<string>();


            var scene = new Scene(new[]{
                new GameObject(
                    new Vector2f(0,-1),
                    new Component[] {
                    new ColliderComponent(ColliderComponent.ColliderType.Rect),
                    new SpriteRendererComponent("Image2.png",200,0),
                    new RigidBodyComponent(true)
                    }
                ),                new GameObject(
                    new Vector2f(1,0),
                    new Component[] {
                    new ColliderComponent(ColliderComponent.ColliderType.Rect),
                    new SpriteRendererComponent("Image2.png",200,0),
                    new RigidBodyComponent(true)
                    }
                ),

                new GameObject(
                    new Vector2f(0,-10),
                    new Component[] {
                        new CameraController(),
                        new SimplePlayerControllerComponent(),
                        new RigidBodyComponent(new Vector2f(0,60),new Vector2f(10,0),false),
                        new SpriteRendererComponent("Image1.png",16,10),

                    }),
                new GameObject(

                    new Component[] {
                        new TilemapComponent(16),
                        new TilemapRendererComponent(),
                        new TilemapTesterComponent(),
                        new ColliderComponent(ColliderComponent.ColliderType.Tilemap),
                        new RigidBodyComponent(true),
                    }
           )
                    },

                    new UIObject[] {
                        new TextRendererConsumerUIObject(new Anchor(new Vector2f(90,0),new Vector2f(20,10)),fpsProv,new UITextStyle(
                            color: Color.White
                            ),10)
                    }
                    );

            var app = new App("Tests", false, scene);

            fpsProv.SetFunc(() => {
                return $"FPS: {1.0f/app.deltaTime}";
            });

            app.Run();

        }
    }
    [RequireComponent(typeof(TilemapComponent))]
    class TilemapTesterComponent : Component {
        public override void Start() {
            var c = GetComponent<TilemapComponent>();
            var p = new PerlinNoise(new Seed(), 100);
            p.Generate();
            var max = 1.0f;
            max /= p.step / 10;
            max = (int)max;
            float val;

            for (int x = 0; x < max; x++) {
                for (int y = 0; y < max; y++) {
                    val = p[x / max, y / max];
                    if (val > 0.6f) {
                        c.SetTile(x - (int)max / 2, y - (int)max / 2, new Tile(TextureManager.GetTexture("grass.png")));
                    }


                }
            }

        }

        public override void Update() {
        }
    }
    class CameraController : Component {
        public override void Start() {
        }

        public override void Update() {
            app.camera.position = transform.fullRect.center;
        }
    }
}