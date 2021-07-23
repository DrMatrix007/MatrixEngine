using MatrixEngine.Physics;
using SFML.Graphics;
using SFML.System;

namespace MatrixEngine.GameObjects.Components.RenderComponents {
    [RequireComponent(typeof(RectComponent))]
    public sealed class SpriteRendererComponent : RendererComponent {

        private RectComponent rectComponent;

        private Sprite sprite;
        public int pixelperunit;

        public Rect spriteRect
        {
            get => new Rect(position, new Vector2f(sprite.TextureRect.Width, sprite.TextureRect.Height));
        }



        public SpriteRendererComponent(string localpathtoimg, int pixelperunit, int layer) {
            sprite = new Sprite(new Texture(localpathtoimg));
            this.layer = layer;
            this.pixelperunit = pixelperunit;
        }

        public override void Render(RenderTarget target) {
            target.Draw(sprite);
        }

        public override void Start() {
            rectComponent = GetComponent<RectComponent>();

        }

        public override void Update() {
            sprite.Position = gameObject.position;
            app.renderer.addToDrawQueue(this);
            var trect = sprite.TextureRect;

            var new_sprite_rect = spriteRect;


            new_sprite_rect.SetSize(new Vector2f(trect.Width, trect.Height) / pixelperunit);

            sprite.Scale = new Vector2f(1, 1) / pixelperunit;
            //Debug.Log(sprite.Scale);

            rectComponent.rect = new_sprite_rect;
        }
    }
}
