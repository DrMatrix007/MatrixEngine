using MatrixEngine.GameObjects.Components.UIComponents;
using MatrixEngine.System;
using SFML.Graphics;
using SFML.System;
using System.Collections.Generic;
using System.Linq;

namespace MatrixEngine.Renderers {
    public class CanvasRenderer {

        RenderTexture target;


        App app;

        List<UIRendererComponent> list;

        public CanvasRenderer(App app) {
            this.app = app;
            list = new List<UIRendererComponent>();

            target = new RenderTexture((uint)app.windowSize.X, (uint)app.windowSize.Y);


        }

        public void Add(UIRendererComponent component) {
            list.Add(component);
        }

        public void Render() {
            if (target.Size != app.window.Size) {
                target = new RenderTexture(app.window.Size.X, app.window.Size.Y);
            }
            target.Clear(Color.Transparent);
            ;

            var new_list = list.OrderBy(x => {
                return x.layer;
            });
            foreach (var component in new_list) {

                component.Render(target);


            }
            target.Display();

            var tmp = app.window.GetView();
            var window_size = app.window.Size;
            app.window.SetView(new View(new Vector2f(+window_size.X / 2, +window_size.Y / 2), (Vector2f)app.window.Size));
            /* draw your stuff */
            var sp = new Sprite(target.Texture);

            //sp.Position =
            //-(Vector2f)app.window.Size / 2;

            Debug.Log(app.window.GetView().Size);


            app.window.Draw(sp);

            app.window.SetView(tmp);

        }


    }
}
