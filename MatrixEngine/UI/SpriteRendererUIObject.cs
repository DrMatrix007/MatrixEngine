using System;
using MatrixEngine.System;
using SFML.Graphics;
using SFML.System;

namespace MatrixEngine.UI {
    public sealed class SpriteRendererUIObject : UIObject {
        private readonly Sprite drawable;

        public Texture texture {
            get => drawable.Texture;
            set => drawable.Texture = value;
        }

        public SpriteRendererUIObject(Anchor anchor, Texture t) : base(anchor) {
            this.drawable = new Sprite(t);
        }

        public override void OnHover(Vector2f hoverPos) {
        }

        public override void OnClick(Vector2f clickPos) {
        }

        public override void Render(RenderTarget target) {
            var t = texture;
            Vector2f set_pos = (anchor.positionInPercentage / 100).Multiply((Vector2f) target.Size);
            Vector2f max_size = (anchor.maxSizeInPercentage / 100).Multiply((Vector2f) target.Size);

            if (Math.Abs(t.Size.X * drawable.Scale.X - max_size.X) > 0.001 ||
                Math.Abs(t.Size.Y * drawable.Scale.Y - max_size.Y) > 0.001) {
                var s = max_size.X / drawable.TextureRect.Width;
                drawable.Scale = new Vector2f(s, s);

                if (t.Size.Y*drawable.Scale.Y > max_size.Y) {
                    s = max_size.Y / drawable.TextureRect.Height;
                    drawable.Scale = new Vector2f(s, s);
                }
            }

            drawable.Position = set_pos;
            
            
            target.Draw(drawable);
        }
    }
}