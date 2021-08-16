using System;
using MatrixEngine.StateManagment;
using SFML.Graphics;
using SFML.System;
using SFML.Window;

namespace MatrixEngine.UI {
    public class TextRendererConsumerUIObject : TextRendererUIObject {
        private Provider<string> provider;
        
        public TextRendererConsumerUIObject(Anchor anchor, Provider<string> prov, UITextStyle uiTextStyle,int layer,Action<UIObject,Vector2f,Mouse.Button> onClick,Action<UIObject,Vector2f> onHover) : base(anchor, "" , uiTextStyle,layer,onClick,onHover) {
            provider = prov;
        }
        public override  (Vector2f pos, Vector2f size) Render(RenderTarget target) {
            text = provider.Get();
            return base.Render(target);
        }
    }
}