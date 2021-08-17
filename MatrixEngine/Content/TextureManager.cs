using System.Collections.Generic;
using SFML.Graphics;

namespace MatrixEngine.Content {
    public static class TextureManager {
        private static Dictionary<string, Texture> _textures;

        static TextureManager() {
            _textures = new Dictionary<string, Texture>();
        }

        public static Texture GetTexture(string path) {
            if (_textures.ContainsKey(path)) {
                return _textures[path];
            }

            _textures[path] = new Texture(path);

            return _textures[path];
        }
    }
}