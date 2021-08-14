using System;
using System.Linq;
using MatrixEngine.UI;
using MatrixEngine.GameObjects;
using MatrixEngine.GameObjects.Components;
using MatrixEngine.GameObjects.Components.PhysicsComponents;
using MatrixEngine.GameObjects.Components.RenderComponents;
using MatrixEngine.GameObjects.Components.StateManagementComponents;
using MatrixEngine.GameObjects.Components.TilemapComponents;
using MatrixEngine.StateManagment;
using MatrixEngine.System;
using SFML.Graphics;
using SFML.System;
using SFML.Window;

namespace MatrixEngine {
    class Program {
        static void Main(string[] args) {
            var prov = new ComponentProvider<TilemapComponent>();

            var counterProv = new CounterProvider();

            var isDebug = false;


            App app = new App("Tests",
                isDebug,
                new Scene(
                    new GameObject[]
                    {
                        new GameObject(
                            new Vector2f(-10, -10),
                            new Component[]
                            {
                                new SpriteRendererComponent("Image1.png", 18, 2),
                                new ConsumerComponent<TilemapComponent>(prov),
                                new ProviderTesterComponent(),
                                new ConsumerComponent<int>(counterProv),
                                new SimplePlayerControllerComponent(),
                                new RigidBodyComponent(new Vector2f(), new Vector2f(0.9f, 0.9f), false),
                                new ColliderComponent(ColliderComponent.ColliderType.Rect),
                                new CameraControllerComponent(),
                                new TestResizeTilemapComponent(prov)
                            }
                        ),
                        new GameObject(
                            new Component[]
                            {
                                new TilemapComponent(),
                                new TilemapRendererComponent(),
                                new ComponentProviderSetterComponent<TilemapComponent>(prov),
                                new RigidBodyComponent(true),
                                new ColliderComponent(ColliderComponent.ColliderType.Tilemap),
                            }
                        ),
                        new GameObject(
                            new Component[]
                            {
                                new SpriteRendererComponent("Image2.png", 400, 55),
                                new RigidBodyComponent(true),
                                new ColliderComponent(ColliderComponent.ColliderType.Rect),
                            }
                        )
                    },
                    // (new int[]
                    // {
                    //     1, 2, 3, 4, 5, 6
                    // }).Select(
                    //     (e, i) => new SpriteRendererUIObject(new Anchor(new Vector2f(10 * e, 10), new Vector2f(10, 10)),
                    //         new Texture("Image1.png"))
                    // )
                    new UIObject[]
                    {
                        // new SpriteRendererUIObject(new Anchor(new Vector2f(),new Vector2f(10,10) ),new Texture("Image1.png") ),

                        new TextRendererConsumerUIObject(new Anchor(new Vector2f(), new Vector2f(20, 10)),
                            new ProviderConverter<string, int>(counterProv, e => e.ToString())),
                    }
                )
            );

            app.Run();
        }
    }

    public class ProviderTesterComponent : Component {
        public override void Start() {
            var r = new Random();
            var p = GetComponent<ConsumerComponent<TilemapComponent>>().GetOutput();
            if (p == null) {
                return;
            }

            p.transform.scale = new Vector2f(1f, 1f);
            // for (int i = 0; i < 900; i++) {
            //     for (int j = 0; j < 900; j++) {
            //         if (r.NextDouble() < 0.2)
            //             p.SetTile(new Vector2i(i, j), new Tile(new Texture("grass.png")));
            //     }
            // }

            var p1 = GetComponent<ConsumerComponent<int>>();
            app.AddToDebug(p1.provider);
        }

        public override void Update() {
            var p = GetComponent<ConsumerComponent<int>>().provider as CounterProvider;

            if (app.keyHandler.isPressed(Keyboard.Key.G)) {
                p.Add();
            }
        }
    }
}