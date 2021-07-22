using MatrixEngine.Physics;
using SFML.Graphics;
using SFML.System;

namespace MatrixEngine.GameObjects.Components {
    [RequireComponent(typeof(RectComponent))]
    public sealed class SpriteRendererComponent : Component {

        private RectComponent rectComponent;

        private Sprite sprite;

        public Rect spriteRect
        {
            get => new Rect(position, new Vector2f(sprite.TextureRect.Width, sprite.TextureRect.Height));
        }

        public int layer
        {
            get;
            private set;
        }

        public SpriteRendererComponent(string localpathtoimg, int layer) {
            sprite = new Sprite(new Texture(localpathtoimg));
            //Debug.Log(sprite.Texture.ToString());
        }

        internal void Draw() {
            app.window.Draw(sprite);
        }

        public override void Start() {
            rectComponent = GetComponent<RectComponent>();

        }

        public override void Update() {
            sprite.Position = gameObject.position;
            app.spriteRenderer.addToDrawQueue(this);
            rectComponent.rect = spriteRect;
        }
    }
}
