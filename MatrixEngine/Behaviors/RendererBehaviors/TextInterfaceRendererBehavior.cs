using MatrixEngine.MatrixMath;
using SFML.Graphics;
using SFML.System;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

using MatrixEngine.Utils;

namespace MatrixEngine.Behaviors.RendererBehaviors
{
    public class TextInterfaceRendererBehavior : UserInterfaceBehavior
    {
        private Text text = new Text();


        public TextInterfaceRendererBehavior(string text, int layer) : base(layer)
        {
            this.text = new Text(text, new Font("CascadiaMono.ttf"));
        }

        public override void Dispose()
        {
        }

        public override bool IsOverlapping(Vector2f pos)
        {
            return text.GetGlobalBounds().Contains(pos.X, pos.Y);
        }

        public override void Render(RenderTarget target)
        {
            Vector2f windowSize = ((Vector2f)target.Size);
            var textSize = windowSize.Multiply(AnchorBehavior.Size);
            var windowTextPos = windowSize.Multiply(AnchorBehavior.Position);
            var w = text.GetGlobalBounds().Width;
            var h = text.GetGlobalBounds().Height;

            //text.GetGlobalBounds().Top.Log();
            //text.GetGlobalBounds().Left.Log();


            var wratio = textSize.X/w;
            var hratio =( textSize.Y/1.5f)/h;

            if (wratio >= hratio)
            {
                text.CharacterSize = (uint)(text.CharacterSize * (hratio/10)).Ceil()*10;
            }
            else
            {
                text.CharacterSize = (uint)(text.CharacterSize * ((wratio/10))).Ceil()*10;
            }


            //if (text.GetGlobalBounds().Height > size.Y)
            //{
            //    style.char_size = (uint)(text.GetGlobalBounds().Height / size.Y);
            //}

            //backrground.Position = pos;
            //backrground.Size = size;
            //backrground.FillColor = style.BackgroundColor;
            text.Position = windowTextPos;


            //GetEngine().QuickDrawRect(new Rect(windowTextPos,textSize), new Color(10,100,100,100));
            //GetEngine().QuickDrawRect(new Rect(text.GetGlobalBounds()), new Color(100, 100, 100, 100));
            //target.Draw(backrground);
            target.Draw(text);

        }
    }
}
