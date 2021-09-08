using SFML.Graphics;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.GameObjects.Components.TilemapComponents {
    [Serializable]
    public class Tile {
        public Texture texture;
        public Color color;
        public Tile(Texture texture) : this(texture, Color.White) {

        }
        public Tile(Texture texture,Color color) {
            this.texture = texture;
            this.color = color;
        }


    }
}
