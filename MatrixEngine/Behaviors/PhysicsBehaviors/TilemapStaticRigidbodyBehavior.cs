using System;
using System.Collections.Generic;
using System.Linq;
using MatrixEngine.MatrixMath;
using MatrixEngine.Utils;
using SFML.Graphics;
using SFML.System;

namespace MatrixEngine.ECS.Behaviors.PhysicsBehaviors
{
    public class TilemapStaticRigidbodyBehavior : StaticRigidbodyBehavior
    {
        private TilemapBehavior _tilemapBehavior;

        protected override void OnStart()
        {
            _tilemapBehavior = GetBehavior<TilemapBehavior>() ??
                               throw new BehaviorNotFoundException(typeof(TilemapBehavior));
        }

        protected override void OnUpdate()
        {
            Logging.Assert(HaveBehavior<TilemapBehavior>(), "There is no TilemapBehavior in this Actor");
        }

        public override void Dispose()
        {
        }

        public override float GetCollidingFix(Rect dynamicStartRect, Rect dynamicEndRect, Direction dir)
        {
            var areaRect = dynamicStartRect.BigRectArea(dynamicEndRect);

            Rect tileRect;
            float fixFloat;
            Vector2i tilepos;
            Vector2f worldpos;
            Tile t;
            Vector2f testingPos;
            var maxSize = areaRect.max;


            var tilemapScale = _tilemapBehavior.Transform.Scale;

            var options = new List<float>();


            var array = new VertexArray();

            

            var x = 0.0f;
            var y = 0.0f;
            for (x = (areaRect.position - tilemapScale).X; x < areaRect.max.X + tilemapScale.X; x += tilemapScale.X/2)
            {
                for (y = (areaRect.position -tilemapScale).Y ; y < areaRect.max.Y + tilemapScale.Y; y += tilemapScale.Y/2)
                {
                    testingPos = new Vector2f(x, y);
                    tilepos = _tilemapBehavior.GetPosOfTileFromWorldPos(testingPos);
                    worldpos = _tilemapBehavior.GetWorldPosFromTilePos(tilepos);
                    t = _tilemapBehavior.GetTileFromTilemapPos(tilepos);

                    array.Append(new Vertex((Vector2f)worldpos) {Color=Color.Black });

                    if (t != null)
                    {
                        tileRect = new Rect(worldpos, tilemapScale);
                        GetEngine().QuickDrawRect(tileRect,Color.Magenta);
                        fixFloat = Physics.GetCollisionFix(dynamicStartRect, dynamicEndRect, tileRect, dir);
                        if (fixFloat!=0)
                        {
                            options.Add(fixFloat);

                        }
                    }
                }
            }
            GetEngine().Window.Draw(array);

            return options.Count == 0 ? 0 : options.Aggregate((a, b) => a.Abs() > b.Abs() ? a : b);
        }
    }
}