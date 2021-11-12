using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.MatrixMath;
using MatrixEngine.Utils;
using SFML.Graphics;
using SFML.System;

namespace MatrixEngine.ECS.Behaviors
{
    public class TilemapRendererBehavior : RendererBehavior
    {
        public TilemapRendererBehavior(float pixelsPerUnit)
        {
            PixelsPerUnit = pixelsPerUnit;
        }

        public float PixelsPerUnit { get; set; }

        private TilemapBehavior tilemap;

        private readonly Dictionary<Vector2i, RenderTexture> chunkTextures = new Dictionary<Vector2i, RenderTexture>();
        private readonly List<Vector2i> nonUpdatedChunks = new List<Vector2i>();

        //private Vector2f add = new(0.5f, 0.5f);
        private Sprite sprite;

        protected override void OnStart()
        {
            tilemap = GetBehavior<TilemapBehavior>();
            if (tilemap == null)
            {
                throw new NullReferenceException($"Tile map is null in {this}");
            }
            tilemap.TilePlaced += TilePlaced;
        }

        private void TilePlaced(object? sender, TilePlacementEventArgs e)
        {
            if (!nonUpdatedChunks.Contains(e.ChunkPos))
            {
                nonUpdatedChunks.Add(e.ChunkPos);
            }
        }

        protected override void OnUpdate()
        {
            Logging.Assert(GetBehavior<TilemapBehavior>()==tilemap,"tilemap has changed! (it shouldn't)");
            RenderTexture();
        }

        public override void Dispose()
        {
        }

        public void RenderTexture()
        {
            //foreach (var c in tilemap.chunks) {
            //    chunkTextures[c.Key] = RenderChunk(c.Value);
            //    c.Value.isRenderedUpdated = true;

            //}

            var trans = GetTransform();
            var camrect = GetEngine().Window.GetView().ToRect();
            foreach (var item in tilemap.chunks)
            {
                var iscolliding = new Rect(((Vector2f)item.Key).Multiply(trans.Scale) + trans.Position,
                    tilemap.CHUNKSIZE * new Vector2f(1, 1).Multiply(trans.Scale)).IsColliding(camrect);

                if ((nonUpdatedChunks.Contains(item.Key) && iscolliding) || (iscolliding && !chunkTextures.ContainsKey(item.Key)))
                {
                    if (chunkTextures.ContainsKey(item.Key))
                    {
                        chunkTextures[item.Key].Dispose();
                        chunkTextures.Remove(item.Key);
                    }
                    chunkTextures[item.Key] = this.RenderChunk(item.Value);
                    nonUpdatedChunks.Remove(item.Key);
                }
                if (iscolliding && !chunkTextures.ContainsKey(item.Key))
                {
                }
                else if (!iscolliding)
                {
                    if (chunkTextures.ContainsKey(item.Key))
                    {
                        chunkTextures[item.Key].Dispose();
                        chunkTextures.Remove(item.Key);
                    }
                }
            }
            //    else
            //{
            //    if (!chunkTextures.ContainsKey(item.Key))
            //    {
            //        chunkTextures[item.Key] = RenderChunk(item.Value);
            //        nonUpdatedChunks.Remove(item.Key);
            //    }
            //}
            nonUpdatedChunks.Clear();
        }

        public RenderTexture RenderChunk(Chunk chunk)
        {
            var tex = new RenderTexture((uint)(chunk.CHUNKSIZE * PixelsPerUnit), (uint)(chunk.CHUNKSIZE * PixelsPerUnit));
            //var tex = new RenderTexture((uint)(chunk.chunkSize ), (uint)(chunk.chunkSize ));
            tex.Clear(Color.Transparent);

            foreach (var item in chunk)
            {
                var s = new Sprite(item.Value.Texture)
                {
                    Position = (Vector2f)item.Key * PixelsPerUnit,
                    //Color = item.Value.color
                };
                //s.Scale /= tilemap.pixelsPerUnit;
                tex.Draw(s);
                tex.Display();
                s.Dispose();
            }
            tex.Display();
            return tex;
        }

        public override void Render(RenderTarget target)
        {
            var camrect = GetEngine().Window.GetView().ToRect();
            var trans = GetTransform();

            foreach (var item in chunkTextures.ToList())
            {
                var iscolliding = new Rect(((Vector2f)item.Key).Multiply(trans.Scale) + trans.Position,
                tilemap.CHUNKSIZE * new Vector2f(1, 1).Multiply(trans.Scale)).IsColliding(camrect);
                if (!iscolliding)

                {
                    item.Value.Dispose();
                    chunkTextures.Remove(item.Key);
                    continue;
                }

                sprite?.Dispose();

                if (tilemap.chunks.ContainsKey(item.Key))
                {
                    sprite = new Sprite(item.Value.Texture)
                    {
                        Texture = (item.Value.Texture),
                        Position = trans.Position
                    };
                    sprite.Position += new Vector2f(item.Key.X * trans.Scale.X, item.Key.Y * trans.Scale.Y);
                    sprite.Scale /= PixelsPerUnit;
                    sprite.Scale = new Vector2f(sprite.Scale.X * trans.Scale.X, sprite.Scale.Y * trans.Scale.Y);
                    target.Draw(sprite);
                }
                else
                {
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