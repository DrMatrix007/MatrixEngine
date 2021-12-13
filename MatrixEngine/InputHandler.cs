using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using SFML.Window;

namespace MatrixEngine
{
    public class InputHandler
    {
        private Dictionary<Keyboard.Key, bool> keys = new Dictionary<Keyboard.Key, bool>();
        private Dictionary<Mouse.Button,bool> mouseButtons = new Dictionary<Mouse.Button,bool>();
        private Dictionary<Mouse.Button,bool> mouseButtonsDown = new Dictionary<Mouse.Button,bool>();

        public float ScrollDelta { get; private set; }
        public float ScrollY { get; private set; }

        public IEnumerable<Keyboard.Key> GetAllPressedKeys()
        {
            return keys.Where(pair => pair.Value == true).Select(e => e.Key);
        }

        public bool IsPressed(Keyboard.Key key)
        {
            return keys.GetValueOrDefault(key) && keys[key];
        }

        public bool IsPressed(IEnumerable<Keyboard.Key> keys)
        {
            return keys.Any(IsPressed);
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

        internal void Window_MouseWheelScrolled(object sender, MouseWheelScrollEventArgs e)
        {
            ScrollDelta = e.Delta;

            ScrollY += ScrollDelta;
        }

        internal void Window_MouseButtonPressed(object sender, MouseButtonEventArgs e)
        {
            mouseButtons[e.Button] = true;
            mouseButtonsDown[e.Button] = true;


        }

        internal void Window_MouseButtonReleased(object sender, MouseButtonEventArgs e)
        {
            mouseButtons[e.Button] = false;
            mouseButtonsDown[e.Button] = false;
        }

        public void Update()
        {
            ScrollDelta = 0;
            mouseButtonsDown.Clear();
        }



        public IEnumerable<Mouse.Button> GetAllPressedMouseButtons()
        {
            return mouseButtons.Where(pair => pair.Value == true).Select(e => e.Key);
        }

        public bool IsMouseButtonPressed(Mouse.Button key)
        {
            return mouseButtons.GetValueOrDefault(key) && mouseButtons[key];
        }

        public bool IsMouseButtonPressed(IEnumerable<Mouse.Button> keys)
        {
            return keys.Any(IsMouseButtonPressed);
        }

        public bool IsAllMouseButtonPressed(IEnumerable<Mouse.Button> keys)
        {
            return keys.All(IsMouseButtonPressed);
        }

        public IEnumerable<Mouse.Button> GetAllPressedDownMouseButtons()
        {
            return mouseButtonsDown.Where(pair => pair.Value == true).Select(e => e.Key);
        }

        public bool IsMouseButtonPressedDown(Mouse.Button key)
        {
            return mouseButtonsDown.GetValueOrDefault(key) && mouseButtonsDown[key];
        }

        public bool IsMouseButtonPressedDown(IEnumerable<Mouse.Button> keys)
        {
            return keys.Any(IsMouseButtonPressedDown);
        }

        public bool IsAllMouseButtonPressedDown(IEnumerable<Mouse.Button> keys)
        {
            return keys.All(IsMouseButtonPressedDown);
        }



    }
}