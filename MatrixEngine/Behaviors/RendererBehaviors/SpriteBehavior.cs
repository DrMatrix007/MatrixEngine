using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.MatrixMath;
using SFML.Graphics;
using SFML.System;

namespace MatrixEngine.Behaviors.RendererBehaviors
{
    public class SpriteBehavior : RendererBehavior
    {
        private RectBehavior rectBehavior;


        private Sprite sprite;


        public SpriteBehavior( Texture texture, int layer) :base(layer)
        {
            sprite = new Sprite(texture);
        }
        private Rect rect = new Rect(0, 0, 0, 0);
        public Rect GetSpriteRect()
        {
            rect.SetAll(rectBehavior.Position, ((Vector2f)sprite.Texture.Size).Multiply(sprite.Scale));
            return rect;
        }

        protected override void OnStart()
        {
            rectBehavior = GetBehavior<RectBehavior>() ?? AddBehavior(new RectBehavior(new Rect(0, 0, 1, 1)));
        }

        protected override void OnUpdate()
        {
            rectBehavior = GetBehavior<RectBehavior>() ?? throw new BehaviorNotFoundException(typeof(RectBehavior));
            //rectBehavior.Size = sprite.Scale.Multiply((Vector2f)sprite.Texture.Size);

        }

        public override void Render(RenderTarget target)
        {

            sprite.Position = rectBehavior.Position;

            sprite.Scale = rectBehavior.Size.Devide(sprite.Texture.Size);

            target.Draw(sprite);
        }

        public override void Dispose()
        {
            sprite?.Dispose();
        }
    }
}