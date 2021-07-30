using SFML.System;
using System;
using System.Collections.Generic;

namespace MatrixEngine.GameObjects.Components.TilemapComponents {
    public sealed class TilemapComponent : Component {
        public readonly int pixelsPerUnit;
        internal Dictionary<Vector2i,Chunk> chunks;

        public const int chunkSize = 50;

        public Vector2f chunkRectSize { get => new Vector2f(chunkSize * transform.scale.X, chunkSize * transform.scale.Y); }

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

        public Tile GetTileFromWorldPos(Vector2f pos) {
            return GetTileFromTilemapPos((Vector2i)(new Vector2f(pos.X/transform.scale.X,pos.Y/transform.scale.Y) - gameObject.transform.position));
        }

        public Tile GetTileFromTilemapPos(Vector2i i) {
            var chunk_vec = new Vector2i((int)MathF.Floor((float)(i.X ) / chunkSize), (int)MathF.Floor((float)(i.Y ) / chunkSize)) *chunkSize;
            if (chunks.ContainsKey(chunk_vec)) {
                if ((i - chunk_vec).Y >chunkSize) {
                    Func<int> x = () => {
                        return 0;
                    };
                }
                return chunks[chunk_vec].GetTileFromLocalPosition(new Vector2i(i.X%chunkSize,i.Y%chunkSize));
                //Error!
            }
            return default;
        }
    }
}
