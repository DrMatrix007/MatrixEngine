using System;
using MatrixEngine.StateManagment;
using SFML.Graphics;

namespace MatrixEngine.UI {
    public class TextRendererConsumerUIObject : TextRendererUIObject {
        private Provider<string> provider;
        
        public TextRendererConsumerUIObject(Anchor anchor, Provider<string> prov, Font font, bool auto_size = true, uint charsize = 10) : base(anchor, "" , font, auto_size, charsize) {
            provider = prov;
        }

        public TextRendererConsumerUIObject(Anchor anchor, Provider<String> prov, bool isAutoSize = true, uint charsize = 10) : base(anchor, "", isAutoSize, charsize) {
            provider = prov;
        }

        public override void Render(RenderTarget target) {
            text = provider.Get();
            base.Render(target);
        }
    }
}