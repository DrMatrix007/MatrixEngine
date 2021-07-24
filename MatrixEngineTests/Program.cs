using MatrixEngine.fonts;
using MatrixEngine.GameObjects;
using MatrixEngine.GameObjects.Components;
using MatrixEngine.GameObjects.Components.PhysicsComponents;
using MatrixEngine.GameObjects.Components.RenderComponents;
using MatrixEngine.GameObjects.Components.UIComponents;
using MatrixEngine.Scenes;
using MatrixEngine.System;
using SFML.Graphics;
using SFML.System;

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

                for (int i = 0; i < 10000; i++) {
                    scene.AddGameObject(
                        new GameObject(
                        new Vector2f(i, 20),
                        new Component[]
                        {
                        new SpriteRendererComponent("Image1.png",16 ,20),
                        new RigidBodyComponent(true),
                        }
                        ));
                    if (i % 1000 == 0) {
                        Debug.Log(i);
                    }
                }
                //Parallel.For(0, 1000, (i) => {

                //    scene.AddGameObject(
                //        new GameObject(
                //        new Vector2f(i, 20),
                //        new Component[]
                //        {
                //        new SpriteRendererComponent("Image1.png",16 ,20),
                //        new RigidBodyComponent(true),
                //        }
                //        ));

                //});


            }

            public override void Update() {
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
