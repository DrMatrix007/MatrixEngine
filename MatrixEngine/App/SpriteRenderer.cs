using MatrixEngine.GameObjects.Components;
using SFML.Graphics;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.App {
    public sealed class SpriteRenderer {
        public List<SpriteRendererComponent> spriteRendererComponents = new List<SpriteRendererComponent>();
        public void Render() {
            var list = spriteRendererComponents.OrderBy(e => e.layer);
            foreach (var item in list) {
                item.Draw();
            }
            spriteRendererComponents.Clear();
        }
        public void addToDrawQueue(SpriteRendererComponent spriteRendererComponent) {
            spriteRendererComponents.Add(spriteRendererComponent);
        }
    }
}
