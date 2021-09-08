using MatrixEngine.Physics;
using SFML.Graphics;
using SFML.System;
using System;

namespace MatrixEngine.GameObjects.Components.RenderComponents {

    public sealed class SpriteRendererComponent : RendererComponent {
        private Sprite sprite;
        public int pixelPerUnit;

        public void SetTexture(Texture texture, int pixelperunit) {
            sprite.Texture?.Dispose();
            sprite.Texture = texture;
            this.pixelPerUnit = pixelperunit;
        }

        public Rect textureRect
        {
            get => new Rect(position, new Vector2f(sprite.TextureRect.Width, sprite.TextureRect.Height));
        }

        public SpriteRendererComponent(string localpathtoimg, int pixelperunit, int layer) {
            if (!string.IsNullOrEmpty(localpathtoimg)) {
                sprite = new Sprite(new Texture(localpathtoimg));
            }
            this.layer = layer;
            this.pixelPerUnit = pixelperunit;
        }

        public void SetTexture(string localpathtoimg, int pixelperunit) {
            if (!string.IsNullOrEmpty(localpathtoimg)) {
                sprite = new Sprite(new Texture(localpathtoimg));
            } else {
                throw new Exception($"string is null/empty");
            }
            pixelPerUnit = pixelperunit;
        }

        public void SetTexture(string localpathtoimg) {
            SetTexture(localpathtoimg, -1);
            pixelPerUnit = sprite.TextureRect.Width;
        }

        public SpriteRendererComponent() : this("", 1, -1) {
            sprite = new Sprite();
        }

        public override void Render(RenderTarget target) {
            target.Draw(sprite);
        }

        public override void Start() {
            // var c = this.GetComponent<ColliderComponent>();
            // if (c != null && c.colliderType == ColliderComponent.ColliderType.Rect) {
            var tr = sprite.TextureRect;
            transform.rect = new Rect(position, new Vector2f(tr.Width, tr.Height) / pixelPerUnit);
            // }
        }

        public override void Update() {
            sprite.Position = gameObject.position;
            app.spriteRenderer.AddToQueue(this);

            sprite.Scale = transform.scale / pixelPerUnit;
            //Debug.Log(sprite.Scale);
        }
    }
}