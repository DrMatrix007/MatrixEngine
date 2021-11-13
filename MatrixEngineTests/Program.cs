using MatrixEngine.ECS;
using MatrixEngine.ECS.Behaviors;
using MatrixEngine.ECS.Behaviors.PhysicsBehaviors;
using MatrixEngine.ECS.Plugins;
using MatrixEngine.Utils;
using SFML.System;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngineTests
{
    public partial class Program
    {
        private static void Main(string[] args)
        {

            var scene = new Scene();
            scene.AddActor(new Actor(new Vector2f(0, 0), new Behavior[]
            {
                new TestBehavior(),
                new SpriteRendererBehavior(new SFML.Graphics.Texture("player.png"),16),
                new DynamicRigidbodyBehavior(new Vector2f(0,10),new Vector2f(10,0))
            }));

            scene.AddActor(new Actor(new Vector2f(0, 0), new Behavior[]
            {
                new SpriteRendererBehavior(new SFML.Graphics.Texture("grass.png"),16),
                new RectStaticRigidbody()
            }));

            scene.AddPlugin(new RendererPlugin());

            scene.AddPlugin(new PhysicsPlugin());

            var engine = new Engine(new WindowSettings() { Name = "tests", Size = new Vector2u(1000, 500) }, scene);



            engine.Run();
        }


    }
}
