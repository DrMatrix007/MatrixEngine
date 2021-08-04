﻿using MatrixGDK.Content;
using MatrixGDK.GameObjects.Components.PhysicsComponents;
using MatrixGDK.Physics;
using MatrixGDK.System;
using SFML.Graphics;
using SFML.System;
using System;
using System.Diagnostics;

namespace MatrixGDK.GameObjects.Components.RenderComponents {
    public sealed class SpriteRendererComponent : RendererComponent {


        private Sprite sprite;
        public int pixelperunit;

        public Rect spriteRect
        {
            get => new Rect(position, new Vector2f(sprite.TextureRect.Width, sprite.TextureRect.Height));
        }



        public SpriteRendererComponent(string localpathtoimg, int pixelperunit, int layer) {

            sprite = new Sprite(TextureManager.GetTexture(localpathtoimg));
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
