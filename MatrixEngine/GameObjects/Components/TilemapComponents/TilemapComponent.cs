using MatrixEngine.Physics;
using SFML.System;
using System;
using System.Collections.Generic;

namespace MatrixEngine.GameObjects.Components.TilemapComponents {

    public sealed class TilemapComponent : Component {
        public int pixelsPerUnit;
        internal Dictionary<Vector2i, Chunk> chunks;

        public const int chunkSize = 50;

        public Vector2f ChunkRectSize { get => new(chunkSize * Transform.scale.X, chunkSize * Transform.scale.Y); }

        public TilemapComponent() : this(16) {
        }

        public Rect TileRect
        {
            get => new(0, 0, Transform.scale.X, Transform.scale.Y);
        }

        public TilemapComponent(int pixelperunit) {
            this.pixelsPerUnit = pixelperunit;

            chunks = new Dictionary<Vector2i, Chunk>();
        }

        public void SetTile(Vector2i i, Tile tile) {
            var chunk_vec = new Vector2i((int)Math.Floor((float)i.X / chunkSize), (int)Math.Floor((float)i.Y / chunkSize)) * chunkSize;
            if (!chunks.ContainsKey(chunk_vec)) {
                chunks[chunk_vec] = new Chunk(chunk_vec, chunkSize);
            }
            chunks[chunk_vec].isRenderedUpdated = false;
            chunks[chunk_vec].SetTileFromLocalPos(GetLocalChunkPos(i, chunk_vec), tile);
        }

        public void SetTile(int x, int y, Tile tile) {
            SetTile(new Vector2i(x, y), tile);
        }

        public Tile GetTileFromTilemapPos(Vector2i i) {
            var chunk_vec = new Vector2i((int)MathF.Floor((float)(i.X) / chunkSize), (int)MathF.Floor((float)(i.Y) / chunkSize)) * chunkSize;
            if (chunks.ContainsKey(chunk_vec)) {
                return chunks[chunk_vec].GetTileFromLocalPosition(GetLocalChunkPos(i, chunk_vec));
            }
            return default;
        }

        private static Vector2i GetLocalChunkPos(Vector2i i, Vector2i chunk_pos) {
            var pos = i - chunk_pos;
            if (pos.X < 0) {
                pos.X = chunkSize + pos.X;
            }
            if (pos.Y < 0) {
                pos.Y = chunkSize + pos.Y;
            }
            return pos;
        }

        public override void LateUpdate() {
        }

        public override void Start() {
        }

        public override void Update() {
        }

        public Tile GetTileFromWorldPos(Vector2f pos) {
            return GetTileFromTilemapPos(GetPosOfTileFromWorldPos(pos));
        }

        public Vector2i GetPosOfTileFromWorldPos(Vector2f pos) {
            return (Vector2i)(new Vector2f(pos.X / Transform.scale.X, pos.Y / Transform.scale.Y) - GameObject.Transform.position);
        }

        public void Clear() {
            chunks.Clear();
        }
    }
}