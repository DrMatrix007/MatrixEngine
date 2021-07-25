using MatrixEngine.GameObjects.Components.RenderComponents;
using SFML.Graphics;
using SFML.System;
using System.Collections.Generic;
using System.Linq;

namespace MatrixEngine.GameObjects.Components.TilemapComponents {
    [RequireComponent(typeof(TilemapComponent))]
    public class TilemapRendererComponent : RendererComponent {

        private TilemapComponent tilemap;

        private RenderTexture renderTexture;

        public TilemapRendererComponent() {
            this.layer = -50;
            renderTexture = new RenderTexture(1, 1);
        }

        private int minx = int.MaxValue;
        private int miny = int.MaxValue;
        private int maxy = int.MinValue;
        private int maxx = int.MinValue;

        public override void Start() {

            base.Start();

            tilemap = GetComponent<TilemapComponent>();

        }
        public override void Update() {
            base.Update();
        }
        private List<Sprite> spritel = new List<Sprite>();
        private List<KeyValuePair<Vector2i, Tile>> checkedlist;
        public override void Render(RenderTarget target) {
            var a = new Vector2f(0.5f, 0.5f);
            var r = app.camera.rect;
            var s = new Vector2f(1, 1) / tilemap.pixelsPerUnit;
            checkedlist = tilemap.tiles.Where((e) => r.IsInside((Vector2f)e.Key + a)).ToList();
            foreach (var item in checkedlist) {
                var sprite = new Sprite();

                sprite.Texture = item.Value.texture;
                sprite.Scale = s;
                sprite.Position = (Vector2f)item.Key;
                spritel.Add(sprite);
            }
            foreach (var item in spritel) {
                target.Draw(item);
            }
            spritel.Clear();
            checkedlist.Clear();
        }
    }
}
