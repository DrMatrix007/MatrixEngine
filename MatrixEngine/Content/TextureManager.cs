using SFML.Graphics;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Content {
    public static class TextureManager {
        private static Dictionary<string, Texture> textures = new Dictionary<string, Texture>();

        public static Texture GetTexture(string path) {

            if (textures.ContainsKey(path)) {

                return textures[path];
            } else {
                textures[path] = new Texture(path);

                return textures[path];
            }
        }



        
    }
}
