﻿using System;
using MatrixEngine.UI;
using SFML.Graphics;
using SFML.System;
using System.Collections.Generic;
using System.Linq;
using MatrixEngine.Physics;
using MatrixEngine.System;
using SFML.Window;

namespace MatrixEngine.Renderers {
    public class CanvasRenderer {

        RenderTexture target;


        App app;

        List<UIObject> list;

        public CanvasRenderer(App app) {
            this.app = app;
            list = new List<UIObject>();

            target = new RenderTexture((uint)app.windowSize.X, (uint)app.windowSize.Y);


        }

        public void Add(UIObject component) {
            list.Add(component);
        }

        public void Render() {
            if (target.Size != app.window.Size) {
                target.Dispose();
                target = new RenderTexture(app.window.Size.X, app.window.Size.Y);
            }
            target.Clear(Color.Transparent);
            ;

            var new_list = list.OrderBy(x => {
                return x.layer;
            });
            foreach (var component in new_list) {

                var (pos, size) = component.Render(target);
                var rect = new Rect(pos+size/2, size);
                var po = Mouse.GetPosition()-app.window.Position;
                if (!rect.IsInside((Vector2f)po)) continue;
                component.OnHover(component, (Vector2f)po);
                foreach (var value in Enum.GetValues<Mouse.Button>()) {
                    if (Mouse.IsButtonPressed(value)) {
                        component.OnClick(component,(Vector2f)po,value);
                    }
                }


            }
            target.Display();

            var tmp = app.window.GetView();
            var window_size = (Vector2f)app.window.Size;
            app.window.SetView(new View(new Vector2f(window_size.X / 2, window_size.Y / 2), (Vector2f)app.window.Size));
            var sp = new Sprite(target.Texture);



            app.window.Draw(sp);

            app.window.SetView(tmp);

            list.Clear();

        }


    }
}