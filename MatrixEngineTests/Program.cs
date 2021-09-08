using MatrixEngine.Content;
using MatrixEngine.Framework;
using MatrixEngine.Framework.MathM;
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
            var p = new PerlinNoise2D(20, 2, new MatrixEngine.Utilities.Range(0, 1));

            p.Generate();
            Console.WriteLine("????????????????????????");
            //for (int x = 0; x < p.size; x++) {
            //    var v = p[x];
            //    //Console.Write(" " + v.ToString(".00"));
            //    for (int i = 0; i < v; i++) {
            //        if (v > .5f) {
            //            Console.Write("0");
            //        } else {
            //            Console.Write(" ");
            //        }
            //    }
            for (int x = 0; x < p.randomGenerationSize * p.size; x++) {
                for (int y = 0; y < p.randomGenerationSize * p.size; y++) {
                    var v = p.floats[x, y];
                    //Console.WriteLine(v);
                    if (v > 0.5f) {
                        Console.Write("0");
                    } else {
                        Console.Write(" ");
                    }
                }
                Console.Write("\n");
            }

            //    Console.WriteLine();
            //}
        }

        public static void Main(string[] args) {
            var fps_prov = new FunctionProvider<string>();
            var app = new App("", false, new Scene(
                new GameObject[] {
                    new GameObject(new Component[]{
                    new ColliderComponent(ColliderComponent.ColliderType.Rect),
                    new RigidBodyComponent(new Vector2f(0,0),new Vector2f(100,100)),
                    new SpriteRendererComponent("Image1.png",16,10),
                    new SimplePlayerControllerComponent(20)
                }),
                new GameObject(
                    new TilemapComponent(16),
                    new TilemapTesterComponent(new Seed()),
                    new TilemapRendererComponent()
                    )
                }, 
                new UIObject[] {
                    new TextConsumerUIObject(new Anchor(new Vector2f(),new Vector2f(20,10)),fps_prov,new UITextStyle(10,Color.White,Color.Black,FontManager.CascadiaCode,isResize:true),10)
                }
                ));
            fps_prov.SetFunc(() => {
                return $"fps is: {(1 / app.deltaTime).ToString("0000.0")}";
            });
            app.Run();
        }

        private static void MainTests(string[] args) {
            FunctionProvider<string> fpsProv = new FunctionProvider<string>();

            var scene = new Scene(
                new[] {
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
                        new SimplePlayerControllerComponent(10),
                        new ColliderComponent(ColliderComponent.ColliderType.None),
                        new RigidBodyComponent(new Vector2f(0,0),new Vector2f(500,500)),
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
                    new SpriteRendererUIObject(new Anchor(new Vector2f(0,0),new Vector2f(10,10)),TextureManager.GetTexture("grass_side.png"),new UIStyle(0,Color.White,Color.Transparent),10)
                    ,
                    new TextConsumerUIObject(new Anchor(new Vector2f(0,90),new Vector2f(20,10)),fpsProv,new UITextStyle(
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
    internal class TilemapTesterComponent : Component {
        public Seed seed;

        public TilemapTesterComponent(Seed s) {
            this.seed = s;
        }

        public override void Start() {
            operationManager.AddAsyncOperation(new Operation(Generate()));
        }

        public IEnumerator Generate() {
            var c = GetComponent<TilemapComponent>();
            var count = 0;
            var maxcount = 20;
            //var p1 = new PerlinNoise1D(100,100,new MatrixEngine.Utilities.Range(20,50));
            //p1.Generate();
            //for (int i = 0; i < p1.fullSize; i++) {

            //    for (int y = 0; y < p1[i]; y++) {
            //        c.SetTile(i - p1.fullSize / 2, -y, new Tile(TextureManager.GetTexture("grass.png")));
            //    }


            //    count++;
            //    if (count >= maxcount) {
            //        count = 0;
            //        yield return null;
            //    }
            //}

            var p1 = new PerlinNoise2D(200, 10, new MatrixEngine.Utilities.Range(0, 1));
            p1.Generate();
            Console.WriteLine(((float)p1.fullSize).Sqr());
            for (int x = 0; x < p1.fullSize; x++) {
                for (int y = 0; y < p1.fullSize; y++) {
                    var v = p1.floats[x, y];
                    if (v < 0.5f) {
                        c.SetTile(x - p1.fullSize / 2, y - p1.fullSize / 2, new Tile(TextureManager.GetTexture("water.png")));
                    } else if (v < 0.8f) {
                        c.SetTile(x - p1.fullSize / 2, y - p1.fullSize / 2, new Tile(TextureManager.GetTexture("grass.png")));
                    } else if (v < 0.93f) {
                        c.SetTile(x - p1.fullSize / 2, y - p1.fullSize / 2, new Tile(TextureManager.GetTexture("stone.png")));
                    } else {
                        c.SetTile(x - p1.fullSize / 2, y - p1.fullSize / 2, new Tile(TextureManager.GetTexture("snow.png")));

                    }
                    //c.SetTile(x - p1.fullSize / 2, y - p1.fullSize / 2, new Tile(r.Texture,new Color((byte)(v * 255), (byte)(v * 255), (byte)(v * 255), 255)));
                }
                count++;
                if (count > maxcount) {
                    count = 0;
                    yield return null;
                }
            }
            yield return null;
        }

        public override void Update() {
            if (keyHandler.isPressedDown(SFML.Window.Keyboard.Key.G)) {
                seed = new Seed();
                GetComponent<TilemapComponent>().Clear();

                operationManager.AddAsyncOperation(new Operation(Generate()));
            }
        }
    }

    internal class CameraController : Component {

        public override void Start() {
        }

        public override void Update() {
            app.camera.position = transform.fullRect.center;
        }
    }
}