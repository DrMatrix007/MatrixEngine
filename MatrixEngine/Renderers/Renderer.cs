using MatrixEngine.GameObjects.Components.RenderComponents;
using MatrixEngine.System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using Debug = MatrixEngine.System.Debug;
namespace MatrixEngine.Renderers {
    public sealed class Renderer {

        App app;

        public Renderer(App app) {
            this.app = app;
            spriteRendererComponents = new List<RendererComponent>();
        }


        public List<RendererComponent> spriteRendererComponents;
        public void Render() {

            var watch = new Stopwatch();
            watch.Start();
            var list = spriteRendererComponents.OrderBy(e => e.layer);
            foreach (var item in list) {
                item.Render(app.window);
            }
            spriteRendererComponents.Clear();
            watch.Stop();
            Debug.Log("rend: " + watch.Elapsed.TotalSeconds.ToString());

        }
        public void addToDrawQueue(RendererComponent spriteRendererComponent) {
            spriteRendererComponents.Add(spriteRendererComponent);
        }
    }
}
