using SFML.Audio;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Content {
    public static class AudioManager {
        private static Dictionary<string, Sound> _sounds;

        static AudioManager() {
            _sounds = new Dictionary<string, Sound>();
        }

        public static Sound GetAudio(string path) {
            if (_sounds.ContainsKey(path)) {
                return _sounds[path];
            }

            _sounds[path] = new Sound(new SoundBuffer(path));
            //_textures[path].Smooth = true;

            return _sounds[path];
        }

    }
}
