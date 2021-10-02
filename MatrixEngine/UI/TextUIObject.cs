using MatrixEngine.Framework;
using SFML.Graphics;
using SFML.System;
using MathUtils = MatrixEngine.Framework.MathUtils;

namespace MatrixEngine.UI {

    public abstract class TextUIObject : UIObject {
        public new UITextStyle style;

        private RectangleShape backrground = new RectangleShape();

        private string _text;

        public string text
        {
            get => _text;
            set {
                _text = value;
                CreateText();
            }
        }

        private new Text drawable;

        private void CreateText() {
            drawable?.Dispose();

            drawable = new Text(text, style.font, style.char_size) { FillColor = style.color };
        }

        public TextUIObject(Anchor anchor, string text, UITextStyle uiTextStyle) :
            base(anchor, uiTextStyle) {
            _text = text;
            style = uiTextStyle;
        }

        public override (Vector2f pos, Vector2f size) Render(RenderTarget target) {
            var pos = MathUtils.Multiply(anchor.positionInPercentage, (Vector2f)target.Size) / 100;
            var size = MathUtils.Multiply(anchor.maxSizeInPercentage, (Vector2f)target.Size) / 100;

            CreateText();

            if (style.is_resize) {
                var w = drawable.GetLocalBounds().Width;
                var h = drawable.GetLocalBounds().Height;

                var wratio = size.X / w;
                var hratio = size.Y / h;

                if (wratio > hratio) {
                    drawable.CharacterSize = (uint)((hratio * drawable.CharacterSize).Floor() - 1);
                } else {
                    drawable.CharacterSize = (uint)((wratio * drawable.CharacterSize).Floor() - 1);
                }
            }

            if (drawable.GetGlobalBounds().Height > size.Y) {
                style.char_size = (uint)(drawable.GetGlobalBounds().Height / size.Y);
            }

            backrground.Position = pos;
            backrground.Size = size;
            backrground.FillColor = style.BackgroundColor;

            drawable.Position = pos;
            target.Draw(backrground);
            target.Draw(drawable);

            return (pos, size);
        }
    }
}