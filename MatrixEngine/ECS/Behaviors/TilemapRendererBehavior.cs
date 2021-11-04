using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using SFML.Graphics;
using SFML.System;

namespace MatrixEngine.ECS.Behaviors
{
    public class TilemapRendererBehavior : RendererBehavior
    {
        public TilemapRendererBehavior(float pixelsPerUnit)
        {
            PixelsPerUnit = pixelsPerUnit;
        }

        public override void Dispose()
        {
            s.Dispose();
        }

        public override void Render(RenderTarget target)
        {
            var a = GetActor();
            var trans = a.Transform;
            var t = a.GetBehavior<TilemapBehavior>();
            if (t == null)
            {
                return;
            }
            var lis = t.tiles.ToList();
            foreach (var tile in lis)
            {
                //s.Texture?.Dispose();
                s.Texture = tile.Value.Texture;
                s.Position = ((Vector2f)tile.Key).Multiply(trans.Scale) + trans.Position;
                s.Scale = trans.Scale / PixelsPerUnit;
                target.Draw(s);
            }
        }

        private Sprite s { get; set; } = new Sprite();

        public float PixelsPerUnit { get; set; }
    }
}