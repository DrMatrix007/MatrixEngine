using System.Collections.Generic;
using System.Linq;

namespace MatrixEngine.Animations {

    public sealed class Animation {
        //private int _current = 0;

        //public int current
        //{
        //    get => _current;
        //    private set => _current = value;
        //}

        public readonly List<AnimationSequence> animationSequences;

        public AnimationSequence GetCurrent(float time) {
            time = time % maxTime;
            foreach (var item in animationSequences) {
                time -= item.durationTime;
                if (time <= 0) {
                    return item;
                }
            }
            return animationSequences.First();
        }

        public float maxTime
        {
            get => animationSequences.Select(e => e.durationTime).Sum();
        }

        public Animation(List<AnimationSequence> animationSequences) {
            this.animationSequences = animationSequences;
        }

        public Animation(params AnimationSequence[] animationSequences) {
            this.animationSequences = animationSequences.ToList();
        }
    }
}