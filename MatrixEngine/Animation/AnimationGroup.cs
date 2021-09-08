using System;
using System.Collections.Generic;

namespace MatrixEngine.Animations {
    public sealed class AnimationGroup {

        private readonly Dictionary<string, Animation> animations;
        private readonly Animation _defaultAnimation;

        public Animation defaultAnimation
        {
            private set;
            get;
        }

        public AnimationGroup(Dictionary<string, Animation> animations, Animation defaultAnimation) {
            this.animations = animations;
            this.defaultAnimation = defaultAnimation;
        }

        public Animation GetAnimation(string s) {
            try {
                return animations[s];
            } catch (Exception) {
                return defaultAnimation;
            }
        }
        public Animation this[string index]
        {
            get => GetAnimation(index);
        }
    }
}
