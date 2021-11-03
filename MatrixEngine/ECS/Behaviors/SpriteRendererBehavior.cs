using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.ECS.Behaviors;
using SFML.Graphics;

namespace MatrixEngine.ECS.Behaviors
{
    public class SpriteRendererBehavior : RendererBehavior
    {
        private Sprite sprite;

        public float PixelsPerUnit = 1;

        public SpriteRendererBehavior(Texture texture, float pixelperunit)
        {
            PixelsPerUnit = pixelperunit;
            sprite = new Sprite(texture);
        }

        protected override void OnStart()
        {
        }

        protected override void OnUpdate()
        {
        }

        public override void Render(RenderTarget target)
        {
            var t = Transform;
            sprite.Position = t.Position;

            sprite.Scale = t.Scale / PixelsPerUnit;

            target.Draw(sprite);
        }

        public override void Dispose()
        {
            sprite?.Dispose();
        }
    }
}