using SFML.Graphics;
using SFML.System;
using System.Collections;
using System.Collections.Generic;

namespace MatrixEngine.GameObjects.Components.TilemapComponents {
    public class Chunk : IEnumerable<KeyValuePair<Vector2i,Tile>>{
        public int chunkSize = 16;

        public bool isRenderedUpdated = true;

        public Vector2i fullPosition;

        public Vector2f size
        {
            get => new Vector2f(chunkSize, chunkSize);
        }

        public Tile[,] tiles;

        public Chunk(Vector2i offset,int chunkSize) {
            this.chunkSize = chunkSize;
            fullPosition = offset;
            tiles = new Tile[chunkSize, chunkSize];
        }



        public Tile GetTileFromLocalPosition(Vector2i offset) {
            
            return tiles[offset.X, offset.Y];
        }
        public void SetTileFromLocalPos(Vector2i offset,Tile t) {
            tiles[offset.X, offset.Y] = t;
        }
        public IEnumerator<KeyValuePair<Vector2i, Tile>> GetEnumerator() {
            for (int i = 0; i < chunkSize; i++) {
                for (int j = 0; j < chunkSize; j++) {

                    var v = new Vector2i(i, j);
                    var t = GetTileFromLocalPosition(v);

                    if (t != null) {
                        yield return new KeyValuePair<Vector2i, Tile>(v, t);
                    }   
                }
            }
        }
        IEnumerator IEnumerable.GetEnumerator() {
            return GetEnumerator();
        }
    }
}
