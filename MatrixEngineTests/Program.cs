using MatrixGDK.Content;
using MatrixGDK.GameObjects;
using MatrixGDK.GameObjects.Components;
using MatrixGDK.GameObjects.Components.PhysicsComponents;
using MatrixGDK.GameObjects.Components.RenderComponents;
using MatrixGDK.GameObjects.Components.StateManagementComponents;
using MatrixGDK.GameObjects.Components.TilemapComponents;
using MatrixGDK.GameObjects.Components.UIComponents;
using MatrixGDK.GameObjects.StateManagment;
using MatrixGDK.Scenes;
using MatrixGDK.System;
using SFML.Graphics;
using SFML.System;
using SFML.Window;
using System;
using static MatrixEngineTests.Program;

namespace MatrixEngineTests {
    class Program {
        class FPSCounterComponent : TextRendererComponent {


            public FPSCounterComponent() : base("", FontManager.CascadiaCode, Color.Red) {
            }

            public override void Render(RenderTarget target) {
                base.Render(target);
            }
            public override void Update() {
                base.Update();

                text = $"FPS: {(1.0f / app.deltaTime).ToString("0")} \nZoom: {app.camera.zoom}";

            }
            public override void Start() {
                base.Start();
            }
        }

        public class Counter {
            int data = 0;
            public void Increment() {
                data++;
            }
            public int GetCounter() {
                return data;
            }
            public override string ToString() {
                return $"Counter: {data}";
            }
        }

        static void Main(string[] args) {

            var prov = new ComponentProvider<TilemapComponent>();

            var counterProv = new StateProvider<Counter>(new Counter());


            App app = new App("Tests",
                true,
                new Scene(
                    new GameObject[] {

                    new GameObject(
                        new Vector2f(-10,-10),
                        new Component[] {
                            new SpriteRendererComponent("Image1.png",16,2),
                            new ConsumerComponent<TilemapComponent>(prov),
                            new ProviderTesterComponent(),
                            new ConsumerComponent<Counter>(counterProv),
                            new SimplePlayerControllerComponent(),
                            new RigidBodyComponent(new Vector2f(),new Vector2f(0.5f,0.5f),false),
                            new ColliderComponent(ColliderComponent.ColliderType.Rect),
                            new CameraControllerComponent(),
                        }

                        ),

                    new GameObject(
                        new Component[] {
                            new FPSCounterComponent(),
                            new TilemapComponent(),
                            new TilemapRendererComponent(),
                            new ComponentProviderSetterComponent<TilemapComponent>(prov),
                            new RigidBodyComponent(true),
                            new ColliderComponent(ColliderComponent.ColliderType.Tilemap),
                        }

                    ),
                    new GameObject(

                        new Component[] {
                            new SpriteRendererComponent("Image2.png",400,55),
                            new RigidBodyComponent(true),
                            new ColliderComponent(ColliderComponent.ColliderType.Rect),


                        }
                        )

                    }
                )
            );

            app.Run();

        }
    }
    public class ProviderTesterComponent : Component {

        Counter t;
        public override void Start() {
            var r = new Random();
            var p = GetComponent<ConsumerComponent<TilemapComponent>>().GetOutput();
            if (p == null) {
                return;
            }
            p.transform.scale = new Vector2f(0.05f, 0.05f);
            for (int i = 0; i < 1000; i++) {
                for (int j = 0; j < 1000; j++) {
                    if (r.NextDouble() < 0.2)
                        p.SetTile(new Vector2i(i, j), new Tile(TextureManager.GetTexture("grass.png")));
                }
            }
            var p1 = GetComponent<ConsumerComponent<Counter>>();
            t = p1.GetOutput();
            app.AddToDebug(t);

        }
        public override void Update() {
            var p = GetComponent<ConsumerComponent<Counter>>();

            if (app.keyHandler.isPressed(Keyboard.Key.G)) {
                p.GetOutput().Increment();
            }



        }

    }

}
