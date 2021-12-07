using SFML.Graphics;
using SFML.System;
using MatrixEngine.MatrixMath;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Behaviors.RendererBehaviors
{
    public class ImageBehavior : UserInterfaceBehavior
    {
        public Sprite sprite;

        public ImageBehavior(Texture texture)
        {
            sprite = new Sprite(texture);
        }

        public override void Dispose()
        {
            sprite?.Dispose();
        }

        public override void Render(RenderTarget target)
        {
            Vector2f windowSize;
            Vector2f windowSpritePos;
            Vector2f windowSpriteSize;

            float newScale;
            float widthHeightRatio;
            float maxPixels;


            RectangleShape shape;


            windowSize = ((Vector2f)target.Size);
            windowSpritePos = windowSize.Multiply(AnchorBehavior.Position);
            windowSpriteSize = windowSize.Multiply(AnchorBehavior.Size);

            widthHeightRatio = windowSize.X / windowSize.Y;


            sprite.Position = AnchorBehavior.Position;
            sprite.Scale = AnchorBehavior.Size;

            maxPixels = MathF.Max(sprite.Texture.Size.X, sprite.Texture.Size.Y);

            if (sprite.Texture.Size.X > maxPixels)
            {
                newScale = sprite.Texture.Size.Y / windowSpriteSize.Y;
                sprite.Scale = new Vector2f(newScale / widthHeightRatio, newScale);
            }
            else if (sprite.Texture.Size.Y > maxPixels)
            {
                newScale = sprite.Texture.Size.X / windowSpriteSize.X;
                sprite.Scale = new Vector2f(newScale / widthHeightRatio, newScale);
            }
            else
            {
                sprite.Scale = AnchorBehavior.Size.Devide(sprite.Texture.Size);
            }
            shape = new RectangleShape() { Position = AnchorBehavior.Position, Size = AnchorBehavior.Size, FillColor = Color.Blue };
            target.Draw(shape);
            //target.Draw(sprite);
            //target.Draw(shape);


            shape.Dispose();




        }
    }
}
