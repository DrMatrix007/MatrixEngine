using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.ECS.Behaviors;
using MatrixEngine.ECS.Plugins;
using MatrixEngine.MatrixMath;
using SFML.Graphics;
using SFML.System;

namespace MatrixEngine.ECS.Behaviors
{
    public class SpriteRendererBehavior : RendererBehavior
    {
        private RectBehavior rectBehavior;


        private Sprite sprite;

        public float PixelsPerUnit = 1;

        public SpriteRendererBehavior(Texture texture, float pixelperunit)
        {
            PixelsPerUnit = pixelperunit;
            sprite = new Sprite(texture);
        }
        private Rect rect = new Rect(0, 0, 0, 0);
        public Rect GetSpriteRect()
        {
            rect.SetAll(Transform.Position, ((Vector2f)sprite.Texture.Size).Multiply(sprite.Scale));
            return rect;
        }

        protected override void OnStart()
        {
            rectBehavior = GetBehavior<RectBehavior>() ?? AddBehavior<RectBehavior>(new RectBehavior());
        }

        protected override void OnUpdate()
        {
            rectBehavior = GetBehavior<RectBehavior>() ?? throw new BehaviorNotFoundException(typeof(RectBehavior));
            rectBehavior.Size = sprite.Scale.Multiply((Vector2f)sprite.Texture.Size);

        }

        public override void Render(RenderTarget target)
        {
            var t = GetTransform();
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