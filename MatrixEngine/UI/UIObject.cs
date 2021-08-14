using SFML.Graphics;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using SFML.System;

namespace MatrixEngine.UI {
    public enum FromVertical {
        Bottom,
        Up
    }

    public enum FromHorizontal {
        Left,
        Right,
    }

    public abstract class UIObject {
        public int layer = 0;


        private Anchor _anchor;

        public Anchor anchor {
            get => _anchor;
            set {
                _anchor = value;
                OnAnchorChange();
            }
        }
        protected UIObject(Anchor anchor) {
            this._anchor = anchor;
        }

        protected void OnAnchorChange() {
            
        }

        public abstract void OnHover(Vector2f hoverPos);

        public abstract void OnClick(Vector2f clickPos);


        public abstract void Render(RenderTarget target);
    }
}