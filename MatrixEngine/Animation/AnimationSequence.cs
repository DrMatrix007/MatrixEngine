using SFML.Graphics;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Animation {
    public sealed class AnimationSequence {

        public Texture texture;

        public float durationTime;

        public AnimationSequence(Texture texture, float durationTime) {
            this.texture = texture;
            this.durationTime = durationTime;
        }
    }
}
