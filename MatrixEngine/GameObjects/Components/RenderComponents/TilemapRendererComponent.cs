using MatrixEngine.Framework;
using MatrixEngine.GameObjects.Components.TilemapComponents;
using MatrixEngine.Physics;
using SFML.Graphics;
using SFML.System;
using System.Collections.Generic;
using System.Linq;

namespace MatrixEngine.GameObjects.Components.RenderComponents {

    [RequireComponent(typeof(TilemapComponent))]
    public class TilemapRendererComponent : RendererComponent {
        private TilemapComponent tilemap;

        internal readonly Dictionary<Vector2i, RenderTexture> chunkTextures;

        public TilemapRendererComponent() {
            layer = -50;
            chunkTextures = new Dictionary<Vector2i, RenderTexture>();
        }

        //private Vector2f add = new(0.5f, 0.5f);
        private Sprite sprite;

        public override void Start() {
            base.Start();

            tilemap = GetComponent<TilemapComponent>();
        }

        public override void Update() {
            base.Update();
            RenderTexture();
        }

        public void RenderTexture() {
            //foreach (var c in tilemap.chunks) {
            //    chunkTextures[c.Key] = RenderChunk(c.Value);
            //    c.Value.isRenderedUpdated = true;

            //}

            foreach (var item in tilemap.chunks) {
                if (!item.Value.isRenderedUpdated && new Rect(((Vector2f)item.Key).Multiply(tilemap.Transform.scale), item.Value.size.Multiply(tilemap.Transform.scale)).isColliding(App.camera.Rect)) {
                    if (chunkTextures.ContainsKey(item.Key)) {
                        chunkTextures[item.Key].Dispose();
                        chunkTextures.Remove(item.Key);
                    }
                    chunkTextures[item.Key] = RenderChunk(item.Value);
                    item.Value.isRenderedUpdated = true;
                }
                if (!new Rect(((Vector2f)item.Key).Multiply(tilemap.Transform.scale) + GameObject.Position, tilemap.ChunkRectSize.Multiply(tilemap.Transform.scale)).isColliding(App.camera.Rect)) {
                    if (chunkTextures.ContainsKey(item.Key)) {
                        chunkTextures[item.Key].Dispose();
                        chunkTextures.Remove(item.Key);
                    }
                } else {
                    if (!chunkTextures.ContainsKey(item.Key)) {
                        chunkTextures[item.Key] = RenderChunk(item.Value);
                        item.Value.isRenderedUpdated = true;
                    }
                }
            }
        }

        public RenderTexture RenderChunk(Chunk chunk) {
            var tex = new RenderTexture((uint)(chunk.chunkSize * tilemap.pixelsPerUnit), (uint)(chunk.chunkSize * tilemap.pixelsPerUnit));
            //var tex = new RenderTexture((uint)(chunk.chunkSize ), (uint)(chunk.chunkSize ));
            tex.Clear(Color.Transparent);

            foreach (var item in chunk) {
                var s = new Sprite(item.Value.texture) {
                    Position = (Vector2f)item.Key * tilemap.pixelsPerUnit,
                    Color = item.Value.color
                };
                //s.Scale /= tilemap.pixelsPerUnit;
                tex.Draw(s);
                tex.Display();
                s.Dispose();
            }
            tex.Display();
            return tex;
        }

        public override void Render(RenderTarget target) {
            foreach (var item in chunkTextures.ToList()) {
                if (!new Rect(((Vector2f)item.Key).Multiply(tilemap.Transform.scale) + GameObject.Position, (Vector2f)new Vector2f(item.Value.Size.X * Transform.scale.X, item.Value.Size.Y * Transform.scale.Y) * 2.0f / tilemap.pixelsPerUnit).isColliding(App.camera.Rect)) {
                    item.Value.Dispose();
                    chunkTextures.Remove(item.Key);
                    continue;
                }
                if (sprite != null) {
                    sprite.Dispose();
                }
                if (tilemap.chunks.ContainsKey(item.Key)) {
                    sprite = new Sprite(item.Value.Texture) {
                        Texture = (item.Value.Texture),
                        Position = GameObject.Position
                    };
                    sprite.Position += new Vector2f(item.Key.X * Transform.scale.X, item.Key.Y * Transform.scale.Y);
                    sprite.Scale /= tilemap.pixelsPerUnit;
                    sprite.Scale = new Vector2f(sprite.Scale.X * Transform.scale.X, sprite.Scale.Y * Transform.scale.Y);
                    App.Window.Draw(sprite);
                } else {
                    item.Value.Dispose();
                    chunkTextures.Remove(item.Key);
                }
            }

            //foreach (var item in tilemap.chunks) {
            //    foreach (var tile in item.Value) {
            //        var s = new Sprite(tile.Value.texture);
            //        s.Position = (Vector2f)(tile.Key + item.Key);
            //        s.Scale /= tilemap.pixelsPerUnit;
            //        app.window.Draw(s);

            //    }
            //}
        }
    }
}