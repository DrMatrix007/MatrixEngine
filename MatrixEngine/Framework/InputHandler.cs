using MatrixEngine.Utilities;
using SFML.System;
using SFML.Window;
using System;
using System.Collections.Generic;
using System.Linq;
using MatrixEngine.Framework;

namespace MatrixEngine.Framework {

    public sealed class InputHandler {
        public readonly App App;

        public enum KeyInput {
            Release,
            Press
        }

        private readonly Dictionary<Keyboard.Key, bool> values;

        private readonly List<Keyboard.Key> pressedDownKeys;

        public InputHandler(App app) {
            App = app;
            values = new Dictionary<Keyboard.Key, bool>();
            pressedDownKeys = new List<Keyboard.Key>();
            foreach (Keyboard.Key key in Enum.GetValues<Keyboard.Key>()) {
                try {
                    values[key] = false;
                } catch (Exception e) {
                    Utils.LogError(e.ToString());
                }
            }
        }

        internal void Update() {
            pressedDownKeys.Clear();
        }

        public bool IsPressedDown(Keyboard.Key k) {
            return pressedDownKeys.Contains(k);
        }

        public Vector2i GetMouseScreenPos() {
            return Mouse.GetPosition(App.Window);
        }

        public Vector2f GetMouseWorldPos() {
            var mousepos = (Vector2f)(GetMouseScreenPos() - (Vector2i)App.Window.Size / 2);

            mousepos = mousepos.Multiply((Vector2f)App.CameraSize).Devide((Vector2f)App.WindowSize);

            return mousepos + App.Camera.position;
        }

        private void SetKey(Keyboard.Key key, bool b) {
            try {
                values[key] = b;
            } catch (Exception) { }
            PressedKeys = GetCurrentPressedKeys();
        }

        internal void PressedKey(Keyboard.Key key) {
            SetKey(key, true);
            pressedDownKeys.Add(key);
        }

        internal void ReleasedKey(Keyboard.Key key) {
            SetKey(key, false);
        }

        public bool IsPressed(Keyboard.Key key) {
            return values[key];
        }

        public bool IsPressed(Mouse.Button button) {
            return Mouse.IsButtonPressed(button);
        }

        public bool IsPressed(params Mouse.Button[] keys) {
            foreach (var item in keys) {
                if (IsPressed(item)) {
                    return true;
                }
            }
            return false;

        }


        public bool IsPressed(params Keyboard.Key[] keys) {
            foreach (var item in keys) {
                if (IsPressed(item)) {
                    return true;
                }
            }
            return false;
        }

        public Keyboard.Key[] GetCurrentPressedKeys() {
            return values.Where(
                    (e) => {
                        return e.Value;
                    }
                ).Select(
                    (e) => {
                        return e.Key;
                    }

                    ).ToArray();
        }

        public Keyboard.Key[] PressedKeys
        {
            private set;
            get;
        } = Array.Empty<Keyboard.Key>();
    }
}