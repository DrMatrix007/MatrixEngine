using MatrixEngine.Content;
using MatrixEngine.System;
using SFML.Graphics;
using SFML.System;
using System;
using System.Diagnostics;
using MatrixEngine.GameObjects.Components.PhysicsComponents;
using MatrixEngine.Physics;

namespace MatrixEngine.GameObjects.Components.RenderComponents {
    public sealed class SpriteRendererComponent : RendererComponent {


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
            var c = this.GetComponent<ColliderComponent>();
            if (c != null && c.colliderType == ColliderComponent.ColliderType.Rect) {
                var tr = sprite.TextureRect;
                transform.rect = new Rect(position,new Vector2f(tr.Width,tr.Height)/pixelperunit);
            }
        }


        public override void Update() {
            sprite.Position = gameObject.position;
            app.renderer.AddToDrawQueue(this);

            sprite.Scale = new Vector2f(transform.scale.X, transform.scale.Y) / (float)pixelperunit;
            //Debug.Log(sprite.Scale);


            

        }
    }
}
