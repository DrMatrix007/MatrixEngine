using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.Animation {
    public sealed class Animation {

        private int _current = 0;

        public int current
        {
            get => _current;
            private set => _current = value;
        }
        
        public Dictionary<string,AnimationSequence> animationSequences;

        public Animation(Dictionary<string, AnimationSequence> animationSequences) {
            this.animationSequences = animationSequences;
        }
    }
}
