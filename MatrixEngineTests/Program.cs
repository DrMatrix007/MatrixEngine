using MatrixEngine.App;
using MatrixEngine.GameObjects;
using MatrixEngine.GameObjects.Components;
using MatrixEngine.Scenes;
using SFML.System;
using System;

namespace MatrixEngineTests {
    class Program {


        class RenderTesterComponent : Component {
            public override void Start() {
                for (int i = 0; i < 100; i++) {
                    scene.AddGameObject(
                        new GameObject(
                        new Vector2f(new Random().Next(-1000, 1000), 20),
                        new Component[]
                        {
                        new SpriteRendererComponent("Image1.png", 20),
                        new RigidBodyComponent(true),
                        }
                        ));
                }
            }

            public override void Update() {
            }
        }

        static void Main(string[] args) {
            App app = new App("Tests",
                new Scene(
                    new GameObject[] {
                        new GameObject(
                            new Component[]{
                            new SpriteRendererComponent("Image1.png",0) ,
                            new SimplePlayerControllerComponent(),
                            new RigidBodyComponent(
                                new Vector2f(0,0),
                                new Vector2f(0.00000001f,0.000000001f),
                                false ),
                            new CameraControllerComponent()
                        }),
                    new GameObject(
                        new Vector2f(-20,-20),
                        new Component[]{
                            new SpriteRendererComponent("Image1.png",-1),
                            new RigidBodyComponent() { isStatic = true },
                            new RenderTesterComponent()
                        }),

                    }
                   ));

            app.Run();

        }
    }
}
