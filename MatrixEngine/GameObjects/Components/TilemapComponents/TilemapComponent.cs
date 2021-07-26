using SFML.System;
using System.Collections.Generic;

namespace MatrixEngine.GameObjects.Components.TilemapComponents {
    public sealed class TilemapComponent : Component {
        public readonly int pixelsPerUnit;
        internal Dictionary<Vector2i,Chunk> chunks;

        public const int chunkSize = 50;

        public TilemapComponent() : this(16) {

        }

        public TilemapComponent(int pixelperunit) {
            this.pixelsPerUnit = pixelperunit;

            chunks = new Dictionary<Vector2i, Chunk>();
        }
        public void SetTile(Vector2i i, Tile tile) {
            var chunk_vec = new Vector2i(i.X/chunkSize, i.Y/chunkSize)*chunkSize;
            if (!chunks.ContainsKey(chunk_vec)) {
                chunks[chunk_vec] = new Chunk(chunk_vec,chunkSize);
            }
            chunks[chunk_vec].isRenderedUpdated = false;
            chunks[chunk_vec].SetTileFromLocalPos(i-chunk_vec,tile);



        }
        public override void LateUpdate() {

        }

        public override void Start() {
        }

        public override void Update() {
        }
    }
}
