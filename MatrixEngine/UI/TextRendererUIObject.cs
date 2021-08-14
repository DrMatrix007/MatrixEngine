using System;
using System.Linq;
using MatrixEngine.Content;
using MatrixEngine.System;
using SFML.Graphics;
using SFML.System;

namespace MatrixEngine.UI {
    public  class TextRendererUIObject : UIObject {
        private Font _font;

        public Font font {
            get => _font;
            set {
                _font = value;
                CreateText();
            }
        }

        private string _text;

        public string text {
            get => _text;
            set {
                _text = value;
                CreateText();
            }
        }

        private uint _charsize;


        public uint charSize {
            get => _charsize;
            set {
                _charsize = value;
                CreateText();
            }
        }


        private Text drawable;
        public bool isAutoSize;

        private void CreateText() {
            drawable?.Dispose();
            drawable = new Text(text, font, charSize);
        }

        public TextRendererUIObject(Anchor anchor, string text, Font font, bool auto_size = true, uint charsize = 10) :
            base(anchor) {
            this._text = text;
            this.font = font;
            this.charSize = charsize;
            this.isAutoSize = auto_size;
        }


        public TextRendererUIObject(Anchor anchor, string text, bool isAutoSize = true, uint charsize = 10) : this(
            anchor, text, FontManager.CascadiaCode, isAutoSize, charsize) {
        }

        public override void OnHover(Vector2f hoverPos) {
        }

        public override void OnClick(Vector2f clickPos) {
        }

        public override void Render(RenderTarget target) {
            var pos = anchor.positionInPercentage.Multiply((Vector2f) target.Size)/100;
            var size = anchor.maxSizeInPercentage.Multiply((Vector2f) target.Size)/100;

            var list = text.Split("\n");
            var longest = list.Aggregate((max, cur) => max.Length > cur.Length ? max : cur);

            if (isAutoSize) {
                charSize = (uint)Math.Min((size.X / longest.Length),(size.Y/list.Length));
            }
            drawable.Position = pos;


            
            target.Draw(drawable);
        }
    }
}