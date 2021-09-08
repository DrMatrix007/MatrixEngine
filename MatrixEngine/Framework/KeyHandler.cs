using MatrixEngine.Utilities;
using SFML.Window;
using System;
using System.Collections.Generic;
using System.Linq;

namespace MatrixEngine.Framework {
    public sealed class KeyHandler {

        public enum KeyInput {
            Release,
            Press
        }

        private Dictionary<Keyboard.Key, bool> values;

        private List<Keyboard.Key> pressedDownKeys;
        public KeyHandler() {

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

        public bool isPressedDown(Keyboard.Key k) {
            return pressedDownKeys.Contains(k);
        }

        private void SetKey(Keyboard.Key key, bool b) {
            try {
                values[key] = b;


            } catch (Exception) { }
            pressedKeys = getCurrentPressedKeys();


        }
        internal void PressedKey(Keyboard.Key key) {
            SetKey(key, true);
            pressedDownKeys.Add(key);
        }
        internal void ReleasedKey(Keyboard.Key key) {
            SetKey(key, false);
        }

        public bool isPressed(Keyboard.Key key) {

            return values[key];

        }

        public Keyboard.Key[] getCurrentPressedKeys() {
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

        public Keyboard.Key[] pressedKeys
        {
            private set;
            get;

        } = new Keyboard.Key[] { };



    }
}