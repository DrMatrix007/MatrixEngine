using SFML.System;

namespace MatrixEngine.UI {
    public struct Anchor {
        public Anchor(Vector2f positionInPercentage, Vector2f maxSizeInPercentage) {
            this.maxSizeInPercentage = maxSizeInPercentage;
            this.positionInPercentage = positionInPercentage;
        }

        public Vector2f maxSizeInPercentage;

        public Vector2f positionInPercentage;
    }
}