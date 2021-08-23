using System;
using System.Linq;
using MatrixEngine.Content;
using MatrixEngine.System;
using SFML.Graphics;
using SFML.System;
using SFML.Window;
using MathUtils = MatrixEngine.System.Math.MathUtils;

namespace MatrixEngine.UI {
    public abstract class TextRendererUIObject : UIObject {
        public new UITextStyle style;

        private string _text;

        public string text {
            get => _text;
            set {
                _text = value;
                CreateText();
            }
        }


        private Text drawable;

        private void CreateText() {
            drawable?.Dispose();

            drawable = new Text(text, style.font, style.char_size);
        }

        public TextRendererUIObject(Anchor anchor, string text, UITextStyle uiTextStyle,int layer) :
            base(anchor, uiTextStyle,layer) {
            _text = text;
            style = uiTextStyle;
        }


        public override (Vector2f pos, Vector2f size) Render(RenderTarget target) {
            var pos = MathUtils.Multiply(anchor.positionInPercentage, (Vector2f)target.Size) / 100;
            var size = MathUtils.Multiply(anchor.maxSizeInPercentage, (Vector2f)target.Size) / 100;

            var list = text.Split("\n");
            var longest = list.Aggregate((max, cur) => max.Length > cur.Length ? max : cur);

            if (style.is_resize) {
                style.char_size = (uint)Math.Min((size.X / longest.Length), (size.Y / list.Length));
            }

            if (drawable.GetGlobalBounds().Height > size.Y) {
                style.char_size = (uint)(drawable.GetGlobalBounds().Height / size.Y);
            }
            
            drawable.Position = pos;
            CreateText();
            target.Draw(new RectangleShape()
                { Position = pos, Size = size, FillColor = style.BackgroundColor });
            target.Draw(drawable);

            return (pos, size);
        }
    }
}