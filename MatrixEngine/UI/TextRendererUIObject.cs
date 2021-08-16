using System;
using System.Linq;
using MatrixEngine.Content;
using MatrixEngine.System;
using SFML.Graphics;
using SFML.System;
using SFML.Window;

namespace MatrixEngine.UI {
    public class TextRendererUIObject : UIObject {
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

        public TextRendererUIObject(Anchor anchor, string text, UITextStyle uiTextStyle,int layer,Action<UIObject,Vector2f,Mouse.Button> onClick,Action<UIObject,Vector2f> onHover) :
            base(anchor, uiTextStyle,layer,onHover,onClick) {
            this._text = text;
            this.style = uiTextStyle;
        }


        public override (Vector2f pos, Vector2f size) Render(RenderTarget target) {
            var pos = anchor.positionInPercentage.Multiply((Vector2f)target.Size) / 100;
            var size = anchor.maxSizeInPercentage.Multiply((Vector2f)target.Size) / 100;

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