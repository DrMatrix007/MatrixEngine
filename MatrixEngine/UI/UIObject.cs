using SFML.Graphics;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using SFML.System;
using SFML.Window;

namespace MatrixEngine.UI {



    public abstract class UIObject {

        public Drawable drawable;

        public UIStyle style;
        
        public int layer = 0;


        private Anchor _anchor;

        public Anchor anchor {
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

        public UIObject( Anchor anchor,UIStyle style, int layer, Action<UIObject, Vector2f> onHover, Action<UIObject, Vector2f,Mouse.Button> onClick) {
            this.style = style;
            this.layer = layer;
            _anchor = anchor;
            OnHover = onHover;
            OnClick = onClick;
        }

        protected void OnAnchorChange() {
            
        }

        public Action<UIObject,Vector2f> OnHover;
        public Action<UIObject,Vector2f,Mouse.Button> OnClick;



        public abstract (Vector2f pos,Vector2f size) Render(RenderTarget target);
    }
}