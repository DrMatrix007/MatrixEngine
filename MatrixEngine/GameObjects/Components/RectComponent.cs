using MatrixEngine.Physics;



namespace MatrixEngine.GameObjects.Components {
    public class RectComponent : Component {

        private Rect _rect;

        public Rect rect
        {
            set { _rect = value; }
            get {
                _rect.x = position.X;
                _rect.y = position.Y;
                return _rect;
            }
        }
        public RectComponent() {
        }

        public override void Start() {
        }

        public override void Update() {
        }
    }
}
