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
                        new SpriteRendererComponent("Image1.png",16 ,20),
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
                            new Vector2f(0,0),
                            new Component[]{
                            new SpriteRendererComponent("Image1.png",16,0) ,
                            new SimplePlayerControllerComponent(),
                            new RigidBodyComponent(
                                new Vector2f(0,0),
                                0.01f,
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

                            new TextRendererComponent("Test123123132\ngg",FontManager.CascadiaCode,Color.Red,100)

                        }),

                    }
                   ));

            app.Run();

        }
    }
}
