using System;
using MatrixEngine.UI;
using SFML.Graphics;
using SFML.System;
using System.Collections.Generic;
using System.Linq;
using MatrixEngine.Physics;
using MatrixEngine.Framework;
using SFML.Window;

namespace MatrixEngine.Renderers {

    public class CanvasRenderer : Renderer {
        private RenderTexture target;
        private readonly List<UIObject> list;

        public CanvasRenderer(App app) : base(app) {
            list = new List<UIObject>();

            target = new RenderTexture((uint)app.WindowSize.X, (uint)app.WindowSize.Y);
        }

        public void Add(UIObject component) {
            list.Add(component);
        }

        public override void Render() {
            if (target.Size != app.Window.Size) {
                target.Texture.Dispose();
                target.Dispose();

                target = new RenderTexture(app.Window.Size.X, app.Window.Size.Y);
            }
            target.Clear(Color.Transparent);
            ;

            var new_list = list.OrderBy(x => {
                return x.layer;
            });
            foreach (var component in new_list) {
                var (pos, size) = component.Render(target);

                var rect = new Rect(pos + size / 2, size);
                var po = Mouse.GetPosition() - app.Window.Position;
                if (!rect.IsInside((Vector2f)po))
                    continue;
                component.OnHover((Vector2f)po);
                foreach (var value in Enum.GetValues<Mouse.Button>()) {
                    if (Mouse.IsButtonPressed(value)) {
                        component.OnClick((Vector2f)po, value);
                    }
                }
            }
            target.Display();

            var tmp = app.Window.GetView();
            var window_size = (Vector2f)app.Window.Size;
            app.Window.SetView(new View(new Vector2f(window_size.X / 2, window_size.Y / 2), (Vector2f)app.Window.Size));
            var sp = new Sprite(target.Texture);

            app.Window.Draw(sp);

            //sp.Texture.Dispose();
            sp.Dispose();

            app.Window.SetView(tmp);
            //tmp.Dispose();

            list.Clear();
        }
    }
}