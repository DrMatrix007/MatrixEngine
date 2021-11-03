using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using SFML.Window;

namespace MatrixEngine.ECS
{
    public class KeyHandler
    {
        private Dictionary<Keyboard.Key, bool> keys = new Dictionary<Keyboard.Key, bool>();

        public IEnumerable<Keyboard.Key> GetAllPressedKeys()
        {
            return keys.Where((pair => pair.Value == true)).Select(e => e.Key);
        }

        public bool IsPressed(Keyboard.Key key)
        {
            return keys.GetValueOrDefault(key) && keys[key];
        }

        public bool IsPressed(IEnumerable<Keyboard.Key> keys)
        {
            return keys.Any(key => IsPressed(key));
        }

        public bool IsAllPressed(IEnumerable<Keyboard.Key> keys)
        {
            return keys.All(IsPressed);
        }

        internal void WindowKeyPressed(object sender, KeyEventArgs e)
        {
            keys[e.Code] = true;
        }

        internal void WindowKeyReleased(object sender, KeyEventArgs e)
        {
            keys[e.Code] = false;
        }
    }
}