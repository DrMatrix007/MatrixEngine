using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine;
using MatrixEngine.Behaviors;

using MatrixEngine.Behaviors.PhysicsBehaviors;
using MatrixEngine.Behaviors.RendererBehaviors;
using MatrixEngine.MatrixMath;
using MatrixEngine.Plugins;
using MatrixEngine.Utils;
using SFML.Graphics;
using SFML.System;

//namespace MatrixEngineTests
//{
//    public static partial class Program
//    {
//        public static void Main(string[] args)
//        {

var engine = new Engine(new WindowSettings() { Name = "Tests", Size = new Vector2u(1000, 500) });

engine.CurrentScene = new Scene();

var dy =
    new DynamicRigidbodyBehavior(new Vector2f(0, 0), new Vector2f(50, 50));

dy.OnCollisionTrigger += (a, b) =>
 {
     "Collision Triggered".Log();
 };

engine.CurrentScene.AddActor(new Actor(new Behavior[]
{
    new SpriteBehavior(new Texture("object.png")),
    new RectBehavior(new Rect(0,0,0.8f,1.6f)),
    new TestBehavior(),
    dy
}));

engine.CurrentScene.AddActor(new Actor(new Behavior[]
{
    new SpriteBehavior(new Texture("object.png")),
    new RectBehavior(new Rect(0,0,0.8f,1.6f)),
    new DynamicRigidbodyBehavior(new Vector2f(0,0),new Vector2f(50,50),true)
}));

engine.CurrentScene.AddActor(new Actor(new Behavior[]
{
    new SpriteBehavior(new Texture("object.png")),
    new RectStaticRigidbodyBehavior(),
}));
var image = new ImageBehavior(new Texture("grass.png"));


engine.CurrentScene.AddActor(new Actor(
    new AnchorBehavior(new Vector2f(-0.5f, -0.5f), new Vector2f(0.5f, 0.5f)),
    image
));
var t = new TilemapBehavior();

var tex = new Texture("grass.png");

var ran = new MatrixRandom(69420);

var s = 200;
var d = 10;

var perlin = new PerlinNoise2D(s, d, MatrixRange.ZeroToOne, 15);

Logging.LogTime(perlin.Generate);

for (int i = 0; i < s * d; i++)
{
    for (int j = 0; j < s * d; j++)
    {
        if (perlin.floats[i, j] < 0.5f)
        {
            t.SetTile(new Vector2i(i, j), new Tile(tex));
        }
    }

    i.Log();
}


engine.CurrentScene.AddActor(new Actor(new Behavior[]
{
                t,
                new TilemapRendererBehavior(16),
                new TilemapStaticRigidbodyBehavior()
}));

engine.CurrentScene.AddPlugin(new RendererPlugin());
engine.CurrentScene.AddPlugin(new PhysicsPlugin());

engine.Run();
