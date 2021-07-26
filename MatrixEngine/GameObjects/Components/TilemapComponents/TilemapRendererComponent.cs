using MatrixEngine.GameObjects.Components.RenderComponents;
using MatrixEngine.Physics;
using MatrixEngine.System;
using SFML.Graphics;
using SFML.System;
using System.Collections.Generic;

namespace MatrixEngine.GameObjects.Components.TilemapComponents {
    [RequireComponent(typeof(TilemapComponent))]
    public class TilemapRendererComponent : RendererComponent {

        private TilemapComponent tilemap;


        private Dictionary<Vector2i, RenderTexture> chunkTextures;

        public TilemapRendererComponent() {
            this.layer = -50;
            chunkTextures = new Dictionary<Vector2i, RenderTexture>();
        }




        private Vector2f add = new Vector2f(0.5f, 0.5f);
        public override void Start() {

            base.Start();

            tilemap = GetComponent<TilemapComponent>();

            RenderTexture();

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

                if (!item.Value.isRenderedUpdated) {

                    chunkTextures[item.Key] = RenderChunk(item.Value);
                    item.Value.isRenderedUpdated = true;


                }
            }
        }

        public RenderTexture RenderChunk(Chunk chunk) {
            var tex = new RenderTexture((uint)(chunk.chunkSize * tilemap.pixelsPerUnit), (uint)(chunk.chunkSize * tilemap.pixelsPerUnit));
            //var tex = new RenderTexture((uint)(chunk.chunkSize ), (uint)(chunk.chunkSize ));
            tex.Clear(Color.Transparent);

            foreach (var item in chunk) {

                var s = new Sprite(item.Value.texture);
                //item.Value.Texture.CopyToImage().GetPixel(0, 0).Log();
                s.Position = (Vector2f)item.Key * tilemap.pixelsPerUnit;
                //s.Scale /= tilemap.pixelsPerUnit;
                tex.Draw(s);
                tex.Display();

            }
            tex.Display();
            return tex;

        }


        public override void Render(RenderTarget target) {


            foreach (var item in chunkTextures) {
                if (!(new Rect((Vector2f)item.Key, (Vector2f)item.Value.Size)).isColliding(app.camera.rect)) {
                    continue;
                }
                var sprite = new Sprite(item.Value.Texture);
                sprite.Position = (Vector2f)item.Key;
                sprite.Scale /= tilemap.pixelsPerUnit;
                app.window.Draw(sprite);

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
