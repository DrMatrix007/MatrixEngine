using SFML.Graphics;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.Framework;
using SFML.System;
using SFML.Window;

namespace MatrixEngine.UI {

    public abstract class UIObject {

        public Scene scene
        {
            get;
            private set;
        }

        public Drawable drawable;

        public UIStyle style;

        private Anchor _anchor;

        public Anchor anchor
        {
            get => _anchor;
            set {
                _anchor = value;
                OnAnchorChange();
            }
        }

        // protected UIObject(Anchor anchor,UIStyle uiStyle) {
        //     this._anchor = anchor;
        //     style = uiStyle;
        // }

        public UIObject(Anchor anchor, UIStyle style) {
            this.style = style;
            _anchor = anchor;
        }

        protected void OnAnchorChange() {
        }

        public abstract void OnHover(Vector2f pos);

        public abstract void OnClick(Vector2f pos, Mouse.Button button);

        public abstract (Vector2f pos, Vector2f size) Render(RenderTarget target);

        public void SetupScene(Scene scene) {
            this.scene = scene;
        }
    }
}