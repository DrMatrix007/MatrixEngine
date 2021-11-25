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

        public Tile GetTileFromLocalPosition(Vector2i offset)
        {
            if ((!offset.X.IsInRangeIncludes(0, CHUNKSIZE-1)) || (!offset.Y.IsInRangeIncludes(0, CHUNKSIZE-1)))
            {
                throw new ArgumentOutOfRangeException(nameof(offset));
            }

            return tiles.GetValueOrDefault(offset);
        }
    }

    public class TilemapBehavior : Behavior
    {
        public readonly ushort CHUNK_SIZE = 50;

        internal Dictionary<Vector2i, Chunk> chunks = new Dictionary<Vector2i, Chunk>();

        public EventHandler<TilePlacementEventArgs> TilePlaced;

        public IEnumerable<KeyValuePair<Vector2i, Tile>> tiles => from chunk in chunks
            from valueTile in chunk.Value.tiles
            select new KeyValuePair<Vector2i, Tile>(chunk.Key + valueTile.Key, valueTile.Value);
        //foreach (var chunk in chunks)
        // {
        //     foreach (var valueTile in chunk.Value.tiles)
        //     {
        //         yield return new KeyValuePair<Vector2i, Tile>(chunk.Key + valueTile.Key, valueTile.Value);
        //     }
        // }

        public Tile SetTile(Vector2i pos, Tile tile)
        {
            var chunkPos = GetChunkPos(pos);

            if (!chunks.ContainsKey(chunkPos))
            {
                chunks[chunkPos] = new Chunk(CHUNK_SIZE);
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

        private Vector2i GetLocalChunkPos(Vector2i i, Vector2i chunk_pos)
        {
            var pos = i - chunk_pos;
            if (pos.X < 0)
            {
                pos.X = CHUNK_SIZE + pos.X;
            }

            if (pos.Y < 0)
            {
                pos.Y = CHUNK_SIZE + pos.Y;
            }

            return pos;
        }

        public Tile GetTileFromTilemapPos(Vector2i i)
        {
            var chunk_vec = new Vector2i((int)MathF.Floor((float)(i.X) / CHUNK_SIZE),
                (int)MathF.Floor((float)(i.Y) / CHUNK_SIZE)) * CHUNK_SIZE;
            if (chunks.ContainsKey(chunk_vec))
            {
                return chunks[chunk_vec].GetTileFromLocalPosition(GetLocalChunkPos(i, chunk_vec));
            }

            return default;
        }

        public Vector2i GetPosOfTileFromWorldPos(Vector2f pos)
        {
            return (Vector2i)(new Vector2f(pos.X / Transform.Scale.X, pos.Y / Transform.Scale.Y) -
                              Transform.Position);
        }

        public Tile GetTileFromWorldPos(Vector2f pos)
        {
            return GetTileFromTilemapPos(GetPosOfTileFromWorldPos(pos));
        }

        public Vector2f GetWorldPosFromTilePos(Vector2i pos)
        {
            return Transform.Position + ((Vector2f)pos).Multiply(Transform.Scale);
        }

        public Vector2i GetChunkPos(Vector2i vector2)
        {
            return ((Vector2f)vector2 / CHUNK_SIZE).FloorToInt() * CHUNK_SIZE;
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