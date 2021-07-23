using MatrixEngine.GameObjects.Components.RenderComponents;
using System.Collections.Generic;
using System.Linq;

namespace MatrixEngine.App {
    public sealed class Renderer {

        App app;

        public Renderer(App app) {
            this.app = app;
            spriteRendererComponents = new List<RendererComponent>();
        }


        public List<RendererComponent> spriteRendererComponents;
        public void Render() {
            var list = spriteRendererComponents.OrderBy(e => e.layer);
            foreach (var item in list) {
                item.Render(app.window);
            }
            spriteRendererComponents.Clear();
        }
        public void addToDrawQueue(RendererComponent spriteRendererComponent) {
            spriteRendererComponents.Add(spriteRendererComponent);
        }
    }
}
