using MatrixEngine.Content;
using MatrixEngine.fonts;
using MatrixEngine.GameObjects;
using MatrixEngine.GameObjects.Components;
using MatrixEngine.GameObjects.Components.PhysicsComponents;
using MatrixEngine.GameObjects.Components.RenderComponents;
using MatrixEngine.GameObjects.Components.TilemapComponents;
using MatrixEngine.GameObjects.Components.UIComponents;
using MatrixEngine.Scenes;
using MatrixEngine.System;
using MatrixEngine.System.AsyncOperations;
using SFML.Graphics;
using SFML.System;
using System.Collections;

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

                text = $"FPS: {(1.0f / app.deltaTime).ToString("0.0")} \nZoom: {app.camera.zoom}";

            }
            public override void Start() {
                base.Start();
            }
        }

        class RenderTesterComponent : Component {
            public override void Start() {
                app.asyncOperationManager.AddAsyncOperation(new AsyncOperation(create()));

                gameObject.SetComponent<TilemapComponent>();
                gameObject.SetComponent<TilemapRendererComponent>();


            }

            public override void Update() {
            }
            private IEnumerator create() {
                var t = GetComponent<TilemapComponent>();

                for (int x = 0; x < 1000; x++) {
                    for (int y = 0; y < 100; y++) {
                        t.SetTile(new Vector2i(x, y), new Tile(TextureManager.GetTexture("Image1.png")));

                    }
                    yield return null;

                }

            }
        }

        static void Main(string[] args) {
            App app = new App("Tests",
                new Scene(
                    new GameObject[] {
                        new GameObject(
                            new Vector2f(0,0),
                            new Component[]{
                            new SpriteRendererComponent("Image1.png",16,0) ,
                            new SimplePlayerControllerComponent(),
                            new RigidBodyComponent(
                                new Vector2f(0,0),
                                0.05f,
                                false ),
                            new CameraControllerComponent()
                        }),
                    new GameObject(
                        new Vector2f(-20,-20),
                        new Component[]{
                            new SpriteRendererComponent("Image2.png",800,-1),
                            new RigidBodyComponent() { isStatic = true },
                            new RenderTesterComponent()
                        }),
                    new GameObject(
                        new Vector2f(0,0),
                        new Component[]{

                            new FPSCounterComponent()

                        }),

                    }
                   ));

            app.Run();

        }
    }
}
