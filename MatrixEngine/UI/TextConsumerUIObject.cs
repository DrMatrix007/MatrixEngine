using System;
using MatrixEngine.StateManagment;
using SFML.Graphics;
using SFML.System;
using SFML.Window;

namespace MatrixEngine.UI {

    public class TextConsumerUIObject : TextUIObject {
        private Provider<string> provider;

        public TextConsumerUIObject(Anchor anchor, Provider<string> prov, UITextStyle uiTextStyle) : base(anchor, "", uiTextStyle) {
            provider = prov;
        }

        public override void OnHover(Vector2f pos) {
            // throw new NotImplementedException();
        }

        public override void OnClick(Vector2f pos, Mouse.Button button) {
            // throw new NotImplementedException();
        }

        public override (Vector2f pos, Vector2f size) Render(RenderTarget target) {
            if (provider.Get() != text) {
                text = provider.Get();
            }
            return base.Render(target);
        }
    }
}