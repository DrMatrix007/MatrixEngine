using System;
using MatrixEngine.Physics;
using MatrixEngine.System;
using SFML.Graphics;
using SFML.System;
using SFML.Window;

namespace MatrixEngine.UI {
    public sealed class SpriteRendererUIObject : UIObject {
        public readonly new Sprite drawable;

        public Texture texture {
            get => drawable.Texture;
            set => drawable.Texture = value;
        }

        public SpriteRendererUIObject(Anchor anchor, Texture t, UIStyle style, int layer,
            Action<UIObject, Vector2f> OnClickTexture,
            Action<UIObject, Vector2f> OnClickRect,
            Action<UIObject, Vector2f> OnHover) : base(anchor, style, layer, OnHover,
            delegate(UIObject sender, Vector2f clickPos, Mouse.Button button) {
                var current = sender as SpriteRendererUIObject;
                if (new Rect(current.drawable.Position,
                        current.drawable.Scale.Multiply((Vector2f)current.drawable.Texture.Size))
                    .IsInside(clickPos)) {
                    current.OnClickTexture(sender, clickPos);
                }
                else {
                    current.OnClickRect(sender, clickPos);
                }
            }) {
            this.OnClickRect = OnClickRect;
            this.OnClickTexture = OnClickTexture;
            this.drawable = new Sprite(t);
        }


        private Action<UIObject, Vector2f> OnClickTexture;
        private Action<UIObject, Vector2f> OnClickRect;


        public override (Vector2f pos, Vector2f size) Render(RenderTarget target) {
            var t = texture;
            Vector2f set_pos = (anchor.positionInPercentage / 100).Multiply((Vector2f)target.Size);
            Vector2f max_size = (anchor.maxSizeInPercentage / 100).Multiply((Vector2f)target.Size);

            if (Math.Abs(t.Size.X * drawable.Scale.X - max_size.X) > 0.001 ||
                Math.Abs(t.Size.Y * drawable.Scale.Y - max_size.Y) > 0.001) {
                var s = max_size.X / drawable.TextureRect.Width;
                drawable.Scale = new Vector2f(s, s);

                if (t.Size.Y * drawable.Scale.Y > max_size.Y) {
                    s = max_size.Y / drawable.TextureRect.Height;
                    drawable.Scale = new Vector2f(s, s);
                }
            }

            drawable.Position = set_pos;


            drawable.Color = style.color;

            target.Draw(new RectangleShape()
                { Position = set_pos, Size = max_size, FillColor = style.BackgroundColor });

            target.Draw(drawable);

            return (set_pos, max_size);
        }
    }
}