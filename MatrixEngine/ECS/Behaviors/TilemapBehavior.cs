using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.MatrixMath;
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

    public class Chunk : IEnumerable<KeyValuePair<Vector2i, Tile>>
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

        public IEnumerator<KeyValuePair<Vector2i, Tile>> GetEnumerator()
        {
            return tiles.GetEnumerator();
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            return GetEnumerator();
        }
    }

    public class TilemapBehavior : Behavior
    {
        public readonly UInt16 CHUNKSIZE = 50;

        internal Dictionary<Vector2i, Chunk> chunks = new Dictionary<Vector2i, Chunk>();

        public EventHandler<TilePlacementEventArgs> TilePlaced;

        public IEnumerable<KeyValuePair<Vector2i, Tile>> tiles
        {
            get
            {
                foreach (var chunk in chunks)
                {
                    foreach (var valueTile in chunk.Value.tiles)
                    {
                        yield return new KeyValuePair<Vector2i, Tile>(chunk.Key + valueTile.Key, valueTile.Value);
                    }
                }
            }
        }

        public Tile SetTile(Vector2i pos, Tile tile)
        {
            var chunkPos = GetChunkPos(pos);

            if (!chunks.ContainsKey(chunkPos))
            {
                chunks[chunkPos] = new Chunk(CHUNKSIZE);
            }

            chunks[chunkPos].SetTile(pos - chunkPos, tile);

            TilePlaced?.Invoke(this, new TilePlacementEventArgs()
            {
                Chunk = chunks[chunkPos],
                GlobalPos = pos,
                ChunkPos = chunkPos,
                Tile = tile
            });

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

    public class TilePlacementEventArgs : EventArgs
    {
        public Chunk Chunk;

        public Tile Tile;

        public Vector2i GlobalPos;

        public Vector2i ChunkPos;
    }
}