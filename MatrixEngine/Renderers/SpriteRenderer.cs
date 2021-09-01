﻿using MatrixEngine.Physics;
using System.Collections.Generic;
using System.Linq;
using MatrixEngine.GameObjects.Components.RenderComponents;
using MatrixEngine.Framework;

namespace MatrixEngine.Renderers {
    public sealed class SpriteRenderer : Renderer {

        App app;

        public SpriteRenderer(App app) : base(app){
            this.app = app;
            spriteRendererComponents = new List<RendererComponent>();
        }


        public List<RendererComponent> spriteRendererComponents;
        public override void Render() {

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
        public void AddToQueue(RendererComponent spriteRendererComponent) {
            spriteRendererComponents.Add(spriteRendererComponent);
        }
    }
}