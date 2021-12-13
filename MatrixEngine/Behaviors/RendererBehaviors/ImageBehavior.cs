using SFML.Graphics;
using SFML.System;
using MatrixEngine.MatrixMath;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using SFML.Window;
using MatrixEngine.Utils;

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



        public override bool IsOverlapping(Vector2f pos)
        {
            return sprite.GetGlobalBounds().Contains(pos.X,pos.Y);
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

            if (Math.Abs(sprite.Texture.Size.X * sprite.Scale.X - windowSpriteSize.X) > 0.001 ||
                Math.Abs(sprite.Texture.Size.Y * sprite.Scale.Y - windowSpriteSize.Y) > 0.001)
            {


                // if (sprite.TextureRect.Width*sprite.Scale.X > maxPixels)
                // {
                newScale = (float)windowSpriteSize.X / sprite.TextureRect.Width;
                sprite.Scale = new Vector2f(newScale, newScale);
                // }
                if (sprite.TextureRect.Height * sprite.Scale.Y > windowSpriteSize.Y)
                {
                    newScale = (float)windowSpriteSize.Y / sprite.TextureRect.Height;
                    sprite.Scale = new Vector2f(newScale, newScale);
                }
            }

            shape = new RectangleShape() { Position = windowSpritePos, Size = windowSpriteSize, FillColor = Color.Blue };
            target.Draw(shape);
            target.Draw(sprite);


            shape.Dispose();




        }
    }
}
