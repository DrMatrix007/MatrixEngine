using MatrixGDK.GameObjects.Components.RenderComponents;
using MatrixGDK.Physics;
using MatrixGDK.System;
using System.Collections.Generic;
using System.Linq;

namespace MatrixGDK.Renderers {
    public sealed class Renderer {

        App app;

        public Renderer(App app) {
            this.app = app;
            spriteRendererComponents = new List<RendererComponent>();
        }


        public List<RendererComponent> spriteRendererComponents;
        public void Render() {

            var list = spriteRendererComponents.OrderBy(e => e.layer);
            var rend_list = new List<RendererComponent>();
            var cam_rect = app.camera.rect;

            Utils.GetTimeInSeconds(() => {

                foreach (var item in list) {
                    item.Render(app.window);

                    
                }
            });

            //foreach (var item in rend_list) {
            //    item.Render(app.window);
            //}

            spriteRendererComponents.Clear();

        }
        public void AddToDrawQueue(RendererComponent spriteRendererComponent) {
            spriteRendererComponents.Add(spriteRendererComponent);
        }
    }
}
