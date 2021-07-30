using MatrixEngine.Content;
using MatrixEngine.fonts;
using MatrixEngine.GameObjects;
using MatrixEngine.GameObjects.Components;
using MatrixEngine.GameObjects.Components.RenderComponents;
using MatrixEngine.GameObjects.Components.StateManagementComponents;
using MatrixEngine.GameObjects.Components.TilemapComponents;
using MatrixEngine.GameObjects.Components.UIComponents;
using MatrixEngine.GameObjects.StateManagment;
using MatrixEngine.Scenes;
using MatrixEngine.System;
using MatrixEngine.GameObjects.Components.StateManagementComponents;
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

        static void Main(string[] args) {

            var prov = new ComponentProvider<TilemapComponent>();


            App app = new App("Tests",
                new Scene(
                    new GameObject[] {

                    new GameObject(
                        new Component[] {
                            new SpriteRendererComponent("Image1.png",16,2),
                            new ConsumerComponent<TilemapComponent>(prov),
                            new ProviderTesterComponent(),
                        }

                        ),

                    new GameObject(
                        new Component[] {
                            new FPSCounterComponent(),
                            new TilemapComponent(),
                            new TilemapRendererComponent(),
                            new ComponentProviderSetterComponent<TilemapComponent>(prov)
                        }

                    ),


                    }
                )
            );

            app.Run();

        }
    }
    internal class ProviderTesterComponent : Component {
        public override void Start() {
            var p = GetComponent<ConsumerComponent<TilemapComponent>>().GetOutput();
            for (int i = 0; i < 1000; i++) {
                for (int j = 0; j < 1000; j++) {
                    p.SetTile(new Vector2i(i,j),new Tile(TextureManager.GetTexture("Image1.png")));
                }
            }
        }

        public override void Update() {



        }
    }
}
