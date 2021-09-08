using MatrixEngine.Animations;
using SFML.Graphics;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.GameObjects.Components.RenderComponents {

    public class SpriteAnimationRendererComponent : RendererComponent {
        private Sprite sprite;
        public readonly int pixelperunit;
        private AnimationGroup animationGroup;

        private Animation currentAnim;

        public void SetAnimation(string s) {
            currentAnim = animationGroup[s];
        }

        public SpriteAnimationRendererComponent(AnimationGroup animationGroup, int pixelperunit) {
            this.animationGroup = animationGroup;
            sprite = new Sprite();
            this.pixelperunit = pixelperunit;
            currentAnim = animationGroup.defaultAnimation;
        }

        public override void Update() {
            base.Update();
            sprite.Position = position;
            sprite.Scale = transform.scale / pixelperunit;

            sprite.Texture = currentAnim.GetCurrent(app.time).texture;
        }

        public override void Render(RenderTarget target) {
            target.Draw(sprite);
        }
    }
}