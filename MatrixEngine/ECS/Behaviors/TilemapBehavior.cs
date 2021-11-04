using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using SFML.Graphics;
using SFML.System;

namespace MatrixEngine.ECS.Behaviors
{
    public class Tile
    {
        public Tile(Texture texture)
        {
            this.Texture = texture;
        }

        public Texture Texture;
    }

    public class Chunk
    {
        public readonly UInt16 CHUNKSIZE;

        internal readonly Dictionary<Vector2i, Tile> tiles = new Dictionary<Vector2i, Tile>();

        public Chunk(UInt16 size)
        {
            CHUNKSIZE = size;
        }

        public void SetTile(Vector2i vector2, Tile tile)
        {
            if (tile == null)
            {
                tiles.Remove(vector2);
            }
            else
            {
                tiles[vector2] = tile;
            }
        }
    }

    public class TilemapBehavior : Behavior
    {
        public readonly UInt16 CHUNKSIZE = 2;

        private Dictionary<Vector2i, Chunk> chunks = new Dictionary<Vector2i, Chunk>();

        public IEnumerable<KeyValuePair<Vector2i, Tile>> tiles
        {
            get
            {
                foreach (var chunk in chunks)
                {
                    foreach (var valueTile in chunk.Value.tiles)
                    {

                        yield return new KeyValuePair<Vector2i, Tile>(chunk.Key+valueTile.Key,valueTile.Value);
                    }
                }
            }
        }

        public Tile SetTile(Vector2i pos, Tile tile)
        {
            var chunk_pos = GetChunkPos(pos);

            if (!chunks.ContainsKey(chunk_pos))
            {
                chunks[chunk_pos] = new Chunk(CHUNKSIZE);
            }

            chunks[chunk_pos].SetTile(pos - chunk_pos, tile);

            return tile;
        }

        public Vector2i GetChunkPos(Vector2i vector2)
        {
            return ((Vector2f)vector2 / CHUNKSIZE).Floor() * CHUNKSIZE;
        }

        protected override void OnStart()
        {
        }

        protected override void OnUpdate()
        {
        }

        public override void Dispose()
        {
        }
    }
}