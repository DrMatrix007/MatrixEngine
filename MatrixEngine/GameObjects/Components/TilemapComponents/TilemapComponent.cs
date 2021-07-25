using SFML.System;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.GameObjects.Components.TilemapComponents {
    public class TilemapComponent : Component {
        public readonly int pixelsPerUnit;
        internal Dictionary<Vector2i, Tile> tiles;

        public TilemapComponent() : this(16) {

        }

        public TilemapComponent(int pixelperunit) {
            this.pixelsPerUnit = pixelperunit;

            tiles = new Dictionary<Vector2i, Tile>();
        }
        public void SetTile(Vector2i i, Tile tile) {
            if (tile != null) {
                tiles[i] = tile;
            } else {
                tiles.Remove(i);
            }

        }

        public override void Start() {
        }

        public override void Update() {
        }
    }
}
