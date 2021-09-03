using MatrixEngine.Content;
using MatrixEngine.Framework;
using MatrixEngine.Framework.Operations;
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
using System.Collections;

namespace MatrixEngineTests {

    internal static class Program {
        private static void Main1(string[] args) {
            var y = 10;

            var r1 = new Rect(0, y, 1, 1);


            var r2 = new Rect(0, y + 1.0f, 1, 1);

            Console.WriteLine(r1.isColliding(r2));
        }


        private static void Main2(string[] args) {
            var p = new PerlinNoise(new Seed(), 10,5);

            p.Generate();

            for (int x = 0; x < 50; x++) {
                for (int y = 0; y < 50; y++) {
                    var v = p[x, y];
                    Console.Write(" "+v.ToString(".00"));
                }
                Console.WriteLine();
            }

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
                ),
                new GameObject(
                    new Vector2f(1,0),
                    new Component[] {
                    new ColliderComponent(ColliderComponent.ColliderType.Rect),
                    new SpriteRendererComponent("Image2.png",200,0),
                    new RigidBodyComponent(true)
                    }
                ),

                new GameObject(
                    new Vector2f(0,-100),
                    new Component[] {
                        new CameraController(),
                        new SimplePlayerControllerComponent(),
                        new RigidBodyComponent(new Vector2f(0,90),new Vector2f(50,50),false),
                        new SpriteRendererComponent("Image1.png",16,10),

                    }),
                new GameObject(

                    new Component[] {
                        new TilemapComponent(16),
                        new TilemapRendererComponent(),
                        new TilemapTesterComponent(new Seed()),
                        new ColliderComponent(ColliderComponent.ColliderType.Tilemap),
                        new RigidBodyComponent(true),
                    })
                },

                new UIObject[] {
                    new SpriteRendererUIObject(new Anchor(new Vector2f(0,0),new Vector2f(10,10)),TextureManager.GetTexture("grass.png"),new UIStyle(0,Color.White,Color.Transparent),10)
                    ,
                    new TextRendererConsumerUIObject(new Anchor(new Vector2f(0,90),new Vector2f(20,10)),fpsProv,new UITextStyle(
                        10,
                        color: Color.White,
                        backgroundColor: Color.Black,
                        isResize:true,
                        font: FontManager.CascadiaCode,
                        charSize:10
                        ),10)
                }
            );

            var app = new App("Tests", false, scene);

            fpsProv.SetFunc(() => {
                return $"FPS: {(1.0f / app.deltaTime).ToString("0.00")},Nice!";
            });
            app.Run();

        }
    }
    [RequireComponent(typeof(TilemapComponent))]
    class TilemapTesterComponent : Component {

        public Seed seed;

        public TilemapTesterComponent(Seed s) {
            this.seed = s;
        }

        public override void Start() {

            operationManager.AddAsyncOperation(new Operation(Generate()));

        }

        public IEnumerator Generate() {
            var c = GetComponent<TilemapComponent>();
            var p = new PerlinNoise(seed, 100,5);
            p.Generate();



            for (int x = 0; x < 100*5; x++) {
                for (int y = 0; y < 100*5; y++) {
                    var v = p[x, y];
                    if (v>0.5f) {
                        c.SetTile(x, y, new Tile(TextureManager.GetTexture("grass.png")));
                    } else {
                        c.SetTile(x, y, null);
                    }
                }
            }


            //for (int x = (int)(-max / 2); x < max / 2; x++) {
            //    var v = p[((x + max / 2) / max), 1];
            //    v *= 20;

            //    for (int y = 0; y < 20 + 50; y++) {
            //        if (y < v + 50) {
            //            c.SetTile(x, -y, new Tile(TextureManager.GetTexture("grass.png")));
            //        } else {
            //            c.SetTile(x, -y, null);
            //        }

            //    }

            //}
            yield return null;

        }

        public override void Update() {

            if (keyHandler.isPressedDown(SFML.Window.Keyboard.Key.G)) {
                seed = new Seed();
                operationManager.AddAsyncOperation(new Operation(Generate()));

            }

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