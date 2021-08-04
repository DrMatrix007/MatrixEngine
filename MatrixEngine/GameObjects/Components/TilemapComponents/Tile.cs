using SFML.Graphics;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixGDK.GameObjects.Components.TilemapComponents {
    [Serializable]
    public class Tile {
        public Texture texture;

        public Tile(Texture texture) {
            this.texture = texture;

        }

    }
}
