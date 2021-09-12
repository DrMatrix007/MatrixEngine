using System;
using System.Collections.Generic;

namespace MatrixEngine.Animations {

    public sealed class AnimationGroup {
        private readonly Dictionary<string, Animation> animations;

        public Animation DefaultAnimation
        {
            private set;
            get;
        }

        public AnimationGroup(Dictionary<string, Animation> animations, Animation defaultAnimation) {
            this.animations = animations;
            this.DefaultAnimation = defaultAnimation;
        }

        public Animation GetAnimation(string s) {
            try {
                return animations[s];
            } catch (Exception) {
                return DefaultAnimation;
            }
        }

        public Animation this[string index]
        {
            get => GetAnimation(index);
        }
    }
}