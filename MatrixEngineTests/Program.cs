using MatrixEngine.Framework;
using MatrixEngine.GameObjects;
using MatrixEngine.GameObjects.Components;
using MatrixEngine.GameObjects.Components.PhysicsComponents;
using MatrixEngine.GameObjects.Components.RenderComponents;
using MatrixEngine.Utilities;
using SFML.System;
using System.Threading.Tasks;

namespace MatrixEngineTests {

    public class Program {

        public static void Main() {
            var t = new Clock();

            var p = new PerlinNoise2D(10, 100, Range.ZeroToOne);

            var task = Task.Run(() => p.Generate());

            while (!task.IsCompleted) {
            }

            //p.Generate();
            t.ElapsedTime.AsSeconds().Log();

            "Done".Log();

            //    var app = new App("Tests", false, null);

            //    app.scene = new Scene();

            //    for (int x = 0; x < 10; x++) {
            //        for (int y = 0; y < 10; y++) {
            //            app.scene.AddGameObject(new GameObject(
            //                new Vector2f(x, y),
            //                new SpriteRendererComponent("object.png", 16, 10),
            //                new RigidBodyComponent(true)
            //            ));
            //        }
            //    }

            //    app.scene.AddGameObject(new GameObject(
            //        new SimplePlayerControllerComponent(10),
            //        new SpriteRendererComponent("object.png", 16, 100),
            //        new RigidBodyComponent(new Vector2f(), new Vector2f(50, 50)),
            //        new RayTesterComponent()
            //       ));

            //    app.Run();
            //}
        }
    }

    public class RayTesterComponent : Component {

        public override void Start() {
        }

        public override void Update() {
            var mouse_pos = InputHandler.GetMouseWorldPos();
            var l = new Line(Transform.fullRect.center, mouse_pos);

            var r = PhysicsEngine.LineCast(l);

            //var v = new VertexArray();

            //v.Append(new Vertex(l.start,Color.Black));
            //v.Append(new Vertex(r,Color.Black));

            //App.Window.Draw(v);
            //v.Dispose();
        }
    }
}